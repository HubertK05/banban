// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use sea_orm::Database;
use tauri::Manager;
use tauri_plugin_log::LogTarget;

pub mod commands;
use commands::*;

fn main() {
    dotenvy::dotenv().ok();

    tauri::Builder::default()
        .setup(|app| {
            let db = tauri::async_runtime::block_on(async {
                Database::connect("sqlite:../database.sqlite3?mode=rwc").await
            })
            .expect("Failed to connect to database");
            app.manage(db);
            Ok(())
        })
        .plugin(
            tauri_plugin_log::Builder::default()
                .targets([LogTarget::LogDir, LogTarget::Stdout, LogTarget::Webview])
                .build(),
        )
        .invoke_handler(tauri::generate_handler![example::greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
