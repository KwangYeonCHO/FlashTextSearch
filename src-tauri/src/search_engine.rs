use std::path::Path;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::Instant;
use ignore::WalkBuilder;
use rayon::prelude::*;
use tauri::{AppHandle, Emitter};

use crate::excel_search::ExcelSearcher;
use crate::fast_text_search::{CompiledMatcher, FastTextSearcher};
use crate::types::{FileMatchResult, SearchProgress, SearchQuery};

/// 搜索引擎全局状态管理（用于控制取消与并发）
pub struct SearchManager {
    pub cancel_flag: Arc<AtomicBool>,
}

impl SearchManager {
    pub fn new() -> Self {
        Self {
            cancel_flag: Arc::new(AtomicBool::new(false)),
        }
    }

    /// 触发中止当前搜索
    pub fn cancel(&self) {
        self.cancel_flag.store(true, Ordering::SeqCst);
    }

    /// 重置取消标记
    pub fn reset_cancel(&self) -> Arc<AtomicBool> {
        self.cancel_flag.store(false, Ordering::SeqCst);
        self.cancel_flag.clone()
    }
}

/// 搜索引擎核心执行器
pub struct SearchEngine;

impl SearchEngine {
    /// 执行后台并行搜索任务
    pub fn start_search(
        app_handle: AppHandle,
        query: SearchQuery,
        cancel_token: Arc<AtomicBool>,
    ) -> Result<(), String> {
        let root = Path::new(&query.root_path);
        if !root.exists() || !root.is_dir() {
            return Err("指定的搜索根目录不存在或不是文件夹".to_string());
        }

        // 预编译关键词匹配器 (SIMD Aho-Corasick / Regex)
        let matcher = Arc::new(CompiledMatcher::build(&query)?);

        // 格式化扩展名过滤列表（统一转为小写、去除点）
        let target_extensions: Vec<String> = query
            .extensions
            .iter()
            .map(|ext| ext.trim().trim_start_matches('.').to_lowercase())
            .filter(|ext| !ext.is_empty() && ext != "*")
            .collect();
        let target_extensions = Arc::new(target_extensions);

        // 启动异步独立线程执行密集型扫描
        std::thread::spawn(move || {
            let start_time = Instant::now();

            let files_scanned = Arc::new(AtomicUsize::new(0));
            let files_matched = Arc::new(AtomicUsize::new(0));
            let total_matches = Arc::new(AtomicUsize::new(0));

            // 构建 ignore 目录遍历器（支持忽略 .git、支持限制子目录层级）
            let mut walk_builder = WalkBuilder::new(&query.root_path);
            walk_builder.hidden(query.ignore_hidden);
            walk_builder.git_ignore(query.ignore_hidden);
            walk_builder.git_global(query.ignore_hidden);
            walk_builder.git_exclude(query.ignore_hidden);

            if !query.include_subdirectories {
                walk_builder.max_depth(Some(1));
            }

            // 收集所有候选文件路径
            let mut file_entries = Vec::new();
            for result in walk_builder.build() {
                if cancel_token.load(Ordering::Relaxed) {
                    break;
                }
                if let Ok(entry) = result {
                    if entry.file_type().map_or(false, |ft| ft.is_file()) {
                        let path = entry.path().to_path_buf();

                        // 扩展名过滤
                        let ext = path
                            .extension()
                            .map(|e| e.to_string_lossy().to_lowercase())
                            .unwrap_or_default();

                        let is_allowed = if target_extensions.is_empty() {
                            // 若用户未指定特定后缀，默认允许常见文本、代码及 Excel
                            Self::is_common_searchable_extension(&ext)
                        } else {
                            target_extensions.contains(&ext)
                        };

                        if is_allowed {
                            file_entries.push(path);
                        }
                    }
                }
            }

            let total_files = file_entries.len();

            // 发送初始发现文件进度事件
            let initial_progress = SearchProgress {
                total_files,
                files_scanned: 0,
                progress_percent: 0.0,
                files_matched: 0,
                total_matches: 0,
                elapsed_ms: start_time.elapsed().as_millis() as u64,
                is_finished: false,
                is_cancelled: false,
                current_file: None,
            };
            let _ = app_handle.emit("search-progress", initial_progress);

            // 使用 Rayon 并行处理所有文件匹配，并分批流式发送给前端
            let chunk_size = 64;
            let max_file_size_bytes = query.max_file_size_mb.unwrap_or(200) * 1024 * 1024;

            let mut batch_buffer: Vec<FileMatchResult> = Vec::new();
            let mut last_emit_time = Instant::now();

            for chunk in file_entries.chunks(chunk_size) {
                if cancel_token.load(Ordering::Relaxed) {
                    break;
                }

                let current_chunk_first_file = chunk
                    .first()
                    .and_then(|p| p.file_name())
                    .map(|s| s.to_string_lossy().to_string());

                // 块内并行处理
                let chunk_results: Vec<FileMatchResult> = chunk
                    .par_iter()
                    .filter_map(|path| {
                        if cancel_token.load(Ordering::Relaxed) {
                            return None;
                        }

                        files_scanned.fetch_add(1, Ordering::Relaxed);

                        let metadata = std::fs::metadata(path).ok()?;
                        let file_size = metadata.len();
                        if file_size > max_file_size_bytes {
                            return None; // 超过设定最大体积限制
                        }

                        let modified_time = metadata
                            .modified()
                            .ok()
                            .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
                            .map(|d| d.as_millis() as u64)
                            .unwrap_or(0);

                        let ext = path
                            .extension()
                            .map(|e| e.to_string_lossy().to_lowercase())
                            .unwrap_or_default();

                        // 路由分流：Excel 文档 vs 纯文本/代码快速路径
                        if ext == "xlsx" || ext == "xls" || ext == "ods" || ext == "xlsb" {
                            ExcelSearcher::search_file(path, file_size, modified_time, &matcher)
                        } else {
                            FastTextSearcher::search_file(path, file_size, modified_time, &matcher)
                        }
                    })
                    .collect();

                // 统计与收集
                for res in chunk_results {
                    files_matched.fetch_add(1, Ordering::Relaxed);
                    total_matches.fetch_add(res.matches.len(), Ordering::Relaxed);
                    batch_buffer.push(res);
                }

                let scanned = files_scanned.load(Ordering::Relaxed);
                let percent = if total_files > 0 {
                    ((scanned as f64 / total_files as f64) * 100.0).min(100.0)
                } else {
                    100.0
                };

                // 限制发送频率（每收集 50 条或每隔 40ms 发送一次 batch），防止高频 IPC 卡顿
                if batch_buffer.len() >= 50 || (last_emit_time.elapsed().as_millis() >= 40 && !batch_buffer.is_empty()) || scanned == total_files {
                    let to_send = std::mem::take(&mut batch_buffer);
                    if !to_send.is_empty() {
                        let _ = app_handle.emit("search-result-batch", to_send);
                    }
                    last_emit_time = Instant::now();

                    // 发送实时进度更新（包含百分比与当前文件名）
                    let progress = SearchProgress {
                        total_files,
                        files_scanned: scanned,
                        progress_percent: (percent * 10.0).round() / 10.0,
                        files_matched: files_matched.load(Ordering::Relaxed),
                        total_matches: total_matches.load(Ordering::Relaxed),
                        elapsed_ms: start_time.elapsed().as_millis() as u64,
                        is_finished: false,
                        is_cancelled: false,
                        current_file: current_chunk_first_file,
                    };
                    let _ = app_handle.emit("search-progress", progress);
                }
            }

            // 发送剩余未发送的结果
            if !batch_buffer.is_empty() {
                let _ = app_handle.emit("search-result-batch", batch_buffer);
            }

            // 发送最终完成事件
            let is_cancelled = cancel_token.load(Ordering::Relaxed);
            let final_scanned = files_scanned.load(Ordering::Relaxed);
            let final_percent = if is_cancelled {
                if total_files > 0 { ((final_scanned as f64 / total_files as f64) * 100.0).min(100.0) } else { 0.0 }
            } else {
                100.0
            };

            let final_progress = SearchProgress {
                total_files,
                files_scanned: final_scanned,
                progress_percent: (final_percent * 10.0).round() / 10.0,
                files_matched: files_matched.load(Ordering::Relaxed),
                total_matches: total_matches.load(Ordering::Relaxed),
                elapsed_ms: start_time.elapsed().as_millis() as u64,
                is_finished: true,
                is_cancelled,
                current_file: None,
            };
            let _ = app_handle.emit("search-finished", final_progress);
        });

        Ok(())
    }

    /// 常见文本与可搜文档默认白名单
    fn is_common_searchable_extension(ext: &str) -> bool {
        matches!(
            ext,
            "txt"
                | "log"
                | "md"
                | "markdown"
                | "json"
                | "xml"
                | "yaml"
                | "yml"
                | "toml"
                | "ini"
                | "cfg"
                | "conf"
                | "csv"
                | "tsv"
                | "rs"
                | "c"
                | "cpp"
                | "h"
                | "hpp"
                | "cs"
                | "java"
                | "py"
                | "js"
                | "ts"
                | "jsx"
                | "tsx"
                | "vue"
                | "html"
                | "htm"
                | "css"
                | "scss"
                | "less"
                | "sql"
                | "sh"
                | "bat"
                | "ps1"
                | "cmd"
                | "go"
                | "php"
                | "rb"
                | "swift"
                | "kt"
                | "dart"
                | "lua"
                | "r"
                | "xlsx"
                | "xls"
                | "ods"
        )
    }
}
