// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::Manager;

use crate::commands::{activity::*, category::*, columns::*, tags::*};

pub mod commands;
pub mod database;
pub mod errors;
pub mod setup;

fn main() {
    dotenvy::dotenv().ok();
    setup::tracing();
    tauri::Builder::default()
        .setup(|app| {
            app.manage(setup::get_database_pool(&app.config()));
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            create_activity,
            delete_activity,
            query_all_activities,
            update_activity_content,
            create_category,
            delete_category,
            create_column,
            rename_column,
            delete_column,
            update_column_ordinal,
            select_all_categories,
            update_category_name,
            create_tag,
            attach_tag_to_category,
            update_tag_name,
            delete_tag,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
