use sea_orm::DbConn;
use serde::{Deserialize, Serialize};
use tauri::State;

use crate::{database::tags::Mutation, errors::AppError, utils::coloring::rgb_int_to_string};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateTagInput {
    pub tag_name: String,
    pub category_id: Option<i32>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateTagOutput {
    pub id: i32,
    pub tag_name: String,
    pub category_id: Option<i32>,
    pub color: String,
    pub ordinal: i32,
}

#[tauri::command]
pub async fn create_tag(
    db: State<'_, DbConn>,
    data: CreateTagInput,
) -> Result<CreateTagOutput, AppError> {
    let model = Mutation::create_tag(db.inner(), data).await?;
    Ok(CreateTagOutput {
        id: model.id,
        tag_name: model.tag_name,
        category_id: model.category_id,
        color: rgb_int_to_string(model.color),
        ordinal: model.ordinal,
    })
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AttachTagToCategoryInput {
    pub category_tag_id: i32,
    pub category_id: Option<i32>,
}

#[tauri::command]
pub async fn attach_tag_to_category(
    db: State<'_, DbConn>,
    data: AttachTagToCategoryInput,
) -> Result<(), AppError> {
    Mutation::update_category_tag(db.inner(), data).await
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateTagNameInput {
    pub category_tag_id: i32,
    pub tag_name: String,
}

#[tauri::command]
pub async fn update_tag_name(
    db: State<'_, DbConn>,
    data: UpdateTagNameInput,
) -> Result<(), AppError> {
    Mutation::update_tag_name(db.inner(), data).await
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateTagOrdinalInput {
    pub category_tag_id: i32,
    pub new_ord: i32,
}

#[tauri::command]
pub async fn update_tag_ordinal(
    db: State<'_, DbConn>,
    data: UpdateTagOrdinalInput,
) -> Result<(), AppError> {
    Mutation::update_tag_ordinal(db.inner(), data).await
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateTagColorInput {
    pub category_tag_id: i32,
    pub color: String,
}

#[tauri::command]
pub async fn update_tag_color(
    db: State<'_, DbConn>,
    data: UpdateTagColorInput,
) -> Result<(), AppError> {
    Mutation::update_tag_color(db.inner(), data).await
}

#[tauri::command]
pub async fn delete_tag(db: State<'_, DbConn>, category_tag_id: i32) -> Result<(), AppError> {
    Mutation::delete_tag_by_id(db.inner(), category_tag_id).await
}
