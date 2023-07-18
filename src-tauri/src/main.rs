// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::Manager;
pub mod commands;
pub mod setup;
use commands::*;

fn main() {
    dotenvy::dotenv().ok();
    setup::tracing();
    tauri::Builder::default()
        .setup(|app| {
            app.manage(setup::get_database_pool());
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![example::greet, activity::create_activity])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
