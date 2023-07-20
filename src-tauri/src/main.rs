// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use commands::activity::{create_activity, delete_activity};
// use app::{setup, commands::{example, activity}};
// use crate::{setup, commands::{example, activity}};
use tauri::Manager;

use crate::commands::{activity::{query_all_activities, update_activity_content}, category::*};

pub mod setup;
pub mod commands;
pub mod database;
pub mod errors;

fn main() {
    dotenvy::dotenv().ok();
    setup::tracing();
    tauri::Builder::default()
        .setup(|app| {
            app.manage(setup::get_database_pool());
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            create_activity,
            delete_activity,
            query_all_activities,
            update_activity_content,
            create_category,
            delete_category,
            select_all_categories,
            update_category_name,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
