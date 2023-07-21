use std::collections::{HashMap, HashSet};

use anyhow::Context;
use entity::activities;
use sea_orm::DbConn;
use serde::{Deserialize, Serialize};
use tauri::State;

use crate::{
    database::activity::{Mutation, Query},
    errors::AppError,
};

#[derive(Deserialize)]
pub struct CreateActivityInput {
    pub name: String,
    pub body: Option<String>,
    pub column_id: i32,
}

#[tauri::command]
pub async fn create_activity(
    db: State<'_, DbConn>,
    data: CreateActivityInput
) -> Result<activities::Model, AppError> {
    let model = Mutation::create_activity(db.inner(), data).await?;
    Ok(model)
}

#[tauri::command]
pub async fn delete_activity<'a>(db: State<'a, DbConn>, id: i32) -> Result<(), AppError> {
    Mutation::delete_activity_by_id(db.inner(), id)
        .await
        .with_context(|| "failed to delete activity")?;
    Ok(())
}

#[derive(Serialize, Debug)]
pub struct CategoryTag {
    pub category_name: String,
    pub tag_name: String,
}

#[derive(Serialize, Debug)]
pub struct QueryActivityOutput {
    pub title: String,
    pub body: Option<String>,
    pub category_tags: HashMap<i32, CategoryTag>,
    pub other_tags: HashSet<String>,
}

pub type QueryActivitiesOutput = HashMap<i32, QueryActivityOutput>;

#[derive(Serialize, Debug)]
pub struct QueryColumnOutput {
    pub name: Option<String>,
    pub activities: QueryActivitiesOutput,
}

pub type QueryActivitiesWithColumnsOutput = HashMap<Option<i32>, QueryColumnOutput>;

#[tauri::command]
pub async fn query_all_activities(
    db: State<'_, DbConn>,
) -> Result<QueryActivitiesWithColumnsOutput, AppError> {
    let res = Query::query_all_activities(db.inner())
        .await
        .context("failed to select all activities")?;
    Ok(res)
}

#[derive(Deserialize)]
pub struct UpdateActivityContentInput {
    pub id: i32,
    pub name: String,
    pub body: Option<String>,
}

#[tauri::command]
pub async fn update_activity_content(
    db: State<'_, DbConn>,
    data: UpdateActivityContentInput,
) -> Result<(), AppError> {
    Mutation::update_activity_content_by_id(db.inner(), data)
        .await
        .context("failed to update activity content")?;
    Ok(())
}

#[derive(Deserialize)]
pub struct UpdateActivityColumnInput {
    pub id: i32,
    pub column_id: Option<i32>,
    pub new_ord: i32,
}

#[tauri::command]
pub async fn update_activity_column(
    db: State<'_, DbConn>,
    data: UpdateActivityColumnInput,
) -> Result<(), AppError> {
    Mutation::update_activity_column_by_id(&db, data)
        .await
        .context("failed to update activity column")?;
    Ok(())
}

#[derive(Deserialize)]
pub struct AddTagToActivityInput {
    pub id: i32,
    pub category_id: Option<i32>,
    pub tag_name: String,
}

#[tauri::command]
pub async fn add_tag_to_activity(
    db: State<'_, DbConn>,
    data: AddTagToActivityInput,
) -> Result<(), AppError> {
    Mutation::add_tag_to_activity(db.inner(), data).await
}

#[derive(Deserialize)]
pub struct RemoveTagFromActivityInput {
    pub id: i32,
    pub category_id: Option<i32>,
    pub tag_name: String,
}

#[tauri::command]
pub async fn remove_tag_from_activity(
    db: State<'_, DbConn>,
    data: RemoveTagFromActivityInput,
) -> Result<(), AppError> {
    Mutation::remove_tag_from_activity(db.inner(), data).await
}
