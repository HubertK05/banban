use std::collections::HashMap;

use crate::{
    commands::{
        category::{
            SelectCategoryOutput, SelectCategoryTagsOutput, TagData, UpdateCategoryNameInput,
            UpdateCategoryOrdinalInput,
        },
        fetch::{CategoryOutput, CategoryTagOutput},
    },
    errors::AppError,
    utils::coloring::rgb_int_to_string,
};
use anyhow::Context;
use entity::{
    categories::{self, Entity as Category, Model},
    category_tags::{self, Entity as CategoryTag},
};
use sea_orm::{
    sea_query::SimpleExpr, ActiveModelTrait, ColumnTrait, ConnectionTrait, DbConn, DbErr,
    EntityTrait, FromQueryResult, IntoActiveModel, PaginatorTrait, QueryFilter, QueryOrder,
    QuerySelect, RelationTrait, Set, TransactionTrait, Value,
};
use serde::Serialize;

#[derive(FromQueryResult)]
struct CategoryQueryResult {
    tag_id: i32,
    tag_name: String,
    tag_ordinal: i32,
    tag_color: i32,
    category_id: Option<i32>,
    category_name: Option<String>,
    category_ordinal: Option<i32>,
}

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
    pub async fn select_all_categories(db: &DbConn) -> Result<SelectCategoryTagsOutput, AppError> {
        let category_tags: Vec<CategoryQueryResult> = category_tags::Entity::find()
            .select_only()
            .column_as(category_tags::Column::TagName, "tag_name")
            .column_as(category_tags::Column::Ordinal, "tag_ordinal")
            .column_as(category_tags::Column::Id, "tag_id")
            .column_as(category_tags::Column::Color, "tag_color")
            .column_as(categories::Column::Id, "category_id")
            .column_as(categories::Column::Name, "category_name")
            .column_as(categories::Column::Ordinal, "category_ordinal")
            .join(
                sea_orm::JoinType::LeftJoin,
                category_tags::Relation::Categories.def(),
            )
            .order_by(categories::Column::Ordinal, sea_orm::Order::Asc)
            .order_by(category_tags::Column::Ordinal, sea_orm::Order::Asc)
            .into_model()
            .all(db)
            .await
            .context("failed to select categories")?;

        let res: SelectCategoryTagsOutput = category_tags.into_iter().fold(
            SelectCategoryTagsOutput::default(),
            |mut acc, record| {
                let tags = if let Some(category_id) = record.category_id {
                    &mut acc
                        .category_tags
                        .entry(category_id)
                        .or_insert(SelectCategoryOutput::new_empty(
                            record.category_name,
                            record.category_ordinal,
                        ))
                        .tags
                } else {
                    &mut acc.other_tags.tags
                };

                tags.push(TagData {
                    id: record.tag_id,
                    tag: record.tag_name,
                    ordinal: record.tag_ordinal,
                    color: rgb_int_to_string(record.tag_color),
                });

                acc
            },
        );

        Ok(res)
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

    pub async fn update_category_ordinal(
        db: &DbConn,
        data: UpdateCategoryOrdinalInput,
    ) -> Result<(), AppError> {
        let old_ord = Query::get_ordinal_from_id(db, data.category_id).await?;
        let mut model = categories::Entity::find_by_id(data.category_id)
            .one(db)
            .await
            .context("failed to select category")?
            .ok_or(AppError::RowNotFound)?
            .into_active_model();

        let tr = db.begin().await.context("failed to begin transaction")?;
        Self::left_shift_ordinals(&tr, old_ord).await?;
        Self::right_shift_ordinals(&tr, data.new_ord).await?;

        model.ordinal = Set(data.new_ord);
        model
            .update(&tr)
            .await
            .context("failed to update category")?;
        tr.commit().await.context("failed to commit transaction")?;

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
