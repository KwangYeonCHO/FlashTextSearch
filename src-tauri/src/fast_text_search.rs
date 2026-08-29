use std::fs::File;
use std::path::Path;
use aho_corasick::AhoCorasick;
use memmap2::Mmap;
use regex::Regex;

use crate::encoding::EncodingHelper;
use crate::types::{FileMatchResult, MatchItem, SearchQuery};

/// 编译后的匹配执行器枚举
pub enum CompiledMatcher {
    /// 基于 Aho-Corasick 的极速单词/多词多字节向量化匹配器
    AhoCorasick(AhoCorasick),
    /// 正则表达式匹配器
    Regex(Regex),
    /// 全词匹配专用正则
    WholeWordRegex(Regex),
}

impl CompiledMatcher {
    /// 从用户搜索条件构建对应的快速匹配执行器
    pub fn build(query: &SearchQuery) -> Result<Self, String> {
        if query.keyword.is_empty() {
            return Err("搜索关键字不能为空".to_string());
        }

        if query.is_regex {
            let regex_pattern = if query.case_sensitive {
                Regex::new(&query.keyword)
            } else {
                Regex::new(&format!("(?i){}", query.keyword))
            };
            return regex_pattern
                .map(CompiledMatcher::Regex)
                .map_err(|e| format!("正则表达式无效: {}", e));
        }

        if query.whole_word {
            // 全词匹配：使用单词边界 \b
            let escaped = regex::escape(&query.keyword);
            let pattern = if query.case_sensitive {
                format!(r"\b{}\b", escaped)
            } else {
                format!(r"(?i)\b{}\b", escaped)
            };
            return Regex::new(&pattern)
                .map(CompiledMatcher::WholeWordRegex)
                .map_err(|e| format!("全词匹配构建失败: {}", e));
        }

        // 默认极速多字节字面量匹配 (Aho-Corasick)
        let ac_builder = aho_corasick::AhoCorasick::builder()
            .ascii_case_insensitive(!query.case_sensitive)
            .build([&query.keyword]);

        ac_builder
            .map(CompiledMatcher::AhoCorasick)
            .map_err(|e| format!("构建匹配器失败: {}", e))
    }

    /// 在单行字符串中查找所有匹配项
    pub fn find_matches_in_line(&self, line: &str) -> Vec<(usize, usize)> {
        let mut matches = Vec::new();
        match self {
            CompiledMatcher::AhoCorasick(ac) => {
                for mat in ac.find_iter(line) {
                    matches.push((mat.start(), mat.end()));
                }
            }
            CompiledMatcher::Regex(re) | CompiledMatcher::WholeWordRegex(re) => {
                for mat in re.find_iter(line) {
                    matches.push((mat.start(), mat.end()));
                }
            }
        }
        matches
    }
}

/// 纯文本与代码文件快速搜索引擎
pub struct FastTextSearcher;

