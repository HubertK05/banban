use std::collections::HashMap;

use crate::commands::fetch::OtherTagOutput;
use crate::commands::tags::{UpdateTagColorInput, UpdateTagOrdinalInput};
use crate::utils::coloring::{rgb_int_to_string, rgb_string_to_int, string_to_color};
use crate::{
    commands::tags::{CreateTagInput, UpdateTagNameInput},
    errors::AppError,
};
use anyhow::Context;
use entity::category_tags::{self, Entity as CategoryTag};
use sea_orm::{
    sea_query::SimpleExpr, ActiveModelTrait, ColumnTrait, ConnectionTrait, DbConn, EntityTrait,
    IntoActiveModel, PaginatorTrait, QueryFilter, Set, TransactionTrait, Value,
};
use sea_orm::{Condition, DbErr};

pub struct Query;

impl Query {
    /// Fetches all non-category tags - tags that can be chosen independentely of each other to the activity - with their ids.
    ///
    /// Used to get non-category tags on application startup.
    pub async fn all_other_tags(db: &DbConn) -> Result<HashMap<i32, OtherTagOutput>, DbErr> {
        let res = CategoryTag::find()
            .filter(Condition::any().add(category_tags::Column::CategoryId.is_null()))
            .all(db)
            .await?;

        let out: HashMap<i32, OtherTagOutput> =
            res.into_iter().fold(HashMap::new(), |mut acc, tag| {
                acc.insert(
                    tag.id,
                    OtherTagOutput {
                        name: tag.tag_name,
                        color: rgb_int_to_string(tag.color),
                        ordinal: tag.ordinal,
                    },
                );
                acc
            });
        Ok(out)
    }

    /// Helper function to get ordinal of the tag with a given id.
    ///
    /// Returns `Err(RowNotFound)` if the tag does not exist.
    async fn get_ordinal_from_id(db: &DbConn, category_tag_id: i32) -> Result<i32, AppError> {
        let res = category_tags::Entity::find_by_id(category_tag_id)
            .one(db)
            .await
            .context("failed to get ordinal from id")?
            .ok_or(AppError::RowNotFound)?;
        Ok(res.ordinal)
    }

    /// Helper function to get the tag amount in a given category.
    ///
    /// If `category_id` is `None`, then returns the non-category tag amount.
    ///
    /// Returns 0 if category with a given id does not exist.
    async fn get_tag_count_from_category(
        db: &DbConn,
        category_id: Option<i32>,
    ) -> Result<i32, AppError> {
        let res = category_tags::Entity::find()
            .filter(
                Condition::any()
                    .add(category_tags::Column::CategoryId.eq(category_id))
                    .add(
                        category_tags::Column::CategoryId
                            .is_null()
                            .and(SimpleExpr::from(category_id.is_none())),
                    ),
            )
            .count(db)
            .await
            .context("failed to determine count of columns")?;
        Ok(res as i32)
    }

    /// Helper function used to fetch category id from tag id.
    ///
    /// Returns `Ok(None)` if tag with the given `id` is a non-category tag.
    async fn get_category_id_from_record_id(db: &DbConn, id: i32) -> Result<Option<i32>, AppError> {
        let res = category_tags::Entity::find_by_id(id)
            .one(db)
            .await
            .context("failed to get category_id")?
            .ok_or(AppError::RowNotFound)?;

        Ok(res.category_id)
    }
}

pub struct Mutation;

impl Mutation {
    /// Creates a new tag and appends it to the end of the list.
    ///
    /// Returns a tag with the given name, generated color, id and ordinal.
    pub async fn create_tag(
        db: &DbConn,
        data: CreateTagInput,
    ) -> Result<category_tags::Model, AppError> {
        let tag_count = Query::get_tag_count_from_category(db, data.category_id).await?;
        let tag_model = category_tags::ActiveModel {
            color: Set(string_to_color(&data.tag_name)),
            tag_name: Set(data.tag_name),
            category_id: Set(data.category_id),
            ordinal: Set(tag_count),
            ..Default::default()
        };

        let res = tag_model.insert(db).await.context("failed to insert tag")?;
        Ok(res)
    }

    /// Renames the tag with id given in `data`.
    ///
    /// Returns `Err(RowNotFound)` if the tag with a given id does not exist.
    pub async fn update_tag_name(db: &DbConn, data: UpdateTagNameInput) -> Result<(), AppError> {
        let mut tag_model = category_tags::Entity::find_by_id(data.category_tag_id)
            .one(db)
            .await
            .context("failed to get category_tags model")?
            .ok_or(AppError::RowNotFound)?
            .into_active_model();

        tag_model.tag_name = Set(data.tag_name);
        category_tags::Entity::update(tag_model)
            .exec(db)
            .await
            .context("failed to update category_tags row")?;
        Ok(())
    }

