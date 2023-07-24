use std::collections::HashMap;

use entity::categories;
use sea_orm::DbConn;
use serde::{Deserialize, Serialize};
use tauri::State;

use crate::{
    database::category::{Mutation, Query},
    errors::AppError,
};

#[tauri::command]
pub async fn create_category(
    db: State<'_, DbConn>,
    name: String,
) -> Result<categories::Model, AppError> {
    Mutation::insert_category(db.inner(), name).await
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TagData {
    pub id: i32,
    pub tag: String,
    pub ordinal: i32,
    pub color: String,
}

#[derive(Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SelectCategoryOutput {
    pub name: Option<String>,
    pub ordinal: Option<i32>,
    pub tags: Vec<TagData>,
}

impl SelectCategoryOutput {
    pub fn new_empty(name: Option<String>, ordinal: Option<i32>) -> Self {
        SelectCategoryOutput {
            name,
            ordinal,
            tags: vec![],
        }
    }
}

#[derive(Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SelectCategoryTagsOutput {
    pub category_tags: HashMap<i32, SelectCategoryOutput>,
    pub other_tags: SelectCategoryOutput,
}

#[tauri::command]
pub async fn select_all_categories(
    db: State<'_, DbConn>,
) -> Result<SelectCategoryTagsOutput, AppError> {
    Query::select_all_categories(db.inner()).await
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateCategoryNameInput {
    pub id: i32,
    pub name: String,
}

#[tauri::command]
pub async fn update_category_name(
    db: State<'_, DbConn>,
    data: UpdateCategoryNameInput,
) -> Result<(), AppError> {
    Mutation::update_category_name(db.inner(), data).await
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateCategoryOrdinalInput {
    pub category_id: i32,
    pub new_ord: i32,
}

#[tauri::command]
pub async fn update_category_ordinal(
    db: State<'_, DbConn>,
    data: UpdateCategoryOrdinalInput,
) -> Result<(), AppError> {
    Mutation::update_category_ordinal(db.inner(), data).await
}

#[tauri::command]
pub async fn delete_category(db: State<'_, DbConn>, id: i32) -> Result<(), AppError> {
    Mutation::delete_category_by_id(db.inner(), id).await
}
