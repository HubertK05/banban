use sea_orm::DbConn;
use serde::Deserialize;
use tauri::State;

use crate::{
    database::{
        activity, category, category_tags,
        columns::{self, Mutation},
    },
    errors::AppError,
};

struct Output {}

#[tauri::command]
pub async fn fetch_all(db: State<'_, DbConn>, name: String) -> Result<(), AppError> {
    let activities = activity::Query::get_all_activities(&db).await?;
    let columns = columns::Query::get_all_columns(&db).await?;
    let tag_categories = category::Query::get_all_categories(&db).await?;
    let category_tags = category_tags::Query::get_all_tags(&db).await?;

    Ok(())
}
