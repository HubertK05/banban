use entity::activity;

use anyhow::Context;
use sea_orm::{ActiveModelTrait, DbConn, Set};
use tauri::State;

use crate::{database::activity::Mutation, errors::AppError};

#[tauri::command]
pub async fn create_activity(
    db: State<'_, DbConn>,
    title: String,
    body: String,
) -> Result<(), AppError> {
    let activity = activity::ActiveModel {
        title: Set(title),
        body: Set(body),
        column_id: Set(None),
        ..Default::default()
    }
    .save(&*db)
    .await
    .context("failed to create activity")?;

    Ok(())
}
