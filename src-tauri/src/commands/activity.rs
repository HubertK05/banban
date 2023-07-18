use entity::activity;

use sea_orm::{DbConn, Set, ActiveModelTrait};
use tauri::State;

#[tauri::command]
pub async fn create_activity(db: State<'_, DbConn>, title: String, body: String) -> Result<(), ()> {
    let activity = activity::ActiveModel {
        title: Set(title),
        body: Set(body),
        column_id: Set(None),
        ..Default::default()
    }
    .save(&*db)
    .await.unwrap();
    Ok(())
}