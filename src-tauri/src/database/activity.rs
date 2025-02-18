use std::collections::HashMap;

use anyhow::Context;
use sea_orm::{sea_query::SimpleExpr, DbConn};

use ::entity::{
    activities,
    activities::{Entity as Activity, Model},
    activity_tags, categories, category_tags, columns,
};
use sea_orm::*;

use crate::{
    commands::{
        activity::{
            AddTagToActivityInput, CreateActivityInput, RemoveTagFromActivityInput,
            UpdateActivityColumnInput, UpdateActivityContentInput,
        },
        fetch::{ActivityOutput, ColumnActivityOutput},
    },
    errors::AppError,
};

pub struct Query;

impl Query {
    /// Fetches an activity from the db given its id.
    ///
    /// Returns `Ok(None)` if the activity is not found.
    pub async fn find_activity_by_id(
        db: &DbConn,
        id: i32,
    ) -> Result<Option<activities::Model>, DbErr> {
        let tasks = Activity::find_by_id(id).one(db).await?;
        Ok(tasks)
    }

    /// Fetches all activities from all columns that have column ids (all activities excluding those from the stash).
    ///
    /// Returns all found activities associated with their ids and with the id of the column that contains it.
    pub async fn all_column_activities(
        db: &DbConn,
    ) -> Result<HashMap<i32, ColumnActivityOutput>, DbErr> {
        let res = Activity::find()
            .find_with_related(category_tags::Entity)
            .filter(Condition::any().add(activities::Column::ColumnId.is_not_null()))
            .order_by_asc(activities::Column::Ordinal)
            .all(db)
            .await?;

        let out = res
            .into_iter()
            .fold(HashMap::new(), |mut acc, (activity, tags)| {
                acc.insert(
                    activity.id,
                    ColumnActivityOutput {
                        name: activity.name,
                        body: activity.body,
                        ordinal: activity.ordinal,
                        tags: tags.into_iter().map(|tag| tag.id).collect(),
                        column_id: activity.column_id.unwrap(),
                    },
                );
                acc
            });
        Ok(out)
    }

    /// Fetches all activities outside the columns that have their ids (from the stash).
    ///
    /// Returns all stash activities associated with their ids.
    pub async fn all_other_activities(db: &DbConn) -> Result<HashMap<i32, ActivityOutput>, DbErr> {
        let res = Activity::find()
            .find_with_related(category_tags::Entity)
            .filter(Condition::any().add(activities::Column::ColumnId.is_null()))
            .order_by_asc(activities::Column::Ordinal)
            .all(db)
            .await?;

        let out = res
            .into_iter()
            .fold(HashMap::new(), |mut acc, (activity, tags)| {
                acc.insert(
                    activity.id,
                    ActivityOutput {
                        name: activity.name,
                        body: activity.body,
                        ordinal: activity.ordinal,
                        tags: tags.into_iter().map(|tag| tag.id).collect(),
                    },
                );
                acc
            });
        Ok(out)
    }

    /// Fetches the ordinal of the activity that has a given id.
    ///
    /// Returns a `RowNotFound` error if the activity with a given id is not found.
    async fn get_ordinal_from_id(db: &DbConn, activity_id: i32) -> Result<i32, AppError> {
        let res = activities::Entity::find_by_id(activity_id)
            .one(db)
            .await
            .context("failed to get ordinal from id")?
            .ok_or(AppError::RowNotFound)?;
        Ok(res.ordinal)
    }

    /// Fetches the column id of the activity that has a given id.
    ///
    /// Returns a `RowNotFound` error if the activity with a given id is not found and
    /// `Ok(None)` if the activity is in the stash.
    async fn get_column_id_from_activity_id(db: &DbConn, id: i32) -> Result<Option<i32>, AppError> {
        let res = activities::Entity::find_by_id(id)
            .one(db)
            .await
            .context("failed to get column_id")?
            .ok_or(AppError::RowNotFound)?;

        Ok(res.column_id)
    }
}

pub struct Mutation;

impl Mutation {
    /// Creates an activity, and returns that activity with its newly created id.
    ///
    /// This also updates ordinals of the activities in a given column to maintain valid ordering.
    pub async fn create_activity(
        db: &DbConn,
        data: CreateActivityInput,
    ) -> Result<activities::Model, AppError> {
        let activity: activities::ActiveModel = activities::ActiveModel {
            name: Set(data.name),
            body: Set(data.body),
            ordinal: Set(0),
            column_id: Set(Some(data.column_id)),
            ..Default::default()
        };

        let tr = db.begin().await.context("failed to begin transaction")?;
        Self::right_shift_ordinals(db, 0, Some(data.column_id)).await?;
        let res = activity
            .insert(&tr)
            .await
            .context("failed to insert activity")?;
        tr.commit().await.context("failed to commit transaction")?;

        Ok(res)
    }

    /// Deletes an activity, given its id.
    ///
    /// This also updates ordinals of the activities in a given column to maintain valid ordering.
    pub async fn delete_activity_by_id(db: &DbConn, id: i32) -> Result<(), AppError> {
        let deleted_ord = Query::get_ordinal_from_id(db, id).await?;
        let column_id = Query::get_column_id_from_activity_id(db, id).await?;
        let tr = db.begin().await.context("failed to begin transaction")?;
        let _ = Activity::delete_by_id(id)
            .exec(&tr)
            .await
            .context("failed to delete activity")?;
        Self::left_shift_ordinals(&tr, deleted_ord, column_id).await?;
        tr.commit().await.context("failed to commit transaction")?;
        Ok(())
    }

