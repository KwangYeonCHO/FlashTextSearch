pub mod document_service;
pub mod encoding;
pub mod excel_search;
pub mod fast_text_search;
pub mod search_engine;
pub mod types;

use std::sync::Arc;
use tauri::{AppHandle, State};

use crate::document_service::DocumentService;
use crate::search_engine::{SearchEngine, SearchManager};
use crate::types::{ExcelWorkbookContent, SearchQuery, TextDocumentContent};

/// 命令：弹出原生系统文件夹选择框
#[tauri::command]
fn select_folder() -> Option<String> {
    DocumentService::select_folder()
}

/// 命令：发起全文检索任务
#[tauri::command]
fn start_search(
    query: SearchQuery,
    state: State<'_, Arc<SearchManager>>,
    app_handle: AppHandle,
) -> Result<(), String> {
    let cancel_token = state.reset_cancel();
    SearchEngine::start_search(app_handle, query, cancel_token)
}

/// 命令：中止当前正在执行的搜索任务
#[tauri::command]
fn cancel_search(state: State<'_, Arc<SearchManager>>) {
    state.cancel();
}

/// 命令：读取纯文本或代码文件内容（供 Monaco 编辑器展示）
#[tauri::command]
fn read_text_file(path: String, max_lines: Option<usize>) -> Result<TextDocumentContent, String> {
    DocumentService::read_text_file(&path, max_lines)
}

/// 命令：读取 Excel 文件所有工作表与数据矩阵
#[tauri::command]
fn read_excel_file(path: String) -> Result<ExcelWorkbookContent, String> {
    DocumentService::read_excel_file(&path)
}

/// 命令：在 Windows 资源管理器中高亮选中指定文件
#[tauri::command]
fn open_in_file_manager(path: String) -> Result<(), String> {
    DocumentService::open_in_file_manager(&path)
}

/// 命令：使用系统默认应用程序打开文件
#[tauri::command]
fn open_with_system_app(path: String) -> Result<(), String> {
    DocumentService::open_with_system_app(&path)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let search_manager = Arc::new(SearchManager::new());

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .manage(search_manager)
        .invoke_handler(tauri::generate_handler![
            select_folder,
            start_search,
            cancel_search,
            read_text_file,
            read_excel_file,
            open_in_file_manager,
            open_with_system_app
        ])
        .run(tauri::generate_context!())
        .expect("运行 FlashTextSearch 应用程序时发生错误");
}

