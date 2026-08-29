use std::fs::File;
use std::path::Path;
use std::process::Command;
use calamine::{open_workbook_auto, Reader, Sheets};
use memmap2::Mmap;

use crate::encoding::EncodingHelper;
use crate::excel_search::ExcelSearcher;
use crate::types::{ExcelSheetContent, ExcelWorkbookContent, TextDocumentContent};

/// 文档读取与系统原生操作服务
pub struct DocumentService;

impl DocumentService {
    /// 打开原生 Windows 文件夹选择对话框
    pub fn select_folder() -> Option<String> {
        let folder = rfd::FileDialog::new()
            .set_title("选择搜索根目录")
            .pick_folder()?;
        Some(folder.to_string_lossy().to_string())
    }

    /// 读取纯文本/代码文件内容以供右侧 Monaco 编辑器预览
    pub fn read_text_file(path_str: &str, max_lines_limit: Option<usize>) -> Result<TextDocumentContent, String> {
        let path = Path::new(path_str);
        if !path.exists() {
            return Err("文件不存在".to_string());
        }

        let metadata = std::fs::metadata(path).map_err(|e| format!("无法读取文件元数据: {}", e))?;
        let file_size = metadata.len();

        let file = File::open(path).map_err(|e| format!("打开文件失败: {}", e))?;

        // 根据文件大小决定读取策略
        let (full_text, encoding) = if file_size > 64 * 1024 {
            let mmap = unsafe { Mmap::map(&file).map_err(|e| format!("内存映射失败: {}", e))? };
            if EncodingHelper::is_binary(&mmap) {
                return Err("该文件为二进制文件，无法作为纯文本预览".to_string());
            }
            let (text, enc) = EncodingHelper::decode_bytes(&mmap);
            (text, enc)
        } else {
            let bytes = std::fs::read(path).map_err(|e| format!("读取文件失败: {}", e))?;
            if EncodingHelper::is_binary(&bytes) {
                return Err("该文件为二进制文件，无法作为纯文本预览".to_string());
            }
            let (text, enc) = EncodingHelper::decode_bytes(&bytes);
            (text, enc)
        };

        let mut lines = Vec::new();
        let mut total_lines = 0;
        let limit = max_lines_limit.unwrap_or(20000); // 默认最多加载 20000 行，保证流畅
        let mut is_truncated = false;

        for line in full_text.lines() {
            total_lines += 1;
            if total_lines <= limit {
                lines.push(line);
            } else {
                is_truncated = true;
            }
        }

        let content = lines.join("\n");

        Ok(TextDocumentContent {
            content,
            encoding: encoding.to_string(),
            total_lines,
            is_truncated,
        })
    }

    /// 读取 Excel 工作簿结构及所有单元格矩阵
    pub fn read_excel_file(path_str: &str) -> Result<ExcelWorkbookContent, String> {
        let path = Path::new(path_str);
        if !path.exists() {
            return Err("Excel 文件不存在".to_string());
        }

        let mut workbook: Sheets<_> = open_workbook_auto(path)
            .map_err(|e| format!("无法打开 Excel 工作簿: {}", e))?;

        let sheet_names = workbook.sheet_names();
        let mut sheets_data = Vec::new();

        for sheet_name in &sheet_names {
            if let Ok(range) = workbook.worksheet_range(sheet_name) {
                let total_rows = range.height();
                let max_cols = range.width();

                // 提取前 500 行作为预览，避免超大表格卡顿
                let preview_row_limit = total_rows.min(500);
                let mut rows = Vec::with_capacity(preview_row_limit);

                for row in range.rows().take(preview_row_limit) {
                    let mut row_cells = Vec::with_capacity(row.len());
                    for cell in row {
                        row_cells.push(ExcelSearcher::format_cell_data(cell));
                    }
                    rows.push(row_cells);
                }

                sheets_data.push(ExcelSheetContent {
                    sheet_name: sheet_name.clone(),
                    rows,
                    total_rows,
                    max_cols,
                });
            }
        }

        Ok(ExcelWorkbookContent {
            sheet_names,
            sheets: sheets_data,
        })
    }

    /// 在 Windows 资源管理器中高亮定位该文件
    pub fn open_in_file_manager(path_str: &str) -> Result<(), String> {
        let path = Path::new(path_str);
        if !path.exists() {
            return Err("目标文件不存在".to_string());
        }

        #[cfg(target_os = "windows")]
        {
            Command::new("explorer")
                .arg(format!("/select,{}", path_str.replace('/', "\\")))
                .spawn()
                .map_err(|e| format!("打开资源管理器失败: {}", e))?;
        }

        #[cfg(not(target_os = "windows"))]
        {
            let parent = path.parent().unwrap_or(path);
            let _ = open::that(parent);
        }

        Ok(())
    }

    /// 使用 Windows 系统默认关联程序打开该文件（如 Excel、VSCode、记事本）
    pub fn open_with_system_app(path_str: &str) -> Result<(), String> {
        let path = Path::new(path_str);
        if !path.exists() {
            return Err("目标文件不存在".to_string());
        }

        #[cfg(target_os = "windows")]
        {
            Command::new("cmd")
                .args(["/c", "start", "", path_str])
                .spawn()
                .map_err(|e| format!("启动默认关联应用失败: {}", e))?;
        }

        #[cfg(not(target_os = "windows"))]
        {
            open::that(path).map_err(|e| format!("打开失败: {}", e))?;
        }

        Ok(())
    }
}