    /// Updates activity content (like name or body).
    ///
    /// Returns `Err(RowNotFound)` if no activity with id given in `data` is found.
    pub async fn update_activity_content_by_id(
        db: &DbConn,
        data: UpdateActivityContentInput,
    ) -> Result<(), AppError> {
        let mut record = Activity::find_by_id(data.id)
            .one(db)
            .await
            .context("failed to find the model with id")?
            .ok_or(AppError::RowNotFound)?
            .into_active_model();

        record.set(activities::Column::Name, data.name.into());
        record.set(activities::Column::Body, data.body.into());

        Activity::update(record)
            .exec(db)
            .await
            .context("failed to update record")?;
        Ok(())
    }

    /// Updates the position (ordinal) of an activity based on its id.
    ///
    /// This changes the ordinal of a given activity and changes ordinals of activities occuring later to match the new position of the activity.
    pub async fn update_activity_column_by_id(
        db: &DbConn,
        data: UpdateActivityColumnInput,
    ) -> Result<(), AppError> {
        let old_column_id = Query::get_column_id_from_activity_id(db, data.id).await?;
        let old_ord = Query::get_ordinal_from_id(db, data.id).await?;

        let tr = db.begin().await.context("failed to begin transaction")?;
        Self::left_shift_ordinals(&tr, old_ord, old_column_id).await?;
        Self::right_shift_ordinals(&tr, data.new_ord, data.column_id).await?;

        let mut record = Activity::find_by_id(data.id)
            .one(&tr)
            .await
            .context("failed to find the model with id")?
            .ok_or(AppError::RowNotFound)?
            .into_active_model();
        record.set(activities::Column::Ordinal, data.new_ord.into());
        record.set(activities::Column::ColumnId, data.column_id.into());

        Activity::update(record)
            .exec(&tr)
            .await
            .context("failed to update record")?;

        tr.commit().await.context("failed to commit transaction")?;

        Ok(())
    }

    pub async fn add_tag_to_activity(
        db: &DbConn,
        data: AddTagToActivityInput,
    ) -> Result<(), AppError> {
        let category_tag_id = category_tags::Entity::find()
            .filter(
                Condition::any()
                    .add(category_tags::Column::CategoryId.eq(data.category_id))
                    .add(
                        category_tags::Column::CategoryId
                            .is_null()
                            .and(SimpleExpr::from(data.category_id.is_none())),
                    ),
            )
            .filter(category_tags::Column::TagName.eq(data.tag_name))
            .one(db)
            .await
            .context("failed to select category tag id")?
            .ok_or(AppError::RowNotFound)?
            .id;

        let model = activity_tags::ActiveModel {
            activity_id: Set(data.id),
            category_tag_id: Set(category_tag_id),
        };

        model
            .insert(db)
            .await
            .context("failed to insert activity_tag")?;
        Ok(())
    }

    pub async fn remove_tag_from_activity(
        db: &DbConn,
        data: RemoveTagFromActivityInput,
    ) -> Result<(), AppError> {
        let category_tag_id = category_tags::Entity::find()
            .filter(
                Condition::any()
                    .add(category_tags::Column::CategoryId.eq(data.category_id))
                    .add(
                        category_tags::Column::CategoryId
                            .is_null()
                            .and(SimpleExpr::from(data.category_id.is_none())),
                    ),
            )
            .filter(category_tags::Column::TagName.eq(data.tag_name))
            .one(db)
            .await
            .context("failed to select category tag id")?
            .ok_or(AppError::RowNotFound)?
            .id;

        activity_tags::Entity::delete_many()
            .filter(activity_tags::Column::CategoryTagId.eq(category_tag_id))
            .filter(activity_tags::Column::ActivityId.eq(data.id))
            .exec(db)
            .await
            .context("failed to delete activity_tag")?;

        Ok(())
    }

    /// Helper function that subtracts 1 from ordinals of activities starting from the `start_ord` on a given column.
    async fn left_shift_ordinals(
        db: &impl ConnectionTrait,
        start_ord: i32,
        column_id: Option<i32>,
    ) -> Result<(), AppError> {
        activities::Entity::update_many()
            .filter(activities::Column::Ordinal.gt(start_ord))
            .filter(
                Condition::any()
                    .add(activities::Column::ColumnId.eq(column_id))
                    .add(
                        activities::Column::ColumnId
                            .is_null()
                            .and(SimpleExpr::from(column_id.is_none())),
                    ),
            )
            .col_expr(
                activities::Column::Ordinal,
                SimpleExpr::from(activities::Column::Ordinal.into_expr())
                    .sub(SimpleExpr::Value(Value::Int(Some(1)))),
            )
            .exec(db)
            .await
            .context("failed to left shift ordinals")?;
        Ok(())
    }

    /// Helper function that adds 1 to ordinals of activities starting from the `start_ord` on a given column.
    async fn right_shift_ordinals(
        db: &impl ConnectionTrait,
        start_ord: i32,
        column_id: Option<i32>,
    ) -> Result<(), AppError> {
        activities::Entity::update_many()
            .filter(activities::Column::Ordinal.gte(start_ord))
            .filter(
                Condition::any()
                    .add(activities::Column::ColumnId.eq(column_id))
                    .add(
                        activities::Column::ColumnId
                            .is_null()
                            .and(SimpleExpr::from(column_id.is_none())),
                    ),
            )
            .col_expr(
                activities::Column::Ordinal,
                SimpleExpr::from(activities::Column::Ordinal.into_expr())
                    .add(SimpleExpr::Value(Value::Int(Some(1)))),
            )
            .exec(db)
            .await
            .context("failed to right shift ordinals")?;
        Ok(())
    }
}
