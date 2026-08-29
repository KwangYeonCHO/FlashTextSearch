use chardetng::EncodingDetector;
use encoding_rs::{Encoding, GB18030, UTF_16BE, UTF_16LE, UTF_8};
use std::borrow::Cow;

/// 编码检测与字符串解码辅助模块
pub struct EncodingHelper;

impl EncodingHelper {
    /// 快速检查缓冲区是否为二进制文件
    /// 规则：检查前 1024 字节中是否存在 NULL (\0) 字节且非 UTF-16 模式
    pub fn is_binary(bytes: &[u8]) -> bool {
        if bytes.is_empty() {
            return false;
        }

        // 检查常见的 UTF-16 BOM
        if bytes.len() >= 2 && ((bytes[0] == 0xFF && bytes[1] == 0xFE) || (bytes[0] == 0xFE && bytes[1] == 0xFF)) {
            return false;
        }

        let sample_len = bytes.len().min(1024);
        let sample = &bytes[..sample_len];

        // 检查零字节数量与控制字符
        let mut null_count = 0;
        let mut non_ascii_control = 0;

        for &b in sample {
            if b == 0 {
                null_count += 1;
            } else if (b < 7 || (b > 13 && b < 27)) && b != 0x1B {
                non_ascii_control += 1;
            }
        }

        // 如果 null 字节过多，或者是明显的不可打印二进制字符聚集，判为二进制
        if null_count > 1 || (sample_len > 0 && (non_ascii_control * 100 / sample_len) > 20) {
            return true;
        }

        false
    }

    /// 智能检测字节数组的编码格式并解码为 Rust UTF-8 字符串
    /// 返回元组: (解码后的文本, 检测出的编码名称)
    pub fn decode_bytes(bytes: &[u8]) -> (String, &'static str) {
        if bytes.is_empty() {
            return (String::new(), "UTF-8");
        }

        // 1. 优先尝试标准的 UTF-8 快速解码（绝大多数代码与现代文本）
        if let Ok(valid_str) = std::str::from_utf8(bytes) {
            return (valid_str.to_string(), "UTF-8");
        }

        // 2. 检查 BOM 标记
        if bytes.len() >= 2 {
            if bytes[0] == 0xFF && bytes[1] == 0xFE {
                let (cow, _, _) = UTF_16LE.decode(&bytes[2..]);
                return (cow.into_owned(), "UTF-16LE");
            }
            if bytes[0] == 0xFE && bytes[1] == 0xFF {
                let (cow, _, _) = UTF_16BE.decode(&bytes[2..]);
                return (cow.into_owned(), "UTF-16BE");
            }
        }
        if bytes.len() >= 3 && bytes[0] == 0xEF && bytes[1] == 0xBB && bytes[2] == 0xBF {
            let (cow, _, _) = UTF_8.decode(&bytes[3..]);
            return (cow.into_owned(), "UTF-8-BOM");
        }

        // 3. 使用 Mozilla 的 chardetng 进行中文及多国语言高准确率探测
        let mut detector = EncodingDetector::new();
        detector.feed(bytes, true);
        let encoding: &'static Encoding = detector.guess(None, true);

        let (cow, _, had_errors) = encoding.decode(bytes);

        if !had_errors {
            return (cow.into_owned(), encoding.name());
        }

        // 4. 若有误码，对于中文 Windows 环境回退尝试 GB18030 (包含 GBK, GB2312)
        let (gbk_cow, _, gbk_errors) = GB18030.decode(bytes);
        if !gbk_errors {
            return (gbk_cow.into_owned(), "GB18030");
        }

        // 5. 兜底容错：将无法识别字符替换为 Unicode 占位符
        let (fallback_cow, _, _) = encoding.decode(bytes);
        (fallback_cow.into_owned(), encoding.name())
    }

    /// 快速将切片解码为 Cow<str>，避免不必要的堆内存分配
    pub fn decode_bytes_fast<'a>(bytes: &'a [u8]) -> Cow<'a, str> {
        if let Ok(valid_str) = std::str::from_utf8(bytes) {
            return Cow::Borrowed(valid_str);
        }
        let (decoded, _) = Self::decode_bytes(bytes);
        Cow::Owned(decoded)
    }
}
