use anyhow::Context;
use entity::activities::{self, Model};
use sea_orm::{ActiveModelTrait, DbConn, Set};
use tauri::State;

use crate::{database::activity::Mutation, errors::AppError};

#[tauri::command]
pub async fn create_activity(
    db: State<'_, DbConn>,
    name: String,
    body: String,
) -> Result<(), AppError> {
    let activity: activities::ActiveModel = activities::ActiveModel {
        name: Set(name),
        body: Set(Some(body)),
        column_id: Set(None),
        ..Default::default()
    };

    let model = activity.insert(&*db).await.unwrap();

    Ok(())
}