    /// Updates the position of tag with id given in `data`.
    ///
    /// This also shifts ordinals of other tags to achieve correct ordering.
    ///
    /// Returns `Err(RowNotFound)` if the tag with a given id does not exist.
    pub async fn update_tag_ordinal(
        db: &DbConn,
        data: UpdateTagOrdinalInput,
    ) -> Result<(), AppError> {
        let category_id = Query::get_category_id_from_record_id(db, data.category_tag_id).await?;
        let old_ord = Query::get_ordinal_from_id(db, data.category_tag_id).await?;
        let mut model = category_tags::Entity::find_by_id(data.category_tag_id)
            .one(db)
            .await
            .context("failed to select column")?
            .ok_or(AppError::RowNotFound)?
            .into_active_model();

        let tr = db.begin().await.context("failed to begin transaction")?;
        Self::left_shift_ordinals(&tr, old_ord, category_id).await?;
        Self::right_shift_ordinals(&tr, data.new_ord, category_id).await?;

        model.ordinal = Set(data.new_ord);
        model.update(&tr).await.context("failed to update column")?;
        tr.commit().await.context("failed to commit transaction")?;
        Ok(())
    }

    /// Updates tag color.
    ///
    /// Returns `Err(RowNotFound)` if tag with id given in `data` does not exist.
    pub async fn update_tag_color(db: &DbConn, data: UpdateTagColorInput) -> Result<(), AppError> {
        let mut tag_model = category_tags::Entity::find_by_id(data.category_tag_id)
            .one(db)
            .await
            .context("failed to get category_tags model")?
            .ok_or(AppError::RowNotFound)?
            .into_active_model();

        tag_model.color = Set(rgb_string_to_int(&data.color)?);
        category_tags::Entity::update(tag_model)
            .exec(db)
            .await
            .context("failed to update category_tags row")?;
        Ok(())
    }

    /// Deletes tag with a given id.
    ///
    /// This also shifts ordinals to maintain correct tag order.
    pub async fn delete_tag_by_id(db: &DbConn, id: i32) -> Result<(), AppError> {
        let category_id = Query::get_category_id_from_record_id(db, id).await?;
        let deleted_ord = Query::get_ordinal_from_id(db, id).await?;
        let tr = db.begin().await.context("failed to begin transaction")?;
        category_tags::Entity::delete_by_id(id)
            .exec(&tr)
            .await
            .context("failed to delete category_tag")?;
        Self::left_shift_ordinals(&tr, deleted_ord, category_id).await?;
        tr.commit().await.context("failed to commit transaction")?;
        Ok(())
    }

    /// Helper function that decrements ordinals greater than `start_ord`.
    async fn left_shift_ordinals(
        db: &impl ConnectionTrait,
        start_ord: i32,
        category_id: Option<i32>,
    ) -> Result<(), AppError> {
        category_tags::Entity::update_many()
            .filter(category_tags::Column::Ordinal.gt(start_ord))
            .filter(
                Condition::any()
                    .add(category_tags::Column::CategoryId.eq(category_id))
                    .add(
                        category_tags::Column::CategoryId
                            .is_null()
                            .and(SimpleExpr::from(category_id.is_none())),
                    ),
            )
            .col_expr(
                category_tags::Column::Ordinal,
                SimpleExpr::from(category_tags::Column::Ordinal.into_expr())
                    .sub(SimpleExpr::Value(Value::Int(Some(1)))),
            )
            .exec(db)
            .await
            .context("failed to left shift ordinals")?;
        Ok(())
    }

    /// Helper function that increments ordinals equal to at least `start_ord`.
    async fn right_shift_ordinals(
        db: &impl ConnectionTrait,
        start_ord: i32,
        category_id: Option<i32>,
    ) -> Result<(), AppError> {
        category_tags::Entity::update_many()
            .filter(category_tags::Column::Ordinal.gte(start_ord))
            .filter(
                Condition::any()
                    .add(category_tags::Column::CategoryId.eq(category_id))
                    .add(
                        category_tags::Column::CategoryId
                            .is_null()
                            .and(SimpleExpr::from(category_id.is_none())),
                    ),
            )
            .col_expr(
                category_tags::Column::Ordinal,
                SimpleExpr::from(category_tags::Column::Ordinal.into_expr())
                    .add(SimpleExpr::Value(Value::Int(Some(1)))),
            )
            .exec(db)
            .await
            .context("failed to right shift ordinals")?;
        Ok(())
    }
}
