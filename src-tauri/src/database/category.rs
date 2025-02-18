use std::collections::HashMap;

use crate::{
    commands::{
        category::UpdateCategoryNameInput,
        fetch::{CategoryOutput, CategoryTagOutput},
    },
    errors::AppError,
    utils::coloring::rgb_int_to_string,
};
use anyhow::Context;
use entity::{
    categories::{self, Entity as Category, Model},
    category_tags::Entity as CategoryTag,
};
use sea_orm::{
    sea_query::SimpleExpr, ActiveModelTrait, ColumnTrait, ConnectionTrait, DbConn, DbErr,
    EntityTrait, IntoActiveModel, PaginatorTrait, QueryFilter, QueryOrder, Set, TransactionTrait,
    Value,
};

pub struct Query;

impl Query {
    pub async fn all_with_category_tags(
        db: &DbConn,
    ) -> Result<
        (
            HashMap<i32, CategoryOutput>,
            HashMap<i32, CategoryTagOutput>,
        ),
        DbErr,
    > {
        let res = Category::find()
            .find_with_related(CategoryTag)
            .all(db)
            .await?;

        let mut categories = HashMap::new();
        let mut category_tags = HashMap::new();
        res.into_iter().for_each(|(category, tags)| {
            categories.insert(
                category.id,
                CategoryOutput {
                    name: category.name,
                    ordinal: category.ordinal,
                    tags: tags.iter().map(|tag| tag.id).collect(),
                },
            );
            tags.into_iter().for_each(|tag| {
                category_tags.insert(
                    tag.id,
                    CategoryTagOutput {
                        name: tag.tag_name,
                        color: rgb_int_to_string(tag.color),
                        ordinal: tag.ordinal,
                        category_id: category.id,
                    },
                );
            })
        });

        Ok((categories, category_tags))
    }

    pub async fn get_all_categories(db: &DbConn) -> Result<Vec<Model>, DbErr> {
        let res = Category::find()
            .order_by_asc(categories::Column::Ordinal)
            .into_model()
            .all(db)
            .await?;
        trace!("{res:#?}");
        Ok(res)
    }

    async fn get_ordinal_from_id(db: &DbConn, id: i32) -> Result<i32, AppError> {
        let res = categories::Entity::find_by_id(id)
            .one(db)
            .await
            .context("failed to get ordinal from id")?
            .ok_or(AppError::RowNotFound)?;
        Ok(res.ordinal)
    }

    async fn get_category_count(db: &DbConn) -> Result<i32, AppError> {
        let res = categories::Entity::find()
            .count(db)
            .await
            .context("failed to determine count of columns")?;
        Ok(res as i32)
    }
}

pub struct Mutation;

impl Mutation {
    pub async fn insert_category(db: &DbConn, name: String) -> Result<categories::Model, AppError> {
        let category_count = Query::get_category_count(db).await?;
        let data = categories::ActiveModel {
            name: Set(name),
            ordinal: Set(category_count),
            ..Default::default()
        };

        let res = data.insert(db).await.context("failed to insert category")?;
        Ok(res)
    }

    pub async fn update_category_name(
        db: &DbConn,
        data: UpdateCategoryNameInput,
    ) -> Result<(), AppError> {
        let mut category = categories::Entity::find_by_id(data.id)
            .one(db)
            .await
            .context("failed to fetch category")?
            .ok_or(AppError::RowNotFound)?
            .into_active_model();

        category.set(categories::Column::Name, data.name.into());

        categories::Entity::update(category)
            .exec(db)
            .await
            .context("failed to update category")?;
        Ok(())
    }

    pub async fn delete_category_by_id(db: &DbConn, id: i32) -> Result<(), AppError> {
        let deleted_ord = Query::get_ordinal_from_id(db, id).await?;
        let tr = db.begin().await.context("failed to begin transaction")?;
        categories::Entity::delete_by_id(id)
            .exec(&tr)
            .await
            .context("failed to delete category")?;
        Self::left_shift_ordinals(&tr, deleted_ord).await?;
        tr.commit().await.context("failed to commit transaction")?;
        Ok(())
    }

    async fn left_shift_ordinals(
        db: &impl ConnectionTrait,
        start_ord: i32,
    ) -> Result<(), AppError> {
        categories::Entity::update_many()
            .filter(categories::Column::Ordinal.gt(start_ord))
            .col_expr(
                categories::Column::Ordinal,
                SimpleExpr::from(categories::Column::Ordinal.into_expr())
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
    ) -> Result<(), AppError> {
        categories::Entity::update_many()
            .filter(categories::Column::Ordinal.gte(start_ord))
            .col_expr(
                categories::Column::Ordinal,
                SimpleExpr::from(categories::Column::Ordinal.into_expr())
                    .add(SimpleExpr::Value(Value::Int(Some(1)))),
            )
            .exec(db)
            .await
            .context("failed to right shift ordinals")?;
        Ok(())
    }
}
