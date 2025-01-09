use tauri::Manager;

#[tauri::command]
pub async fn close_splashscreen(window: tauri::Window) {
    if let Some(splashscreen) = window.get_webview_window("splashscreen") {
        splashscreen.close().unwrap();
    }
    window.get_webview_window("main").unwrap().show().unwrap();
}
