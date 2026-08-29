pub mod document_service;
pub mod encoding;
pub mod excel_search;
pub mod fast_text_search;
pub mod office_doc_search;
pub mod search_engine;
pub mod types;
pub mod updater_service;

use std::sync::Arc;
use tauri::{AppHandle, State};

use crate::document_service::DocumentService;
use crate::search_engine::{SearchEngine, SearchManager};
use crate::types::{ExcelWorkbookContent, SearchQuery, TextDocumentContent};
use crate::updater_service::{UpdateCheckResult, UpdaterService};

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

/// 命令：检查 GitHub Release 自动更新 (完全异步后台执行，零卡顿)
#[tauri::command]
async fn check_app_update() -> Result<UpdateCheckResult, String> {
    tauri::async_runtime::spawn_blocking(|| {
        UpdaterService::check_update()
    })
    .await
    .map_err(|e| format!("更新检查异步调度失败: {}", e))?
}

/// 命令：下载并执行更新替换 (完全异步后台执行)
#[tauri::command]
async fn install_app_update(tag_name: String) -> Result<(), String> {
    tauri::async_runtime::spawn_blocking(move || {
        UpdaterService::download_and_install_update(&tag_name)
    })
    .await
    .map_err(|e| format!("更新安装异步调度失败: {}", e))?
}

use tauri::{
    menu::{Menu, MenuItem, PredefinedMenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    LogicalSize, Manager, Size, WindowEvent,
};

/// 系统托盘菜单句柄状态，用于前端切换语言时动态更新托盘菜单文本
pub struct TrayMenuState {
    pub show_item: MenuItem<tauri::Wry>,
    pub reset_item: MenuItem<tauri::Wry>,
    pub quit_item: MenuItem<tauri::Wry>,
}

/// 命令：根据当前界面语言动态更新系统托盘右键菜单文字
#[tauri::command]
fn update_tray_menu_language(state: tauri::State<TrayMenuState>, lang: String) -> Result<(), String> {
    let (show_text, reset_text, quit_text) = match lang.as_str() {
        "ko" => ("메인 창 열기", "화면 위치 초기화 (중앙 복원)", "프로그램 종료"),
        "en" => ("Open Main Window", "Reset Window Position (Center)", "Exit Application"),
        _ => ("显示主窗口", "恢复窗口显示 (居中复位)", "退出程序"),
    };

    let _ = state.show_item.set_text(show_text);
    let _ = state.reset_item.set_text(reset_text);
    let _ = state.quit_item.set_text(quit_text);
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // 启动时静默清理历史升级备份文件 (.old)
    if let Ok(exe_path) = std::env::current_exe() {
        let old_exe = exe_path.with_extension("exe.old");
        if old_exe.exists() {
            let _ = std::fs::remove_file(old_exe);
        }
    }

    let search_manager = Arc::new(SearchManager::new());

    tauri::Builder::default()
        // 1. 防止重复运行单实例插件：二次启动时自动唤醒并置顶已有窗口
        .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            if let Some(window) = app.get_webview_window("main") {
                let _ = window.show();
                let _ = window.unminimize();
                let _ = window.set_focus();
            }
        }))
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
            open_with_system_app,
            check_app_update,
            install_app_update,
            update_tray_menu_language
        ])
        // 2. 拦截窗口关闭事件：点击 X 最小化隐藏到系统托盘，不真正退出
        .on_window_event(|window, event| {
            if let WindowEvent::CloseRequested { api, .. } = event {
                api.prevent_close();
                let _ = window.hide();
            }
        })
        // 3. 构建系统托盘与右键菜单（右键退出才算真正退出）
        .setup(|app| {
            let show_item = MenuItem::with_id(app, "show", "显示主窗口", true, None::<&str>)?;
            let reset_item = MenuItem::with_id(app, "reset_position", "恢复窗口显示 (居中复位)", true, None::<&str>)?;
            let separator = PredefinedMenuItem::separator(app)?;
            let quit_item = MenuItem::with_id(app, "quit", "退出程序", true, None::<&str>)?;
            let tray_menu = Menu::with_items(app, &[&show_item, &reset_item, &separator, &quit_item])?;

            // 注册托盘菜单状态到 Tauri 上下文供前端实时动态修改文字
            app.manage(TrayMenuState {
                show_item: show_item.clone(),
                reset_item: reset_item.clone(),
                quit_item: quit_item.clone(),
            });

            let _tray = TrayIconBuilder::new()
                .icon(app.default_window_icon().unwrap().clone())
                .tooltip("FlashText Search - 极速文本搜索")
                .menu(&tray_menu)
                .show_menu_on_left_click(false)
                .on_menu_event(|app, event| match event.id.as_ref() {
                    "show" => {
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.show();
                            let _ = window.unminimize();
                            let _ = window.set_focus();
                        }
                    }
                    "reset_position" => {
                        // 恢复窗口显示并将超出屏幕画面的程序复位到当前屏幕可视范围内
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.show();
                            let _ = window.unminimize();
                            let _ = window.set_size(Size::Logical(LogicalSize { width: 1280.0, height: 820.0 }));
                            let _ = window.center();
                            let _ = window.set_focus();
                        }
                    }
                    "quit" => {
                        app.exit(0);
                    }
                    _ => {}
                })
                .on_tray_icon_event(|tray, event| {
                    if let TrayIconEvent::Click {
                        button: MouseButton::Left,
                        button_state: MouseButtonState::Up,
                        ..
                    } = event
                    {
                        let app = tray.app_handle();
                        if let Some(window) = app.get_webview_window("main") {
                            if window.is_visible().unwrap_or(false) {
                                let _ = window.hide();
                            } else {
                                let _ = window.show();
                                let _ = window.unminimize();
                                let _ = window.set_focus();
                            }
                        }
                    }
                })
                .build(app)?;

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("运行 FlashTextSearch 应用程序时发生错误");
}


