use crate::commands::tags::{UpdateTagColorInput, UpdateTagOrdinalInput};
use crate::utils::coloring::{rgb_string_to_int, string_to_color};
use crate::{
    commands::tags::{AttachTagToCategoryInput, CreateTagInput, UpdateTagNameInput},
    errors::AppError,
};
use anyhow::Context;
use entity::category_tags;
use sea_orm::Condition;
use sea_orm::{
    sea_query::SimpleExpr, ActiveModelTrait, ColumnTrait, ConnectionTrait, DbConn, EntityTrait,
    IntoActiveModel, PaginatorTrait, QueryFilter, QuerySelect, Set, TransactionTrait, Value,
};

pub struct Query;

impl Query {
    async fn get_ordinal_from_id(db: &DbConn, category_tag_id: i32) -> Result<i32, AppError> {
        let res = category_tags::Entity::find_by_id(category_tag_id)
            .one(db)
            .await
            .context("failed to get ordinal from id")?
            .ok_or(AppError::RowNotFound)?;
        Ok(res.ordinal)
    }

    async fn get_tag_count_from_category(
        db: &DbConn,
        category_id: Option<i32>,
    ) -> Result<i32, AppError> {
        let res = category_tags::Entity::find()
            .filter(
                Condition::any()
                    .add(category_tags::Column::CategoryId.eq(category_id))
                    .add(category_tags::Column::CategoryId.is_null().and(SimpleExpr::from(category_id == None)))
            )
            .count(db)
            .await
            .context("failed to determine count of columns")?;
        Ok(res as i32)
    }

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

    pub async fn update_category_tag(
        db: &DbConn,
        data: AttachTagToCategoryInput,
    ) -> Result<(), AppError> {
        let mut tag_model = category_tags::Entity::find_by_id(data.category_tag_id)
            .one(db)
            .await
            .context("failed to get category_tags model")?
            .ok_or(AppError::RowNotFound)?
            .into_active_model();

        tag_model.category_id = Set(data.category_id);
        category_tags::Entity::update(tag_model)
            .exec(db)
            .await
            .context("failed to update category_tags row")?;
        Ok(())
    }

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
                    .add(category_tags::Column::CategoryId.is_null().and(SimpleExpr::from(category_id == None)))
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
                    .add(category_tags::Column::CategoryId.is_null().and(SimpleExpr::from(category_id == None)))
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
