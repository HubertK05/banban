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

pub fn get_database_pool(config: &Config) -> DatabaseConnection {
    tauri::async_runtime::block_on(async {
        std::fs::create_dir_all(app_data_dir(config).unwrap()).unwrap();
        let dir = app_data_dir(config)
            .unwrap()
            .join("database.sqlite3?mode=rwc");
        trace!("Database directory: {dir:?}");
        let dir = format!("sqlite:{}", dir.to_string_lossy());
        let pool = match SqlitePool::connect(&dir).await {
            Ok(c) => c,
            Err(e) => {
                std::fs::write(&dir, &[]).unwrap();
                SqlitePool::connect(&dir).await.unwrap()
            }
        };

        MIGRATOR
            .run(&pool)
            .await
            .expect("Failed to run database migrations");

        SqlxSqliteConnector::from_sqlx_sqlite_pool(pool)
    })
}
