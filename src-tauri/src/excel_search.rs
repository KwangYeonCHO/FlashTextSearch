use std::path::Path;
use calamine::{open_workbook_auto, Data, Reader, Sheets};

use crate::fast_text_search::CompiledMatcher;
use crate::types::{FileMatchResult, MatchItem};

/// Excel 表格快速搜索引擎
pub struct ExcelSearcher;

impl ExcelSearcher {
    /// 搜索 Excel 文件中的所有工作表及单元格内容
    pub fn search_file(
        path: &Path,
        file_size: u64,
        modified_time: u64,
        matcher: &CompiledMatcher,
    ) -> Option<FileMatchResult> {
        let mut workbook: Sheets<_> = open_workbook_auto(path).ok()?;
        let sheet_names = workbook.sheet_names();

        let mut matches = Vec::new();
        let mut match_count = 0;

        for sheet_name in sheet_names {
            if let Ok(range) = workbook.worksheet_range(&sheet_name) {
                // 遍历该工作表内的所有有数据的单元格
                for (row_idx, row) in range.rows().enumerate() {
                    let row_number = row_idx + 1; // 1-based 行号

                    for (col_idx, cell) in row.iter().enumerate() {
                        let cell_text = Self::format_cell_data(cell);
                        if cell_text.is_empty() {
                            continue;
                        }

                        let hit_ranges = matcher.find_matches_in_line(&cell_text);
                        if !hit_ranges.is_empty() {
                            let col_letter = Self::col_index_to_letter(col_idx);
                            let cell_coord = format!("{}{}", col_letter, row_number);

                            for (start_byte, end_byte) in hit_ranges {
                                match_count += 1;

                                let char_start = cell_text[..start_byte].chars().count();
                                let char_end = cell_text[..end_byte].chars().count();

                                matches.push(MatchItem {
                                    match_index: match_count,
                                    line_number: row_number,
                                    column_number: Some(col_idx + 1),
                                    sheet_name: Some(sheet_name.clone()),
                                    cell_coord: Some(cell_coord.clone()),
                                    preview_line: cell_text.clone(),
                                    match_start: char_start,
                                    match_end: char_end,
                                });
                            }
                        }
                    }
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

    /// 将 Calamine 单元格数据转换为可搜索的文本字符串
    pub fn format_cell_data(cell: &Data) -> String {
        match cell {
            Data::Empty => String::new(),
            Data::String(s) => s.trim().to_string(),
            Data::Float(f) => {
                if f.fract() == 0.0 {
                    format!("{:.0}", f)
                } else {
                    format!("{}", f)
                }
            }
            Data::Int(i) => format!("{}", i),
            Data::Bool(b) => format!("{}", b),
            Data::DateTime(dt) => format!("{}", dt),
            Data::DateTimeIso(s) => s.clone(),
            Data::DurationIso(s) => s.clone(),
            Data::Error(e) => format!("{:?}", e),
        }
    }

    /// 将 0-based 列索引转换为 Excel 标准列号（0 -> "A", 1 -> "B", 26 -> "AA", 27 -> "AB" 等）
    pub fn col_index_to_letter(mut col_idx: usize) -> String {
        let mut result = Vec::new();
        loop {
            let rem = col_idx % 26;
            result.push((b'A' + rem as u8) as char);
            if col_idx < 26 {
                break;
            }
            col_idx = col_idx / 26 - 1;
        }
        result.into_iter().rev().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_column_letter_conversion() {
        assert_eq!(ExcelSearcher::col_index_to_letter(0), "A");
        assert_eq!(ExcelSearcher::col_index_to_letter(1), "B");
        assert_eq!(ExcelSearcher::col_index_to_letter(25), "Z");
        assert_eq!(ExcelSearcher::col_index_to_letter(26), "AA");
        assert_eq!(ExcelSearcher::col_index_to_letter(27), "AB");
    }
}