impl FastTextSearcher {
    /// 搜索单个纯文本或源代码文件
    pub fn search_file(
        path: &Path,
        file_size: u64,
        modified_time: u64,
        matcher: &CompiledMatcher,
    ) -> Option<FileMatchResult> {
        let file = File::open(path).ok()?;

        // 如果文件超过 32KB，使用零拷贝的内存映射 mmap 读取；否则直接读入内存
        let (content_cow, _encoding) = if file_size > 32 * 1024 {
            let mmap = unsafe { Mmap::map(&file).ok()? };
            if EncodingHelper::is_binary(&mmap) {
                return None; // 跳过二进制文件
            }
            let (decoded, enc) = EncodingHelper::decode_bytes(&mmap);
            (decoded, enc)
        } else {
            let bytes = std::fs::read(path).ok()?;
            if EncodingHelper::is_binary(&bytes) {
                return None;
            }
            let (decoded, enc) = EncodingHelper::decode_bytes(&bytes);
            (decoded, enc)
        };

        let mut matches = Vec::new();
        let mut match_count = 0;

        // 逐行扫描匹配
        for (line_idx, raw_line) in content_cow.lines().enumerate() {
            let line_number = line_idx + 1; // 1-based 行号
            let hit_ranges = matcher.find_matches_in_line(raw_line);

            if !hit_ranges.is_empty() {
                for (start_byte, end_byte) in hit_ranges {
                    match_count += 1;

                    // 计算字符偏移（处理 UTF-8 多字节字符）
                    let char_start = raw_line[..start_byte].chars().count();
                    let char_end = raw_line[..end_byte].chars().count();
                    let col_number = char_start + 1;

                    // 构造预览文本（如果单行过长，如压缩的 JS/JSON，截取包含关键词的上下文片段）
                    let (preview_line, preview_start, preview_end) =
                        Self::create_line_snippet(raw_line, start_byte, end_byte, char_start, char_end);

                    matches.push(MatchItem {
                        match_index: match_count,
                        line_number,
                        column_number: Some(col_number),
                        sheet_name: None,
                        cell_coord: None,
                        preview_line,
                        match_start: preview_start,
                        match_end: preview_end,
                    });
                }
            }
        }

        if matches.is_empty() {
            None
        } else {
            let file_name = path
                .file_name()
                .map(|s| s.to_string_lossy().to_string())
                .unwrap_or_default();
            let extension = path
                .extension()
                .map(|s| s.to_string_lossy().to_string().to_lowercase())
                .unwrap_or_default();

            Some(FileMatchResult {
                file_path: path.to_string_lossy().to_string(),
                file_name,
                extension,
                file_size,
                last_modified: modified_time,
                matches,
            })
        }
    }

    /// 创建限制长度的单行上下文预览片段
    fn create_line_snippet(
        full_line: &str,
        _start_byte: usize,
        _end_byte: usize,
        char_start: usize,
        char_end: usize,
    ) -> (String, usize, usize) {
        let chars: Vec<char> = full_line.chars().collect();
        let total_chars = chars.len();

        // 如果整行在 200 个字符内，直接返回整行
        if total_chars <= 200 {
            return (full_line.trim_end().to_string(), char_start, char_end);
        }

        // 超长单行截取策略：关键字前 40 字符，后 80 字符
        let slice_start = char_start.saturating_sub(40);
        let slice_end = (char_end + 80).min(total_chars);

        let mut snippet = String::new();
        if slice_start > 0 {
            snippet.push_str("...");
        }
        let snippet_body: String = chars[slice_start..slice_end].iter().collect();
        snippet.push_str(&snippet_body);
        if slice_end < total_chars {
            snippet.push_str("...");
        }

        let prefix_offset = if slice_start > 0 { 3 } else { 0 };
        let new_start = prefix_offset + (char_start - slice_start);
        let new_end = prefix_offset + (char_end - slice_start);

        (snippet, new_start, new_end)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matcher_substring() {
        let query = SearchQuery {
            root_path: "".to_string(),
            keyword: "test".to_string(),
            extensions: vec![],
            is_regex: false,
            case_sensitive: false,
            whole_word: false,
            include_subdirectories: true,
            ignore_hidden: true,
            max_file_size_mb: None,
        };

        let matcher = CompiledMatcher::build(&query).expect("matcher build failed");
        let matches = matcher.find_matches_in_line("This is a Test string with TEST content.");
        assert_eq!(matches.len(), 2);
    }

    #[test]
    fn test_matcher_whole_word() {
        let query = SearchQuery {
            root_path: "".to_string(),
            keyword: "test".to_string(),
            extensions: vec![],
            is_regex: false,
            case_sensitive: false,
            whole_word: true,
            include_subdirectories: true,
            ignore_hidden: true,
            max_file_size_mb: None,
        };

        let matcher = CompiledMatcher::build(&query).expect("matcher build failed");
        let matches = matcher.find_matches_in_line("test testing tested a test");
        assert_eq!(matches.len(), 2);
    }
}

