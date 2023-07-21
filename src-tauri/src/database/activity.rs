use std::collections::{HashMap, HashSet};

use anyhow::Context;
use sea_orm::{DbConn, sea_query::SimpleExpr};

use ::entity::{activities, activities::Entity as Activity, activity_tags, category_tags, columns};
use sea_orm::*;
use tracing::debug;

use crate::{commands::activity::{QueryActivityOutput, UpdateActivityContentInput, UpdateActivityColumnInput, QueryColumnOutput, CategoryTag, QueryActivitiesWithColumnsOutput, AddTagToActivityInput, RemoveTagFromActivityInput, CreateActivityInput}, errors::AppError};

#[derive(FromQueryResult)]
struct ActivityQueryResult {
    id: i32,
    name: String,
    body: Option<String>,
    column_id: Option<i32>,
    column_name: Option<String>,
    category_id: Option<i32>,
    category_name: Option<String>,
    tag_name: Option<String>,
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
            ])
            .column_as(columns::Column::Id, "column_id")
            .column_as(columns::Column::Name, "column_name")
            .join(JoinType::InnerJoin, activities::Relation::Columns.def())
            .join_rev(
                JoinType::LeftJoin,
                activity_tags::Relation::Activities.def(),
            )
            .join_rev(
                JoinType::LeftJoin,
                activity_tags::Relation::CategoryTags.def(),
            )
            .join(
                JoinType::LeftJoin,
                category_tags::Relation::Categories.def(),
            )
            .into_model()
            .all(db)
            .await?;

        let res: QueryActivitiesWithColumnsOutput =
            res.into_iter().fold(HashMap::new(), |mut acc, x| {
                let column_entry = acc.entry(x.column_id).or_insert(QueryColumnOutput {
                    name: x.column_name,
                    activities: HashMap::new(),
                });

                let activity_entry =
                    column_entry
                        .activities
                        .entry(x.id)
                        .or_insert(QueryActivityOutput {
                            title: x.name,
                            body: x.body,
                            category_tags: HashMap::new(),
                            other_tags: HashSet::new(),
                        });

                if let Some(tag_name) = x.tag_name {
                    if let (Some(category_id), Some(category_name)) =
                        (x.category_id, x.category_name)
                    {
                        activity_entry.category_tags.insert(
                            category_id,
                            CategoryTag {
                                category_name,
                                tag_name,
                            },
                        );
                    } else {
                        activity_entry.other_tags.insert(tag_name);
                    };
                }

                acc
            });

        debug!("Selected activities: {res:?}");

        Ok(res)
    }

    async fn get_ordinal_from_id(db: &DbConn, activity_id: i32) -> Result<i32, AppError> {
        let res = activities::Entity::find_by_id(activity_id)
            .one(db).await.context("failed to get ordinal from id")?
            .ok_or(AppError::RowNotFound)?;
        Ok(res.ordinal)
    }

    async fn get_activity_count_by_column(db: &DbConn, column_id: Option<i32>) -> Result<i32, AppError> {
        let res: Option<i32> = activities::Entity::find()
            .select_only()
            .filter(activities::Column::ColumnId.eq(column_id))
            .column_as(activities::Column::Id.count(), "count").into_tuple().one(db).await.context("failed to determine count of activities")?;
        Ok(res.expect("where is the count???"))
    }

    async fn get_column_id_from_activity_id(db: &DbConn, id: i32) -> Result<Option<i32>, AppError> {
        let res = activities::Entity::find_by_id(id)
            .one(db).await.context("failed to get column_id")?
            .ok_or(AppError::RowNotFound)?;

        Ok(res.column_id)
    }
}

pub struct Mutation;

impl Mutation {
    pub async fn create_activity(
        db: &DbConn,
        data: CreateActivityInput
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
        let res = activity.insert(&tr).await.context("failed to insert activity")?;
        tr.commit().await.context("failed to commit transaction")?;
    
        Ok(res)
    }

    pub async fn delete_activity_by_id(
        db: &DbConn,
        id: i32,
    ) -> Result<(), AppError> {
        let deleted_ord = Query::get_ordinal_from_id(db, id).await?;
        let column_id = Query::get_column_id_from_activity_id(db, id).await?;
        let tr = db.begin().await.context("failed to begin transaction")?;
        let _ = Activity::delete_by_id(id).exec(&tr).await.context("failed to delete activity")?;
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
        let mut record = Activity::find_by_id(data.id)
            .one(db)
            .await
            .context("failed to find the model with id")?
            .ok_or(AppError::RowNotFound)?
            .into_active_model();

        record.set(activities::Column::ColumnId, data.column_id.into());
        let tr = db.begin().await.context("failed to begin transaction")?;
        Self::left_shift_ordinals(&tr, old_ord, old_column_id).await?;
        Self::right_shift_ordinals(&tr, data.new_ord, data.column_id).await?;
        
        Activity::update(record).exec(&tr).await.context("failed to update record")?;
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
            .exec(db).await.context("failed to delete activity_tag")?;

        Ok(())
    }

    async fn left_shift_ordinals(db: &impl ConnectionTrait, start_ord: i32, column_id: Option<i32>) -> Result<(), AppError> {
        activities::Entity::update_many()
            .filter(activities::Column::Ordinal.gt(start_ord))
            .filter(activities::Column::ColumnId.eq(column_id))
            .col_expr(activities::Column::Ordinal, SimpleExpr::from(activities::Column::Ordinal.into_expr()).sub(SimpleExpr::Value(Value::Int(Some(1)))))
            .exec(db).await.context("failed to left shift ordinals")?;
        Ok(())
    }

    async fn right_shift_ordinals(db: &impl ConnectionTrait, start_ord: i32, column_id: Option<i32>) -> Result<(), AppError> {
        activities::Entity::update_many()
            .filter(activities::Column::Ordinal.gte(start_ord))
            .filter(activities::Column::ColumnId.eq(column_id))
            .col_expr(activities::Column::Ordinal, SimpleExpr::from(activities::Column::Ordinal.into_expr()).add(SimpleExpr::Value(Value::Int(Some(1)))))
            .exec(db).await.context("failed to right shift ordinals")?;
        Ok(())
    }
}
