use sea_orm::DbConn;

use ::entity::{activities, activities::Entity as Activity};
use sea_orm::*;

pub struct Query;

impl Query {
    pub async fn find_activity_by_id(
        db: &DbConn,
        id: i32,
    ) -> Result<Option<activities::Model>, DbErr> {
        let tasks = Activity::find_by_id(id).one(db).await?;
        Ok(tasks)
    }
}

pub struct Mutation;

impl Mutation {
    pub async fn create_activity(
        db: &DbConn,
        form_data: activities::Model,
    ) -> Result<activities::ActiveModel, DbErr> {
        activities::ActiveModel {
            name: Set(form_data.name.to_owned()),
            body: Set(form_data.body.to_owned()),
            ..Default::default()
        }
        .save(db)
        .await
    }
}
