use crate::{
    commands::{
        columns::{RenameColumnInput, UpdateColumnOrdinalInput},
        tags::{AttachTagToCategoryInput, CreateTagInput, UpdateTagNameInput},
    },
    errors::AppError,
};
use anyhow::Context;
use entity::columns;
use sea_orm::{DbConn, EntityTrait, ActiveModelTrait, Set, IntoActiveModel, QueryFilter, ColumnTrait, sea_query::SimpleExpr, Value, QuerySelect, TransactionTrait, ConnectionTrait};

pub struct Query;

impl Query {
    async fn get_ordinal_from_id(db: &DbConn, id: i32) -> Result<i32, AppError> {
        let res = columns::Entity::find_by_id(id)
            .one(db)
            .await
            .context("failed to get ordinal from id")?
            .ok_or(AppError::RowNotFound)?;
        Ok(res.ordinal)
    }

    async fn get_column_count(db: &DbConn) -> Result<i32, AppError> {
        let res: Option<i32> = columns::Entity::find()
            .select_only()
            .column_as(columns::Column::Id.count(), "count")
            .into_tuple()
            .one(db)
            .await
            .context("failed to determine count of columns")?;
        Ok(res.expect("where is the count???"))
    }
}

pub struct Mutation;

impl Mutation {
    pub async fn insert_column(db: &DbConn, name: String) -> Result<columns::Model, AppError> {
        let column_count = Query::get_column_count(db).await?;
        let model = columns::ActiveModel {
            name: Set(name),
            ordinal: Set(column_count),
            ..Default::default()
        };
        let model = model.insert(db).await.context("failed to insert column")?;
        Ok(model)
    }

    pub async fn update_column_name(db: &DbConn, data: RenameColumnInput) -> Result<(), AppError> {
        let mut model = columns::Entity::find_by_id(data.id)
            .one(db)
            .await
            .context("failed to select column")?
            .ok_or(AppError::RowNotFound)?
            .into_active_model();

        model.name = Set(data.new_name);
        model.update(db).await.context("failed to update column")?;
        Ok(())
    }

    pub async fn update_column_ordinal(db: &DbConn, data: UpdateColumnOrdinalInput) -> Result<(), AppError> {
        let old_ord = Query::get_ordinal_from_id(db, data.column_id).await?;
        let mut model = columns::Entity::find_by_id(data.column_id)
        .one(db)
        .await
        .context("failed to select column")?
        .ok_or(AppError::RowNotFound)?
        .into_active_model();
    
        let tr = db.begin().await.context("failed to begin transaction")?;
        Self::left_shift_ordinals(&tr, old_ord).await?;
        Self::right_shift_ordinals(&tr, data.new_ord).await?;

        model.ordinal = Set(data.new_ord);
        model.update(&tr).await.context("failed to update column")?;
        tr.commit().await.context("failed to commit transaction")?;
        
        Ok(())
    }

    pub async fn delete_column_by_id(db: &DbConn, id: i32) -> Result<(), AppError> {
        let deleted_ordinal = Query::get_ordinal_from_id(db, id).await?;
        let tr = db.begin().await.context("failed to begin transaction")?;
        columns::Entity::delete_by_id(id).exec(&tr).await.context("failed to delete column")?;
        Self::left_shift_ordinals(&tr, deleted_ordinal).await?;
        tr.commit().await.context("failed to commit transaction")?;
        Ok(())
    }

    async fn left_shift_ordinals(db: &impl ConnectionTrait, start_ord: i32) -> Result<(), AppError> {
        // todo: figure out how to do update_many() with column values as expressions
        columns::Entity::update_many()
            .filter(columns::Column::Ordinal.gt(start_ord))
            .col_expr(columns::Column::Ordinal, SimpleExpr::from(columns::Column::Ordinal.into_expr()).sub(SimpleExpr::Value(Value::Int(Some(1)))))
            .exec(db).await.context("failed to left shift ordinals")?;
        Ok(())
    }

    async fn right_shift_ordinals(db: &impl ConnectionTrait, start_ord: i32) -> Result<(), AppError> {
        columns::Entity::update_many()
            .filter(columns::Column::Ordinal.gte(start_ord))
            .col_expr(columns::Column::Ordinal, SimpleExpr::from(columns::Column::Ordinal.into_expr()).add(SimpleExpr::Value(Value::Int(Some(1)))))
            .exec(db).await.context("failed to right shift ordinals")?;
        Ok(())
    }
}
