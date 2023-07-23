use std::collections::HashMap;

use crate::{
    commands::category::{SelectCategoryTagsOutput, SelectCategoryOutput, UpdateCategoryNameInput, UpdateCategoryOrdinalInput, TagOrdinal},
    errors::AppError,
};
use anyhow::Context;
use entity::{category_tags, categories};
use sea_orm::{DbConn, EntityTrait, QuerySelect, RelationTrait, FromQueryResult, ActiveModelTrait, Set, IntoActiveModel, sea_query::SimpleExpr, QueryFilter, ColumnTrait, Value, ConnectionTrait, TransactionTrait, QueryOrder};

#[derive(FromQueryResult)]
struct CategoryQueryResult {
    tag_id: i32,
    tag_name: String,
    tag_ordinal: i32,
    category_id: Option<i32>,
    category_name: Option<String>,
    category_ordinal: Option<i32>,
}

pub struct Query;

impl Query {
    pub async fn select_all_categories(db: &DbConn) -> Result<SelectCategoryTagsOutput, AppError> {
        let category_tags: Vec<CategoryQueryResult> = category_tags::Entity::find()
            .select_only()
            .column_as(category_tags::Column::TagName, "tag_name")
            .column_as(category_tags::Column::Ordinal, "tag_ordinal")
            .column_as(category_tags::Column::Id, "tag_id")
            .column_as(categories::Column::Id, "category_id")
            .column_as(categories::Column::Name, "category_name")
            .column_as(categories::Column::Ordinal, "category_ordinal")
            .join(
                sea_orm::JoinType::InnerJoin,
                category_tags::Relation::Categories.def(),
            )
            .order_by(categories::Column::Ordinal, sea_orm::Order::Asc)
            .order_by(category_tags::Column::Ordinal, sea_orm::Order::Asc)
            .into_model()
            .all(db)
            .await
            .context("failed to select categories")?;

        let res: SelectCategoryTagsOutput =
            category_tags
                .into_iter()
                .fold(SelectCategoryTagsOutput { category_tags: HashMap::new(), other_tags: SelectCategoryOutput { name: None, ordinal: None, tags: Vec::new() } }, |mut acc, record| {
                    if let Some(category_id) = record.category_id {
                        let entry = acc.category_tags.entry(category_id).or_insert(SelectCategoryOutput {
                            name: record.category_name,
                            tags: vec![],
                            ordinal: record.category_ordinal,
                        });
                        entry.tags.push(TagOrdinal {
                            id: record.tag_id,
                            tag: record.tag_name,
                            ordinal: record.tag_ordinal,
                        });
                    } else {
                        acc.other_tags.tags.push(TagOrdinal {
                            id: record.tag_id,
                            tag: record.tag_name,
                            ordinal: record.tag_ordinal,
                        });
                    }

                    acc
                });

        Ok(res)
    }

    async fn get_ordinal_from_id(db: &DbConn, id: i32) -> Result<i32, AppError> {
        let res = categories::Entity::find_by_id(id)
            .one(db).await.context("failed to get ordinal from id")?
            .ok_or(AppError::RowNotFound)?;
        Ok(res.ordinal)
    }

    async fn get_category_count(db: &DbConn) -> Result<i32, AppError> {
        let res: Option<i32> = categories::Entity::find()
            .select_only()
            .column_as(categories::Column::Id.count(), "count").into_tuple().one(db).await.context("failed to determine count of categories")?;
        Ok(res.expect("where is the count???"))
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

    pub async fn update_category_ordinal(db: &DbConn, data: UpdateCategoryOrdinalInput) -> Result<(), AppError> {
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
        model.update(&tr).await.context("failed to update category")?;
        tr.commit().await.context("failed to commit transaction")?;
        
        Ok(())
    }

    pub async fn delete_category_by_id(db: &DbConn, id: i32) -> Result<(), AppError> {
        let deleted_ord = Query::get_ordinal_from_id(db, id).await?;
        let tr = db.begin().await.context("failed to begin transaction")?;
        categories::Entity::delete_by_id(id).exec(&tr).await.context("failed to delete category")?;
        Self::left_shift_ordinals(&tr, deleted_ord).await?;
        tr.commit().await.context("failed to commit transaction")?;
        Ok(())
    }

    async fn left_shift_ordinals(db: &impl ConnectionTrait, start_ord: i32) -> Result<(), AppError> {
        categories::Entity::update_many()
            .filter(categories::Column::Ordinal.gt(start_ord))
            .col_expr(categories::Column::Ordinal, SimpleExpr::from(categories::Column::Ordinal.into_expr()).sub(SimpleExpr::Value(Value::Int(Some(1)))))
            .exec(db).await.context("failed to left shift ordinals")?;
        Ok(())
    }

    async fn right_shift_ordinals(db: &impl ConnectionTrait, start_ord: i32) -> Result<(), AppError> {
        categories::Entity::update_many()
            .filter(categories::Column::Ordinal.gte(start_ord))
            .col_expr(categories::Column::Ordinal, SimpleExpr::from(categories::Column::Ordinal.into_expr()).add(SimpleExpr::Value(Value::Int(Some(1)))))
            .exec(db).await.context("failed to right shift ordinals")?;
        Ok(())
    }
}
