use std::collections::{HashMap, HashSet};

use anyhow::Context;
use sea_orm::{sea_query::SimpleExpr, DbConn};

use ::entity::{
    activities, activities::Entity as Activity, activity_tags, categories, category_tags, columns,
};
use sea_orm::*;
use tracing::debug;

use crate::{
    commands::activity::{
        AddTagToActivityInput, CategoryTag, CreateActivityInput, QueryActivitiesWithColumnsOutput,
        QueryActivityOutput, QueryColumnOutput, RemoveTagFromActivityInput,
        UpdateActivityColumnInput, UpdateActivityContentInput,
    },
    errors::AppError,
    utils::coloring::{int_to_rgb, rgb_int_to_string},
};

#[derive(FromQueryResult)]
struct ActivityQueryResult {
    id: i32,
    name: String,
    body: Option<String>,
    ordinal: i32,
    column_id: Option<i32>,
    column_name: Option<String>,
    column_ordinal: Option<i32>,
    category_id: Option<i32>,
    category_name: Option<String>,
    category_ordinal: Option<i32>,
    tag_id: Option<i32>,
    tag_name: Option<String>,
    tag_ordinal: Option<i32>,
    tag_color: Option<i32>,
}

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
    ) -> Result<QueryActivitiesWithColumnsOutput, DbErr> {
        let res: Vec<ActivityQueryResult> = Activity::find()
            .select_only()
            .columns([
                activities::Column::Id,
                activities::Column::Name,
                activities::Column::Body,
                activities::Column::Ordinal,
            ])
            .column_as(columns::Column::Id, "column_id")
            .column_as(columns::Column::Name, "column_name")
            .column_as(columns::Column::Ordinal, "column_ordinal")
            .column_as(category_tags::Column::Id, "tag_id")
            .column_as(category_tags::Column::TagName, "tag_name")
            .column_as(category_tags::Column::Ordinal, "tag_ordinal")
            .column_as(category_tags::Column::Color, "tag_color")
            .column_as(categories::Column::Id, "category_id")
            .column_as(categories::Column::Ordinal, "category_ordinal")
            .column_as(categories::Column::Name, "category_name")
            .join(JoinType::InnerJoin, activities::Relation::Columns.def())
            .join_rev(
                JoinType::LeftJoin,
                activity_tags::Relation::Activities.def(),
            )
            .join(
                JoinType::LeftJoin,
                activity_tags::Relation::CategoryTags.def(),
            )
            .join(
                JoinType::LeftJoin,
                category_tags::Relation::Categories.def(),
            )
            .order_by(columns::Column::Ordinal, sea_orm::Order::Asc)
            .order_by(activities::Column::Ordinal, sea_orm::Order::Asc)
            .order_by(categories::Column::Ordinal, sea_orm::Order::Asc)
            .order_by(category_tags::Column::Ordinal, sea_orm::Order::Asc)
            .into_model()
            .all(db)
            .await?;

        let res: QueryActivitiesWithColumnsOutput = res.into_iter().fold(
            QueryActivitiesWithColumnsOutput::default(),
            |mut acc, record| {
                let activities = if let Some(column_id) = record.column_id {
                    &mut acc
                        .columns
                        .entry(column_id)
                        .or_insert(QueryColumnOutput::new_empty(
                            record.column_name,
                            record.column_ordinal,
                        ))
                        .activities
                } else {
                    &mut acc.other_activities.activities
                };

                let activity_entry =
                    activities
                        .entry(record.id)
                        .or_insert(QueryActivityOutput::new_empty(
                            record.name,
                            record.body,
                            record.ordinal,
                        ));

                if let Some(tag_id) = record.tag_id {
                    if let Some(category_id) = record.category_id {
                        activity_entry.category_tags.insert(
                            category_id,
                            CategoryTag {
                                tag_id,
                                category_name: record
                                    .category_name
                                    .expect("No category name found for category"),
                                tag_name: record.tag_name.expect("No tag name found for tag"),
                                category_ordinal: record
                                    .category_ordinal
                                    .expect("No category ordinal found for category"),
                                tag_ordinal: record
                                    .tag_ordinal
                                    .expect("No tag ordinal found for category"),
                                tag_color: rgb_int_to_string(
                                    record.tag_color.expect("No tag color found for the tag"),
                                ),
                            },
                        );
                    } else {
                        activity_entry
                            .other_tags
                            .push(record.tag_name.expect("No tag name found for tag"));
                    };
                }

                acc
            },
        );

        Ok(res)
    }

    async fn get_ordinal_from_id(db: &DbConn, activity_id: i32) -> Result<i32, AppError> {
        let res = activities::Entity::find_by_id(activity_id)
            .one(db)
            .await
            .context("failed to get ordinal from id")?
            .ok_or(AppError::RowNotFound)?;
        Ok(res.ordinal)
    }

    async fn get_activity_count_by_column(
        db: &DbConn,
        column_id: Option<i32>,
    ) -> Result<i32, AppError> {
        let res = activities::Entity::find()
            .filter(activities::Column::ColumnId.eq(column_id))
            .count(db)
            .await
            .context("failed to determine count of columns")?;
        Ok(res as i32)
    }

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
            .filter(category_tags::Column::CategoryId.eq(data.category_id))
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
            .filter(category_tags::Column::CategoryId.eq(data.category_id))
            .filter(category_tags::Column::TagName.eq(data.tag_name))
            .one(db)
            .await
            .context("failed to select category tag id")?
            .ok_or(AppError::RowNotFound)?
            .id;

        activity_tags::Entity::delete_many()
            .filter(activity_tags::Column::CategoryTagId.eq(category_tag_id))
            .exec(db)
            .await
            .context("failed to delete activity_tag")?;

        Ok(())
    }

    async fn left_shift_ordinals(
        db: &impl ConnectionTrait,
        start_ord: i32,
        column_id: Option<i32>,
    ) -> Result<(), AppError> {
        activities::Entity::update_many()
            .filter(activities::Column::Ordinal.gt(start_ord))
            .filter(activities::Column::ColumnId.eq(column_id))
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

    async fn right_shift_ordinals(
        db: &impl ConnectionTrait,
        start_ord: i32,
        column_id: Option<i32>,
    ) -> Result<(), AppError> {
        activities::Entity::update_many()
            .filter(activities::Column::Ordinal.gte(start_ord))
            .filter(activities::Column::ColumnId.eq(column_id))
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
