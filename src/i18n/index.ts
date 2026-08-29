import { ref, computed } from "vue";

export type LanguageKey = "zh" | "ko" | "en";

export interface TranslationDictionary {
  appTitle: string;
  appSubtitle: string;
  folderPlaceholder: string;
  keywordPlaceholder: string;
  browse: string;
  startSearch: string;
  stopSearch: string;
  formatFilter: string;
  presetAll: string;
  presetText: string;
  presetExcel: string;
  presetCode: string;
  presetCustom: string;
  customPlaceholder: string;
  caseSensitive: string;
  caseSensitiveTip: string;
  regex: string;
  regexTip: string;
  wholeWord: string;
  wholeWordTip: string;
  subdirectories: string;
  subdirectoriesTip: string;
  searchResults: string;
  files: string;
  matches: string;
  hitsCount: string;
  expandAll: string;
  collapseAll: string;
  noResults: string;
  noResultsTip: string;
  documentPreview: string;
  selectFileTip: string;
  selectFileSubTip: string;
  matchIndex: string;
  prevMatchTip: string;
  nextMatchTip: string;
  locateInExplorer: string;
  openWithApp: string;
  loadingDocument: string;
  readError: string;
  openDefaultApp: string;
  truncatedNotice: string;
  sheetRows: string;
  noSheetData: string;
  ready: string;
  scanning: string;
  cancelled: string;
  finished: string;
  scanned: string;
  matchedFiles: string;
  totalHits: string;
  elapsedTime: string;
  speed: string;
  filesPerSec: string;
  f3Next: string;
  shiftF3Prev: string;
  scanningCurrent: string;
  theme: string;
  themeDarkSlate: string;
  themeMidnightBlue: string;
  themeObsidianGold: string;
  themeLightCrisp: string;
  language: string;
  searchHistory: string;
  folderHistory: string;
  clearAll: string;
  noHistory: string;
  deleteItem: string;
  checkUpdates: string;
  newVersionAvailable: string;
  currentVersion: string;
  latestVersion: string;
  updateNow: string;
  remindLater: string;
  updating: string;
  isLatestVersion: string;
}

