use sea_orm::{DatabaseConnection, SqlxSqliteConnector};
use sqlx::migrate::Migrator;
use sqlx::SqlitePool;
#[cfg(not(dev))]
use tauri::path::PathResolver;
use tauri::App;
#[cfg(not(dev))]
use tauri::Manager;
use tracing_subscriber::prelude::*;
use tracing_subscriber::{fmt, EnvFilter};

const DATABASE_FILE_NAME: &str = "database.sqlite3";
pub fn tracing() {
    let fmt_layer = fmt::layer()
        .without_time()
        .with_line_number(true)
        .with_level(true)
        .with_target(true);

    let filter_layer = EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_new("trace"))
        .unwrap();

    tracing_subscriber::registry()
        .with(filter_layer)
        .with(fmt_layer)
        .init();
}

static MIGRATOR: Migrator = sqlx::migrate!("../migrations");

#[cfg(dev)]
pub fn get_database_pool(_app: &App) -> DatabaseConnection {
    trace!("Connecting to developer database");
    tauri::async_runtime::block_on(async {
        let pool = SqlitePool::connect(&format!("sqlite:../{DATABASE_FILE_NAME}?mode=rwc"))
            .await
            .unwrap();

        MIGRATOR
            .run(&pool)
            .await
            .expect("Failed to run database migrations");

        SqlxSqliteConnector::from_sqlx_sqlite_pool(pool)
    })
}

#[cfg(not(dev))]
pub fn get_database_pool(app: &App) -> DatabaseConnection {
    use tauri::Manager;

    tauri::async_runtime::block_on(async {
        let app_data_dir = app
            .path()
            .resolve(".", tauri::path::BaseDirectory::AppData)
            .unwrap();

        // Tauri path resolver returns the UNC path on Windows, starting with "\\?\".
        // This path format is generally not supported.
        // See issue https://github.com/tauri-apps/tauri/issues/5850.
        let app_data_dir = dunce::simplified(&app_data_dir);
        let file_path = app_data_dir.join(DATABASE_FILE_NAME);
        trace!("App data dir: {app_data_dir:?}, database file path: {file_path:?}");

        if !std::fs::exists(&file_path).unwrap() {
            std::fs::create_dir_all(&file_path).expect("failed to create database file file");
        }
        let url = format!(
            "sqlite:{}",
            app_data_dir
                .join(format!("{DATABASE_FILE_NAME}?mode=rwc"))
                .to_string_lossy()
        );
        trace!("Connecting to production database");
        let pool = SqlitePool::connect(&url).await.unwrap();

        if let Err(error) = MIGRATOR.run(&pool).await {
            warn!("Failed to run database migrations: {error}");
            std::fs::write(file_path, "").expect("failed to clear database file");
        }

        MIGRATOR
            .run(&pool)
            .await
            .expect("failed to run database migrations");

        SqlxSqliteConnector::from_sqlx_sqlite_pool(pool)
    })
}
