use std::fs::File;
use std::io::Read;
use std::path::Path;
use quick_xml::events::Event;
use quick_xml::Reader;

use crate::fast_text_search::CompiledMatcher;
use crate::types::{FileMatchResult, MatchItem};

/// 办公文档与韩软文字处理检索器 (Word: .docx/.doc, Hancom: .hwpx/.hwp)
pub struct OfficeDocSearch;

impl OfficeDocSearch {
    /// 检查扩展名是否为受支持的 Word / HWP 文档格式
    pub fn is_supported_extension(ext: &str) -> bool {
        matches!(
            ext.to_ascii_lowercase().as_str(),
            "docx" | "doc" | "hwpx" | "hwp"
        )
    }

    /// 搜索单个 Word / HWP 文档并返回命中的全部结果聚合
    pub fn search_file(
        path: &Path,
        file_size: u64,
        modified_time: u64,
        matcher: &CompiledMatcher,
    ) -> Option<FileMatchResult> {
        let full_text = Self::extract_document_text(path).ok()?;
        let mut matches = Vec::new();
        let mut match_count = 0;

        for (line_idx, raw_line) in full_text.lines().enumerate() {
            let line_number = line_idx + 1; // 1-based 行号/段落号
            let hit_ranges = matcher.find_matches_in_line(raw_line);

            if !hit_ranges.is_empty() {
                for (start_byte, end_byte) in hit_ranges {
                    match_count += 1;

                    let char_start = raw_line[..start_byte].chars().count();
                    let char_end = raw_line[..end_byte].chars().count();
                    let col_number = char_start + 1;

                    let (preview_line, preview_start, preview_end) =
                        Self::create_line_snippet(raw_line, char_start, char_end);

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

    /// 提取文档全文（段落以换行符分隔）
    pub fn extract_document_text(path: &Path) -> Result<String, String> {
        let ext = path
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("")
            .to_ascii_lowercase();

        match ext.as_str() {
            "docx" => Self::extract_docx(path),
            "hwpx" => Self::extract_hwpx(path),
            "hwp" => Self::extract_hwp(path),
            "doc" => Self::extract_doc(path),
            _ => Err(format!("不支持的文档格式: .{}", ext)),
        }
    }

    /// 创建限制长度的单行上下文预览片段
    fn create_line_snippet(
        full_line: &str,
        char_start: usize,
        char_end: usize,
    ) -> (String, usize, usize) {
        let chars: Vec<char> = full_line.chars().collect();
        let total_chars = chars.len();

        if total_chars <= 200 {
            return (full_line.trim_end().to_string(), char_start, char_end);
        }

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

    /// 提取 .docx (Microsoft Word OpenXML) 全文
    fn extract_docx(path: &Path) -> Result<String, String> {
        let file = File::open(path).map_err(|e| format!("打开 docx 文件失败: {}", e))?;
        let mut archive = zip::ZipArchive::new(file).map_err(|e| format!("解析 docx Zip 失败: {}", e))?;

        let mut extracted_paragraphs = Vec::new();

        // 1. 读取主文档 word/document.xml
        if let Ok(mut doc_file) = archive.by_name("word/document.xml") {
            let mut xml_content = String::new();
            if doc_file.read_to_string(&mut xml_content).is_ok() {
                Self::parse_openxml_paragraphs(&xml_content, &mut extracted_paragraphs, "w");
            }
        }

        // 2. 如果主文档提取为空，尝试扫描所有 word/*.xml
        if extracted_paragraphs.is_empty() {
            let file_names: Vec<String> = archive
                .file_names()
                .filter(|n| n.starts_with("word/") && n.ends_with(".xml"))
                .map(|s| s.to_string())
                .collect();

            for name in file_names {
                if name.contains("document") || name.contains("header") || name.contains("footer") {
                    if let Ok(mut xml_file) = archive.by_name(&name) {
                        let mut xml_content = String::new();
                        if xml_file.read_to_string(&mut xml_content).is_ok() {
                            Self::parse_openxml_paragraphs(&xml_content, &mut extracted_paragraphs, "w");
                        }
                    }
                }
            }
        }

        Ok(extracted_paragraphs.join("\n"))
    }

    /// 提取 .hwpx (韩软 Hangul Word Processor OpenXML) 全文
    fn extract_hwpx(path: &Path) -> Result<String, String> {
        let file = File::open(path).map_err(|e| format!("打开 hwpx 文件失败: {}", e))?;
        let mut archive = zip::ZipArchive::new(file).map_err(|e| format!("解析 hwpx Zip 失败: {}", e))?;

        let mut extracted_paragraphs = Vec::new();

        let mut section_names: Vec<String> = archive
            .file_names()
            .filter(|n| {
                let lower = n.to_ascii_lowercase();
                lower.contains("section") && lower.ends_with(".xml")
            })
            .map(|s| s.to_string())
            .collect();

        section_names.sort();

        for name in section_names {
            if let Ok(mut xml_file) = archive.by_name(&name) {
                let mut xml_content = String::new();
                if xml_file.read_to_string(&mut xml_content).is_ok() {
                    Self::parse_openxml_paragraphs(&xml_content, &mut extracted_paragraphs, "hp");
                }
            }
        }

        Ok(extracted_paragraphs.join("\n"))
    }

    /// 使用 quick-xml 快速流式提取 XML 中的段落与文字
    fn parse_openxml_paragraphs(xml: &str, out_paragraphs: &mut Vec<String>, prefix: &str) {
        let mut reader = Reader::from_str(xml);
        reader.config_mut().trim_text(false);

        let mut buf = Vec::new();
        let mut in_p = false;
        let mut in_t = false;
        let mut current_p = String::new();

        let p_tag = format!("{}:p", prefix).into_bytes();
        let t_tag = format!("{}:t", prefix).into_bytes();

        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Start(ref e)) => {
                    let name = e.name();
                    let name_ref = name.as_ref();
                    if name_ref == p_tag.as_slice() || name_ref == b"p" {
                        in_p = true;
                        current_p.clear();
                    } else if name_ref == t_tag.as_slice() || name_ref == b"t" {
                        in_t = true;
                    }
                }
                Ok(Event::Text(ref e)) => {
                    if in_t || in_p {
                        if let Ok(text) = e.unescape() {
                            current_p.push_str(&text);
                        }
                    }
                }
                Ok(Event::End(ref e)) => {
                    let name = e.name();
                    let name_ref = name.as_ref();
                    if name_ref == t_tag.as_slice() || name_ref == b"t" {
                        in_t = false;
                    } else if name_ref == p_tag.as_slice() || name_ref == b"p" {
                        in_p = false;
                        let trimmed = current_p.trim();
                        if !trimmed.is_empty() {
                            out_paragraphs.push(trimmed.to_string());
                        }
                        current_p.clear();
                    }
                }
                Ok(Event::Eof) => break,
                Err(_) => break,
                _ => {}
            }
            buf.clear();
        }

        if !current_p.trim().is_empty() {
            out_paragraphs.push(current_p.trim().to_string());
        }
    }

    /// 提取 .hwp (Hangul Word Processor 5.0 OLE 二进制格式) 全文
    fn extract_hwp(path: &Path) -> Result<String, String> {
        let file = File::open(path).map_err(|e| format!("打开 HWP 文件失败: {}", e))?;
        let mut comp = cfb::CompoundFile::open(file)
            .map_err(|e| format!("解析 HWP 复合文档结构失败: {}", e))?;

        let mut is_compressed = true;
        if comp.is_stream("/FileHeader") {
            if let Ok(mut stream) = comp.open_stream("/FileHeader") {
                let mut header = [0u8; 256];
                if stream.read_exact(&mut header).is_ok() {
                    let flags = u32::from_le_bytes([header[36], header[37], header[38], header[39]]);
                    is_compressed = (flags & 1) != 0;
                }
            }
        }

        let mut extracted_paragraphs = Vec::new();

        for section_idx in 0..100 {
            let section_path = format!("/BodyText/Section{}", section_idx);
            if !comp.is_stream(&section_path) {
                break;
            }

            let mut raw_bytes = Vec::new();
            if let Ok(mut stream) = comp.open_stream(&section_path) {
                if stream.read_to_end(&mut raw_bytes).is_err() {
                    continue;
                }
            }

            let decompressed_data = if is_compressed {
                let mut decompressed = Vec::new();
                let mut decoder = flate2::read::DeflateDecoder::new(&raw_bytes[..]);
                if decoder.read_to_end(&mut decompressed).is_ok() {
                    decompressed
                } else {
                    let mut zlib_decomp = Vec::new();
                    let mut zlib_decoder = flate2::read::ZlibDecoder::new(&raw_bytes[..]);
                    if zlib_decoder.read_to_end(&mut zlib_decomp).is_ok() {
                        zlib_decomp
                    } else {
                        raw_bytes
                    }
                }
            } else {
                raw_bytes
            };

            Self::parse_hwp_records(&decompressed_data, &mut extracted_paragraphs);
        }

        Ok(extracted_paragraphs.join("\n"))
    }

    /// 解析 HWP 5.0 记录流
    fn parse_hwp_records(data: &[u8], out_paragraphs: &mut Vec<String>) {
        let mut offset = 0;
        let len = data.len();

        while offset + 4 <= len {
            let header = u32::from_le_bytes([
                data[offset],
                data[offset + 1],
                data[offset + 2],
                data[offset + 3],
            ]);
            offset += 4;

            let tag_id = (header & 0x3FF) as u16;
            let mut size = ((header >> 20) & 0xFFF) as usize;

            if size == 0xFFF && offset + 4 <= len {
                size = u32::from_le_bytes([
                    data[offset],
                    data[offset + 1],
                    data[offset + 2],
                    data[offset + 3],
                ]) as usize;
                offset += 4;
            }

            if offset + size > len {
                break;
            }

            if tag_id == 67 {
                let text_bytes = &data[offset..offset + size];
                let u16_chars: Vec<u16> = text_bytes
                    .chunks_exact(2)
                    .map(|chunk| u16::from_le_bytes([chunk[0], chunk[1]]))
                    .collect();

                let mut para_text = String::new();
                for &ch in &u16_chars {
                    if ch >= 0x0020 || ch == 0x0009 || ch == 0x000A || ch == 0x000D {
                        if let Some(c) = char::from_u32(ch as u32) {
                            para_text.push(c);
                        }
                    } else if ch == 0x0000 {
                        // skip
                    } else {
                        para_text.push(' ');
                    }
                }

                let trimmed = para_text.trim();
                if !trimmed.is_empty() {
                    out_paragraphs.push(trimmed.to_string());
                }
            }

            offset += size;
        }
    }

    /// 提取 .doc (Microsoft Word 97-2004 OLE 二进制格式) 全文
    fn extract_doc(path: &Path) -> Result<String, String> {
        let file = File::open(path).map_err(|e| format!("打开 DOC 文件失败: {}", e))?;
        let mut comp = cfb::CompoundFile::open(file)
            .map_err(|e| format!("打开 doc 复合文档结构失败: {}", e))?;

        if !comp.is_stream("/WordDocument") {
            return Err("未找到 WordDocument 数据流".to_string());
        }

        let mut raw_bytes = Vec::new();
        if let Ok(mut stream) = comp.open_stream("/WordDocument") {
            let _ = stream.read_to_end(&mut raw_bytes);
        }

        if raw_bytes.is_empty() {
            return Err("WordDocument 流内容为空".to_string());
        }

        let mut extracted = Vec::new();
        let mut current_line = String::new();

        let mut i = 0;
        while i + 1 < raw_bytes.len() {
            let u = u16::from_le_bytes([raw_bytes[i], raw_bytes[i + 1]]);
            if (u >= 0x0020 && u <= 0xD7AF) || u == 0x0009 || u == 0x000A || u == 0x000D {
                if let Some(c) = char::from_u32(u as u32) {
                    current_line.push(c);
                }
                i += 2;
            } else {
                if current_line.chars().count() >= 3 {
                    extracted.push(current_line.trim().to_string());
                }
                current_line.clear();
                i += 1;
            }
        }

        if current_line.chars().count() >= 3 {
            extracted.push(current_line.trim().to_string());
        }

        Ok(extracted.join("\n"))
    }
}
