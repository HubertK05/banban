use sea_orm::{DatabaseConnection, SqlxSqliteConnector};
use sqlx::migrate::Migrator;
use sqlx::SqlitePool;
use tauri::api::path::app_data_dir;
use tauri::Config;
use tracing_subscriber::prelude::*;
use tracing_subscriber::{fmt, EnvFilter};

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
pub fn get_database_pool(config: &Config) -> DatabaseConnection {
    trace!("Connecting to developer database");
    tauri::async_runtime::block_on(async {
        let pool = SqlitePool::connect("sqlite:../database.sqlite3?mode=rwc")
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
pub fn get_database_pool(config: &Config) -> DatabaseConnection {
    tauri::async_runtime::block_on(async {
        let app_data_dir = app_data_dir(config).unwrap();
        trace!("App data dir: {app_data_dir:?}");
        std::fs::create_dir_all(&app_data_dir).unwrap();
        let url = format!(
            "sqlite:{}",
            app_data_dir
                .join("database.sqlite3?mode=rwc")
                .to_string_lossy()
        );
        trace!("Connecting to production database");
        let pool = SqlitePool::connect(&url).await.unwrap();

        MIGRATOR
            .run(&pool)
            .await
            .expect("Failed to run database migrations");

        SqlxSqliteConnector::from_sqlx_sqlite_pool(pool)
    })
}
