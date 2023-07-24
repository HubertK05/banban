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
#[serde(rename_all = "camelCase")]
pub struct CreateActivityInput {
    pub name: String,
    pub body: Option<String>,
    pub column_id: i32,
}

#[tauri::command]
pub async fn create_activity(
    db: State<'_, DbConn>,
    data: CreateActivityInput,
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
#[serde(rename_all = "camelCase")]
pub struct CategoryTag {
    pub category_name: String,
    pub category_ordinal: i32,
    pub tag_id: i32,
    pub tag_name: String,
    pub tag_ordinal: i32,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct QueryActivityOutput {
    pub title: String,
    pub body: Option<String>,
    pub category_tags: HashMap<i32, CategoryTag>,
    pub other_tags: Vec<String>,
    pub activity_ordinal: i32,
}

impl QueryActivityOutput {
    pub fn new_empty(title: String, body: Option<String>, activity_ordinal: i32) -> Self {
        Self {
            title,
            body,
            category_tags: HashMap::new(),
            other_tags: Vec::new(),
            activity_ordinal,
        }
    }
}

pub type QueryActivitiesOutput = HashMap<i32, QueryActivityOutput>;

#[derive(Serialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct QueryColumnOutput {
    pub name: Option<String>,
    pub column_ordinal: Option<i32>,
    pub activities: QueryActivitiesOutput,
}

impl QueryColumnOutput {
    pub fn new_empty(name: Option<String>, column_ordinal: Option<i32>) -> Self {
        Self {
            name,
            column_ordinal,
            activities: HashMap::new(),
        }
    }
}

#[derive(Serialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct QueryActivitiesWithColumnsOutput {
    pub columns: HashMap<i32, QueryColumnOutput>,
    pub other_activities: QueryColumnOutput,
}

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
#[serde(rename_all = "camelCase")]
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
#[serde(rename_all = "camelCase")]
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
#[serde(rename_all = "camelCase")]
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
#[serde(rename_all = "camelCase")]
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
