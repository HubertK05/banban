use entity::columns;
use sea_orm::DbConn;
use serde::Deserialize;
use tauri::State;

use crate::{database::columns::Mutation, errors::AppError};

#[tauri::command]
pub async fn create_column(
    db: State<'_, DbConn>,
    name: String,
) -> Result<columns::Model, AppError> {
    Mutation::insert_column(db.inner(), name).await
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RenameColumnInput {
    pub id: i32,
    pub new_name: String,
}

#[tauri::command]
pub async fn rename_column(db: State<'_, DbConn>, data: RenameColumnInput) -> Result<(), AppError> {
    Mutation::update_column_name(db.inner(), data).await
}

#[tauri::command]
pub async fn delete_column(db: State<'_, DbConn>, id: i32) -> Result<(), AppError> {
    Mutation::delete_column_by_id(db.inner(), id).await
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateColumnOrdinalInput {
    pub column_id: i32,
    pub new_ord: i32,
}

#[tauri::command]
pub async fn update_column_ordinal(db: State<'_, DbConn>, data: UpdateColumnOrdinalInput) -> Result<(), AppError> {
    Mutation::update_column_ordinal(db.inner(), data).await
}
