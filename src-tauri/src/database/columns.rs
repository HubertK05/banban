use std::collections::HashMap;

use crate::{
    commands::{
        columns::{RenameColumnInput, UpdateColumnOrdinalInput},
        fetch::ColumnOutput,
    },
    errors::AppError,
};
use anyhow::Context;
use entity::{
    activities,
    columns::{self, Entity as Column, Model},
};
use sea_orm::{
    sea_query::SimpleExpr, ActiveModelTrait, ColumnTrait, Condition, ConnectionTrait, DbConn,
    DbErr, EntityTrait, IntoActiveModel, PaginatorTrait, QueryFilter, QueryOrder, Set,
    TransactionTrait,
};

pub struct Query;

impl Query {
    /// Helper function to fetch column ordinal based on id.
    async fn get_ordinal_from_id(db: &DbConn, id: i32) -> Result<i32, AppError> {
        let res = columns::Entity::find_by_id(id)
            .one(db)
            .await
            .context("failed to get ordinal from id")?
            .ok_or(AppError::RowNotFound)?;
        Ok(res.ordinal)
    }

    /// Helper function to get a current number of columns.
    async fn get_column_count(db: &DbConn) -> Result<i32, AppError> {
        let res = columns::Entity::find()
            .count(db)
            .await
            .context("failed to determine count of columns")?;
        Ok(res as i32)
    }

    /// Fetches all persisted columns with their contents associated with their ids.
    ///
    /// Used at the startup of the application.
    pub async fn all_columns(db: &DbConn) -> Result<HashMap<i32, ColumnOutput>, DbErr> {
        let res = Column::find()
            .find_with_related(activities::Entity)
            .order_by_asc(columns::Column::Ordinal)
            .all(db)
            .await?;

        let out: HashMap<i32, ColumnOutput> =
            res.into_iter()
                .fold(HashMap::new(), |mut acc, (column, activities)| {
                    acc.insert(
                        column.id,
                        ColumnOutput {
                            name: column.name,
                            ordinal: column.ordinal,
                            activities: activities
                                .into_iter()
                                .map(|activity| activity.id)
                                .collect(),
                        },
                    );
                    acc
                });
        Ok(out)
    }

    /// Helper function to get the number of activities in a column given by id.
    ///
    /// A `None` value in `column_id` means counting activities in the stash.
    ///
    /// Returns 0 if column with id equal to `column_id` does not exist.
    async fn get_activity_count_in_column(
        db: &DbConn,
        column_id: Option<i32>,
    ) -> Result<i32, AppError> {
        let res = activities::Entity::find()
            .filter(
                Condition::any()
                    .add(activities::Column::ColumnId.eq(column_id))
                    .add(
                        activities::Column::ColumnId
                            .is_null()
                            .and(SimpleExpr::from(column_id == None)),
                    ),
            )
            .count(db)
            .await
            .context("failed to determine count of columns")?;
        Ok(res as i32)
    }
}

pub struct Mutation;

impl Mutation {
    /// Appends column to the end of the column list.
    ///
    /// Returns the created column with appropriate id and ordinal.
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

    /// Renames the column with id and new name provided in `data`.
    ///
    /// Returns `Err(RowNotFound)` if column with the given id does not exist.
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

    /// Updates the position of the given column in the column list.
    ///
    /// Returns `Err(RowNotFound)` if column with id given in `data` does not exist.
    pub async fn update_column_ordinal(
        db: &DbConn,
        data: UpdateColumnOrdinalInput,
    ) -> Result<(), AppError> {
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

    /// Deletes column with the id equal to `id`.
    ///
    /// Updates ordinals to maintain correct order of columns.
    pub async fn delete_column_by_id(db: &DbConn, id: i32) -> Result<(), AppError> {
        let deleted_ordinal = Query::get_ordinal_from_id(db, id).await?;
        let other_activity_count = Query::get_activity_count_in_column(db, None)
            .await
            .context("failed to determine the count of other activities")?;
        let tr = db.begin().await.context("failed to begin transaction")?;

        activities::Entity::update_many()
            .filter(activities::Column::ColumnId.eq(id))
            .col_expr(
                activities::Column::Ordinal,
                activities::Column::Ordinal
                    .into_expr()
                    .add(other_activity_count),
            )
            .exec(&tr)
            .await
            .context("failed to update activity ordinals")?;

        columns::Entity::delete_by_id(id)
            .exec(&tr)
            .await
            .context("failed to delete column")?;

        Self::left_shift_ordinals(&tr, deleted_ordinal).await?;
        tr.commit().await.context("failed to commit transaction")?;
        Ok(())
    }

    /// Helper function that decrements ordinals greater than `start_ord`.
    async fn left_shift_ordinals(
        db: &impl ConnectionTrait,
        start_ord: i32,
    ) -> Result<(), AppError> {
        columns::Entity::update_many()
            .filter(columns::Column::Ordinal.gt(start_ord))
            .col_expr(
                columns::Column::Ordinal,
                columns::Column::Ordinal.into_expr().sub(1),
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
    ) -> Result<(), AppError> {
        columns::Entity::update_many()
            .filter(columns::Column::Ordinal.gte(start_ord))
            .col_expr(
                columns::Column::Ordinal,
                columns::Column::Ordinal.into_expr().add(1),
            )
            .exec(db)
            .await
            .context("failed to right shift ordinals")?;
        Ok(())
    }
}
