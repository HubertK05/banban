use std::collections::HashMap;

use entity::categories;
use sea_orm::DbConn;
use serde::{Serialize, Deserialize};
use tauri::State;

use crate::{errors::AppError, database::category::{Mutation, Query}};

#[tauri::command]
pub async fn create_category(db: State<'_, DbConn>, name: String) -> Result<categories::Model, AppError> {
    Mutation::insert_category(db.inner(), name).await
}

#[derive(Serialize)]
pub struct SelectCategoryOutput {
    pub name: String,
    pub tags: Vec<String>,
}

pub type SelectCategoriesOutput = HashMap<i32, SelectCategoryOutput>;

#[tauri::command]
pub async fn select_all_categories(db: State<'_, DbConn>) -> Result<SelectCategoriesOutput, AppError> {
    Query::select_all_categories(db.inner()).await
}

#[derive(Deserialize)]
pub struct UpdateCategoryNameInput {
    pub id: i32,
    pub name: i32,
}

#[tauri::command]
pub async fn update_category_name(db: State<'_, DbConn>, data: UpdateCategoryNameInput) -> Result<(), AppError> {
    Mutation::update_category_name(db.inner(), data).await
}

#[derive(Deserialize)]
pub struct UpdateCategoryOrdinalInput {
    pub category_id: i32,
    pub new_ord: i32,
}

#[tauri::command]
pub async fn update_category_ordinal(db: State<'_, DbConn>, data: UpdateCategoryOrdinalInput) -> Result<(), AppError> {
    Mutation::update_category_ordinal(db.inner(), data).await
}

#[tauri::command]
pub async fn delete_category(db: State<'_, DbConn>, id: i32) -> Result<(), AppError> {
    Mutation::delete_category_by_id(db.inner(), id).await
}