export const translations: Record<LanguageKey, TranslationDictionary> = {
  zh: {
    appTitle: "FlashText Search",
    appSubtitle: "极速文本与文档搜索",
    folderPlaceholder: "选择或输入搜索根目录路径...",
    keywordPlaceholder: "输入要搜索的文本内容 (例如 test, function, 订单号)...",
    browse: "浏览",
    startSearch: "极速搜索",
    stopSearch: "停止搜索",
    formatFilter: "格式过滤:",
    presetAll: "全部文本/代码",
    presetText: "纯文本 (*.txt, *.log, *.md)",
    presetExcel: "Excel 表格 (*.xlsx, *.xls)",
    presetCode: "源代码文件",
    presetCustom: "自定义后缀...",
    customPlaceholder: "如 txt, xlsx, log",
    caseSensitive: "区分大小写",
    caseSensitiveTip: "区分英文字母大小写",
    regex: "正则表达式",
    regexTip: "使用正则表达式检索",
    wholeWord: "全词匹配",
    wholeWordTip: "仅匹配完整独立的单词",
    subdirectories: "包含子目录",
    subdirectoriesTip: "递归搜索所有子文件夹",
    searchResults: "搜索结果",
    files: "个文件",
    matches: "处命中",
    hitsCount: "处",
    expandAll: "全部展开",
    collapseAll: "全部折叠",
    noResults: "未找到匹配内容",
    noResultsTip: "请尝试更换搜索目录、关键字或检查扩展名过滤",
    documentPreview: "文档预览与定位",
    selectFileTip: "选择左侧搜索结果查看文档预览",
    selectFileSubTip: "支持实时平滑跳转到指定命中行、多匹配项导航与 Excel 工作表单元格高亮",
    matchIndex: "匹配项:",
    prevMatchTip: "跳转到上一个匹配项 (Shift + F3)",
    nextMatchTip: "跳转到下一个匹配项 (F3)",
    locateInExplorer: "定位",
    openWithApp: "打开",
    loadingDocument: "正在极速加载文档内容...",
    readError: "无法读取该文档内容",
    openDefaultApp: "使用系统默认应用打开该文档",
    truncatedNotice: "前 20,000 行预览",
    sheetRows: "行",
    noSheetData: "当前工作表无数据",
    ready: "就绪",
    scanning: "极速扫描中...",
    cancelled: "已取消",
    finished: "搜索完成",
    scanned: "已扫描:",
    matchedFiles: "匹配文件:",
    totalHits: "总匹配项:",
    elapsedTime: "耗时:",
    speed: "速度:",
    filesPerSec: "文件/秒",
    f3Next: "F3 下一个匹配",
    shiftF3Prev: "Shift+F3 上一个匹配",
    scanningCurrent: "正在扫描:",
    theme: "主题",
    themeDarkSlate: "极夜暗影 (Dark Slate)",
    themeMidnightBlue: "深海星夜 (Midnight Navy)",
    themeObsidianGold: "黑曜金奢 (Obsidian Gold)",
    themeLightCrisp: "晨曦素雅 (Light Crisp)",
    language: "语言",
    searchHistory: "搜索内容历史",
    folderHistory: "搜索目录历史",
    clearAll: "清空全部",
    noHistory: "暂无历史记录",
    deleteItem: "删除此条记录",
    checkUpdates: "检查更新",
    newVersionAvailable: "发现新版本可用",
    currentVersion: "当前版本:",
    latestVersion: "最新版本:",
    updateNow: "立即自动升级并重启",
    remindLater: "稍后提醒",
    updating: "正在从 GitHub 下载最新版本并应用更新，请稍候...",
    isLatestVersion: "当前已是最新版本",
  },
  ko: {
    appTitle: "FlashText Search",
    appSubtitle: "초고속 텍스트 & 문서 검색",
    folderPlaceholder: "검색할 폴더 경로를 선택하거나 입력하세요...",
    keywordPlaceholder: "검색할 텍스트를 입력하세요 (예: test, function, 주문번호)...",
    browse: "찾아보기",
    startSearch: "초고속 검색",
    stopSearch: "검색 중지",
    formatFilter: "형식 필터:",
    presetAll: "모든 텍스트/코드",
    presetText: "일반 텍스트 (*.txt, *.log, *.md)",
    presetExcel: "Excel 시트 (*.xlsx, *.xls)",
    presetCode: "소스 코드 파일",
    presetCustom: "사용자 지정 확장자...",
    customPlaceholder: "예: txt, xlsx, log",
    caseSensitive: "대소문자 구분",
    caseSensitiveTip: "영문 대소문자를 구분하여 검색",
    regex: "정규식",
    regexTip: "정규 표현식으로 검색",
    wholeWord: "단어 단위",
    wholeWordTip: "온전한 단어 일치 항목만 검색",
    subdirectories: "하위 폴더 포함",
    subdirectoriesTip: "모든 하위 디렉터리 재귀 검색",
    searchResults: "검색 결과",
    files: "개 파일",
    matches: "건 일치",
    hitsCount: "건",
    expandAll: "모두 펼치기",
    collapseAll: "모두 접기",
    noResults: "일치하는 내용이 없습니다",
    noResultsTip: "검색 경로, 검색어 또는 확장자 설정을 확인해 보세요",
    documentPreview: "문서 미리보기 및 위치 이동",
    selectFileTip: "왼쪽 검색 결과를 선택하여 미리보기를 확인하세요",
    selectFileSubTip: "일치하는 줄로 자동 스크롤, 다중 일치 탐색 및 Excel 셀 강조 표시 지원",
    matchIndex: "일치 항목:",
    prevMatchTip: "이전 일치 항목으로 이동 (Shift + F3)",
    nextMatchTip: "다음 일치 항목으로 이동 (F3)",
    locateInExplorer: "탐색기 위치",
    openWithApp: "열기",
    loadingDocument: "문서 내용을 빠르게 불러오는 중...",
    readError: "문서 내용을 읽을 수 없습니다",
    openDefaultApp: "기본 연결 프로그램으로 열기",
    truncatedNotice: "상위 20,000행 미리보기",
    sheetRows: "행",
    noSheetData: "현재 워크시트에 데이터가 없습니다",
    ready: "준비 완료",
    scanning: "초고속 검색 중...",
    cancelled: "취소됨",
    finished: "검색 완료",
    scanned: "스캔 완료:",
    matchedFiles: "일치 파일:",
    totalHits: "총 일치 건수:",
    elapsedTime: "소요 시간:",
    speed: "속도:",
    filesPerSec: "파일/초",
    f3Next: "F3 다음 일치",
    shiftF3Prev: "Shift+F3 이전 일치",
    scanningCurrent: "스캔 중:",
    theme: "테마",
    themeDarkSlate: "다크 슬레이트 (Dark Slate)",
    themeMidnightBlue: "미드나잇 네이비 (Midnight Navy)",
    themeObsidianGold: "옵시디언 골드 (Obsidian Gold)",
    themeLightCrisp: "라이트 크리스프 (Light Crisp)",
    language: "언어",
    searchHistory: "검색어 기록",
    folderHistory: "폴더 기록",
    clearAll: "모두 지우기",
    noHistory: "검색 기록이 없습니다",
    deleteItem: "이 기록 삭제",
    checkUpdates: "업데이트 확인",
    newVersionAvailable: "새 버전 사용 가능",
    currentVersion: "현재 버전:",
    latestVersion: "최신 버전:",
    updateNow: "지금 자동 업데이트 및 재시작",
    remindLater: "나중에 알림",
    updating: "GitHub에서 최신 버전을 다운로드하여 적용하는 중입니다...",
    isLatestVersion: "현재 최신 버전입니다",
  },
  en: {
    appTitle: "FlashText Search",
    appSubtitle: "Ultra-Fast Text & Document Search",
    folderPlaceholder: "Select or enter search root directory path...",
    keywordPlaceholder: "Enter search keyword (e.g. test, function, order_id)...",
    browse: "Browse",
    startSearch: "Fast Search",
    stopSearch: "Stop Search",
    formatFilter: "Format Filter:",
    presetAll: "All Text / Code",
    presetText: "Plain Text (*.txt, *.log, *.md)",
    presetExcel: "Excel Spreadsheets (*.xlsx, *.xls)",
    presetCode: "Source Code Files",
    presetCustom: "Custom Extensions...",
    customPlaceholder: "e.g. txt, xlsx, log",
    caseSensitive: "Case Sensitive",
    caseSensitiveTip: "Match exact uppercase and lowercase characters",
    regex: "Regex",
    regexTip: "Use Regular Expressions",
    wholeWord: "Whole Word",
    wholeWordTip: "Match only whole isolated words",
    subdirectories: "Include Subfolders",
    subdirectoriesTip: "Recursively search all subdirectories",
    searchResults: "Search Results",
    files: "files",
    matches: "matches",
    hitsCount: "matches",
    expandAll: "Expand All",
    collapseAll: "Collapse All",
    noResults: "No matching content found",
    noResultsTip: "Try changing the folder path, keyword, or extension filters",
    documentPreview: "Document Preview & Jump",
    selectFileTip: "Select a search result from the left to view document preview",
    selectFileSubTip: "Supports smooth scrolling to matched line, multi-match navigation, and Excel cell highlight",
    matchIndex: "Match:",
    prevMatchTip: "Jump to previous match (Shift + F3)",
    nextMatchTip: "Jump to next match (F3)",
    locateInExplorer: "Locate",
    openWithApp: "Open",
    loadingDocument: "Loading document content...",
    readError: "Unable to read document content",
    openDefaultApp: "Open in system default application",
    truncatedNotice: "First 20,000 lines preview",
    sheetRows: "rows",
    noSheetData: "No data in current worksheet",
    ready: "Ready",
    scanning: "Scanning at high speed...",
    cancelled: "Cancelled",
    finished: "Search Finished",
    scanned: "Scanned:",
    matchedFiles: "Matched Files:",
    totalHits: "Total Hits:",
    elapsedTime: "Time Elapsed:",
    speed: "Speed:",
    filesPerSec: "files/sec",
    f3Next: "F3 Next Match",
    shiftF3Prev: "Shift+F3 Prev Match",
    scanningCurrent: "Scanning:",
    theme: "Theme",
    themeDarkSlate: "Dark Slate (Default)",
    themeMidnightBlue: "Midnight Navy",
    themeObsidianGold: "Obsidian Gold",
    themeLightCrisp: "Light Crisp",
    language: "Language",
    searchHistory: "Search History",
    folderHistory: "Folder History",
    clearAll: "Clear All",
    noHistory: "No history records",
    deleteItem: "Delete this record",
    checkUpdates: "Check Updates",
    newVersionAvailable: "New Version Available",
    currentVersion: "Current:",
    latestVersion: "Latest:",
    updateNow: "Update & Restart Now",
    remindLater: "Remind Me Later",
    updating: "Downloading latest release from GitHub and applying update...",
    isLatestVersion: "You are up to date",
  },
};

// 语言状态管理与持久化存储
const savedLang = (localStorage.getItem("flashtext_lang") as LanguageKey) || "zh";
export const currentLang = ref<LanguageKey>(savedLang);

export const setLanguage = (lang: LanguageKey) => {
  currentLang.value = lang;
  localStorage.setItem("flashtext_lang", lang);
};

export const t = computed(() => translations[currentLang.value] || translations.zh);
