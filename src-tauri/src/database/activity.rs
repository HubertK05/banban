use std::collections::{HashMap, HashSet};

use anyhow::Context;
use sea_orm::DbConn;

use ::entity::{activities, activities::Entity as Activity, columns};
use sea_orm::*;
use tracing::debug;

use crate::{commands::activity::{QueryActivitiesOutput, QueryActivityOutput, UpdateActivityContentInput, UpdateActivityColumnInput}, errors::AppError};

pub struct Query;

impl Query {
    pub async fn find_activity_by_id(
        db: &DbConn,
        id: i32,
    ) -> Result<Option<activities::Model>, DbErr> {
        let tasks = Activity::find_by_id(id).one(db).await?;
        Ok(tasks)
    }

    pub async fn query_all_activities(
        db: &DbConn,
    ) -> Result<QueryActivitiesOutput, DbErr> {
        let res = Activity::find()
            .columns([activities::Column::Id, activities::Column::Name, activities::Column::Body])
            .columns([columns::Column::Id, columns::Column::Name])
            .join(JoinType::InnerJoin, activities::Relation::Columns.def()).all(db).await?;

        let res: HashMap<i32, QueryActivityOutput> = res.into_iter().map(|x| {
            (
                x.id,
                QueryActivityOutput {
                    title: x.name,
                    body: x.body,
                    category_tags: HashMap::new(),
                    other_tags: HashSet::new(),
                    column_id: x.column_id,
                },
            )
        }).collect();

        debug!("Selected activities: {res:?}");
        
        Ok(res)
    }
}

pub struct Mutation;

impl Mutation {
    pub async fn create_activity(
        db: &DbConn,
        name: String,
        body: Option<String>,
    ) -> Result<activities::Model, AppError> {
        let activity: activities::ActiveModel = activities::ActiveModel {
            name: Set(name),
            body: Set(body),
            ..Default::default()
        };
    
        let res = activity.insert(&*db).await.context("failed to insert activity")?;
        Ok(res)
    }

    pub async fn delete_activity_by_id(
        db: &DbConn,
        id: i32,
    ) -> Result<(), AppError> {
        let _ = Activity::delete_by_id(id).exec(db).await.context("failed to delete activity")?;
        Ok(())
    }

    pub async fn update_activity_content_by_id(
        db: &DbConn,
        data: UpdateActivityContentInput
    ) -> Result<(), AppError> {
        let mut record = Activity::find_by_id(data.id)
            .one(db)
            .await
            .context("failed to find the model with id")?
            .ok_or(AppError::RowNotFound)?
            .into_active_model();

        record.set(activities::Column::Name, data.title.into());
        record.set(activities::Column::Body, data.body.into());

        Activity::update(record).exec(db).await.context("failed to update record")?;
        Ok(())
    }

    pub async fn update_activity_column_by_id(
        db: &DbConn,
        data: UpdateActivityColumnInput
    ) -> Result<(), AppError> {
        let mut record = Activity::find_by_id(data.id)
            .one(db)
            .await
            .context("failed to find the model with id")?
            .ok_or(AppError::RowNotFound)?
            .into_active_model();

        record.set(activities::Column::ColumnId, data.column_id.into());

        Activity::update(record).exec(db).await.context("failed to update record")?;
        Ok(())
    }
}
