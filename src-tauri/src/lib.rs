// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[macro_use]
extern crate tracing;

use crate::commands::{activity::*, category::*, columns::*, fetch::*, splashscreen::*, tags::*};
use tauri::Manager;

pub mod commands;
pub mod database;
pub mod errors;
pub mod setup;
pub mod utils;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    dotenvy::dotenv().ok();
    setup::tracing();
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            app.manage(setup::get_database_pool(&app.config()));
            info!("Spinning up banban");
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            create_activity,
            delete_activity,
            query_all_activities,
            update_activity_content,
            update_activity_column,
            add_tag_to_activity,
            remove_tag_from_activity,
            create_category,
            delete_category,
            create_column,
            rename_column,
            delete_column,
            update_column_ordinal,
            select_all_categories,
            update_category_name,
            update_category_ordinal,
            create_tag,
            attach_tag_to_category,
            update_tag_name,
            update_tag_ordinal,
            update_tag_color,
            delete_tag,
            create_column,
            rename_column,
            delete_column,
            update_column_ordinal,
            close_splashscreen,
            fetch_all
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
