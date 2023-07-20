use std::collections::{HashMap, HashSet};

use anyhow::Context;
use entity::activities;
use sea_orm::DbConn;
use serde::{Serialize, Deserialize};
use tauri::State;

use crate::{database::activity::{Mutation, Query}, errors::AppError};

#[tauri::command]
pub async fn create_activity(
    db: State<'_, DbConn>,
    name: String,
    body: Option<String>,
) -> Result<activities::Model, AppError> {
    let model = Mutation::create_activity(db.inner(), name, body).await?;
    Ok(model)
}

#[tauri::command]
pub async fn delete_activity<'a>(db: State<'a, DbConn>, id: i32) -> Result<(), AppError> {
    Mutation::delete_activity_by_id(db.inner(), id).await.with_context(|| {"failed to delete activity"})?;
    Ok(())
}

#[derive(Serialize, Debug)]
pub struct CategoryTag {
    category_name: String,
    tag_name: String,
}

#[derive(Serialize, Debug)]
pub struct QueryActivityOutput {
    pub title: String,
    pub body: Option<String>,
    pub category_tags: HashMap<i32, CategoryTag>,
    pub other_tags: HashSet<String>,
    pub column_id: Option<i32>,
    pub column_name: Option<String>,
}

pub type QueryActivitiesOutput = HashMap<i32, QueryActivityOutput>;

#[tauri::command]
pub async fn query_all_activities(db: State<'_, DbConn>) -> Result<QueryActivitiesOutput, AppError> {
    let res = Query::query_all_activities(db.inner()).await.context("failed to select all activities")?;
    Ok(res)
}

#[derive(Deserialize)]
pub struct UpdateActivityContentInput {
    pub id: i32,
    pub title: String,
    pub body: Option<String>,
}

#[tauri::command]
pub async fn update_activity_content(db: State<'_, DbConn>, data: UpdateActivityContentInput) -> Result<(), AppError> {
    Mutation::update_activity_content_by_id(db.inner(), data).await.context("failed to update activity content")?;
    Ok(())
}

#[derive(Deserialize)]
pub struct UpdateActivityColumnInput {
    pub id: i32,
    pub column_id: Option<i32>,
}

#[tauri::command]
pub async fn update_activity_column(db: State<'_, DbConn>, data: UpdateActivityColumnInput) -> Result<(), AppError> {
    Mutation::update_activity_column_by_id(&db, data).await.context("failed to update activity column")?;
    Ok(())
}
