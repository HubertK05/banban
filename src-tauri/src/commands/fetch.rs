use std::collections::HashMap;

use sea_orm::DbConn;
use serde::{Deserialize, Serialize};
use tauri::State;

use crate::{
    database::{
        activity, category,
        columns::{self, Mutation},
        tags,
    },
    errors::AppError,
};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ColumnOutput {
    pub name: String,
    pub ordinal: i32,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CategoryTagOutput {
    pub name: String,
    pub color: String,
    pub category_id: i32,
    pub ordinal: i32,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OtherTagOutput {
    pub name: String,
    pub color: String,
    pub ordinal: i32,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CategoryOutput {
    pub name: String,
    pub ordinal: i32,
    pub tags: Vec<i32>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivityOutput {
    pub name: String,
    pub body: Option<String>,
    pub ordinal: i32,
    pub tags: Vec<i32>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ColumnActivityOutput {
    pub name: String,
    pub body: Option<String>,
    pub ordinal: i32,
    pub tags: Vec<i32>,
    pub column_id: i32,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FetchOutput {
    columns: HashMap<i32, ColumnOutput>,
    activities: HashMap<i32, ColumnActivityOutput>,
    other_activities: HashMap<i32, ActivityOutput>,
    categories: HashMap<i32, CategoryOutput>,
    category_tags: HashMap<i32, CategoryTagOutput>,
    other_tags: HashMap<i32, OtherTagOutput>,
}

#[tauri::command]
pub async fn fetch_all(db: State<'_, DbConn>) -> Result<FetchOutput, AppError> {
    let activities = activity::Query::all_column_activities(&db).await?;
    let other_activities = activity::Query::all_other_activities(&db).await?;
    let columns = columns::Query::all_columns(&db).await?;
    let (categories, category_tags) = category::Query::all_with_category_tags(&db).await?;
    let other_tags = tags::Query::all_other_tags(&db).await?;

    Ok(FetchOutput {
        columns,
        activities,
        other_activities,
        categories,
        category_tags,
        other_tags,
    })
}
