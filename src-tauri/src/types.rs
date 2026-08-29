use serde::{Deserialize, Serialize};

/// 搜索请求参数结构体
/// 前端通过 Tauri invoke 传递该结构体发起搜索任务
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchQuery {
    /// 搜索根目录绝对路径
    pub root_path: String,
    /// 搜索关键字
    pub keyword: String,
    /// 指定文件后缀列表，如 ["txt", "xlsx", "log", "md"]，空则根据通用文本白名单过滤
    pub extensions: Vec<String>,
    /// 是否使用正则表达式匹配
    pub is_regex: bool,
    /// 是否区分大小写
    pub case_sensitive: bool,
    /// 是否全词匹配
    pub whole_word: bool,
    /// 是否递归遍历子目录
    pub include_subdirectories: bool,
    /// 忽略隐藏文件与目录（如 .git, .vscode）
    pub ignore_hidden: bool,
    /// 单个文件最大大小限制（MB），默认 200MB
    pub max_file_size_mb: Option<u64>,
}

/// 单处命中的详细位置信息
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MatchItem {
    /// 该文件内的匹配序号 (1-based, 比如 1, 2, 3...)
    pub match_index: usize,
    /// 行号 (纯文本/代码文件 1-based；Excel 为工作表的行索引 1-based)
    pub line_number: usize,
    /// 列号（字符偏移，1-based）
    pub column_number: Option<usize>,
    /// Excel 专属：命中的工作表名称（如 "Sheet1", "2026预算表"）
    pub sheet_name: Option<String>,
    /// Excel 专属：命中的单元格坐标（如 "A1", "C14"）
    pub cell_coord: Option<String>,
    /// 命中的单行或单元格预览文本内容（已去除首尾多余空白符）
    pub preview_line: String,
    /// 关键词在 preview_line 中的起始字符索引（用于前端高亮标记）
    pub match_start: usize,
    /// 关键词在 preview_line 中的结束字符索引
    pub match_end: usize,
}

/// 单个文件的搜索结果聚合
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FileMatchResult {
    /// 文件绝对路径
    pub file_path: String,
    /// 文件名（包含后缀，如 "readme.md"）
    pub file_name: String,
    /// 文件扩展名（小写，不带点，如 "txt", "xlsx"）
    pub extension: String,
    /// 文件大小（字节数）
    pub file_size: u64,
    /// 文件最后修改时间戳（毫秒）
    pub last_modified: u64,
    /// 该文件中所有命中位置列表
    pub matches: Vec<MatchItem>,
}

/// 实时搜索进度状态
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchProgress {
    /// 已扫描的文件总数
    pub files_scanned: usize,
    /// 包含匹配项的文件数量
    pub files_matched: usize,
    /// 累计命中的匹配项总数
    pub total_matches: usize,
    /// 当前已消耗时间（毫秒）
    pub elapsed_ms: u64,
    /// 搜索任务是否已结束
    pub is_finished: bool,
    /// 是否被用户手动取消
    pub is_cancelled: bool,
    /// 当前正在扫描的文件路径或目录简述
    pub current_file: Option<String>,
}

/// 纯文本/代码文件预览内容返回结构
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TextDocumentContent {
    /// 文件的完整或分段文本内容
    pub content: String,
    /// 文件检测出的编码格式（如 "UTF-8", "GB18030", "UTF-16LE"）
    pub encoding: String,
    /// 文件总行数
    pub total_lines: usize,
    /// 是否由于文件超大而截断了预览
    pub is_truncated: bool,
}

/// Excel 单个工作表预览数据
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExcelSheetContent {
    /// 工作表名称
    pub sheet_name: String,
    /// 表格二维数据矩阵（行 -> 列文本）
    pub rows: Vec<Vec<String>>,
    /// 总行数
    pub total_rows: usize,
    /// 最大列数
    pub max_cols: usize,
}

/// Excel 整个工作簿预览返回结构
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExcelWorkbookContent {
    /// 所有工作表名称列表
    pub sheet_names: Vec<String>,
    /// 各工作表数据列表
    pub sheets: Vec<ExcelSheetContent>,
}
