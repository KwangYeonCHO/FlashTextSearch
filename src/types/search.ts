/**
 * 搜索查询参数接口
 */
export interface SearchQuery {
  /** 搜索根目录绝对路径 */
  rootPath: string;
  /** 搜索关键词 */
  keyword: string;
  /** 扩展名过滤列表，如 ["txt", "xlsx", "log", "md"] */
  extensions: string[];
  /** 是否使用正则表达式 */
  isRegex: boolean;
  /** 是否区分大小写 */
  caseSensitive: boolean;
  /** 是否全词匹配 */
  wholeWord: boolean;
  /** 是否递归搜索子目录 */
  includeSubdirectories: boolean;
  /** 是否忽略隐藏文件与目录 (.git, .vscode 等) */
  ignoreHidden: boolean;
  /** 单个文件最大大小限制（MB） */
  maxFileSizeMb?: number;
}

/**
 * 单处匹配位置接口
 */
export interface MatchItem {
  /** 该文件内的匹配序号 (1-based) */
  matchIndex: number;
  /** 行号 (纯文本 1-based，Excel 为行索引 1-based) */
  lineNumber: number;
  /** 列号 (1-based) */
  columnNumber?: number;
  /** Excel 专属：工作表名称 */
  sheetName?: string;
  /** Excel 专属：单元格坐标，如 "B14" */
  cellCoord?: string;
  /** 预览行内容 */
  previewLine: string;
  /** 关键词在 previewLine 中的起始字符偏移 */
  matchStart: number;
  /** 关键词在 previewLine 中的结束字符偏移 */
  matchEnd: number;
}

/**
 * 单个文件的搜索结果聚合
 */
export interface FileMatchResult {
  /** 文件绝对路径 */
  filePath: string;
  /** 文件名 */
  fileName: string;
  /** 文件后缀（小写，不带点） */
  extension: string;
  /** 文件大小（字节） */
  fileSize: number;
  /** 最后修改时间戳 (毫秒) */
  lastModified: number;
  /** 该文件内的所有匹配项列表 */
  matches: MatchItem[];
}

/**
 * 实时搜索进度状态
 */
export interface SearchProgress {
  /** 已扫描的文件总数 */
  filesScanned: number;
  /** 包含匹配项的文件数量 */
  filesMatched: number;
  /** 累计命中的匹配项总数 */
  totalMatches: number;
  /** 耗时 (毫秒) */
  elapsedMs: number;
  /** 是否已搜索完毕 */
  isFinished: boolean;
  /** 是否被用户取消 */
  isCancelled: boolean;
  /** 当前正在扫描的文件 */
  currentFile?: string;
}

/**
 * 纯文本/代码文件预览内容
 */
export interface TextDocumentContent {
  content: string;
  encoding: string;
  totalLines: number;
  isTruncated: boolean;
}

/**
 * Excel 工作表数据
 */
export interface ExcelSheetContent {
  sheetName: string;
  rows: string[][];
  totalRows: number;
  maxCols: number;
}

/**
 * Excel 整个工作簿数据
 */
export interface ExcelWorkbookContent {
  sheetNames: string[];
  sheets: ExcelSheetContent[];
}
