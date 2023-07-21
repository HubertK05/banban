use std::collections::HashMap;

use crate::{
    commands::category::{SelectCategoriesOutput, SelectCategoryOutput, UpdateCategoryNameInput},
    errors::AppError,
};
use anyhow::Context;
use entity::{categories, category_tags};
use sea_orm::{
    ActiveModelTrait, DbConn, EntityTrait, FromQueryResult, IntoActiveModel, QuerySelect,
    RelationTrait, Set,
};

#[derive(FromQueryResult)]
struct CategoryQueryResult {
    tag_name: String,
    category_id: Option<i32>,
    category_name: Option<String>,
}

pub struct Query;

impl Query {
    pub async fn select_all_categories(db: &DbConn) -> Result<SelectCategoriesOutput, AppError> {
        let category_tags: Vec<CategoryQueryResult> = category_tags::Entity::find()
            .columns([
                category_tags::Column::TagName,
                category_tags::Column::CategoryId,
            ])
            .columns([categories::Column::Name])
            .join(
                sea_orm::JoinType::InnerJoin,
                category_tags::Relation::Categories.def(),
            )
            .into_model()
            .all(db)
            .await
            .context("failed to select categories")?;

        let res: HashMap<i32, SelectCategoryOutput> =
            category_tags
                .into_iter()
                .fold(HashMap::new(), |mut acc, record| {
                    if let (Some(category_id), Some(category_name)) =
                        (record.category_id, record.category_name)
                    {
                        let entry = acc.entry(category_id).or_insert(SelectCategoryOutput {
                            name: category_name,
                            tags: vec![],
                        });
                        entry.tags.push(record.tag_name);
                    }

                    acc
                });

        Ok(res)
    }
}

pub struct Mutation;

impl Mutation {
    pub async fn insert_category(db: &DbConn, name: String) -> Result<categories::Model, AppError> {
        let data = categories::ActiveModel {
            name: Set(name),
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
        categories::Entity::delete_by_id(id)
            .exec(db)
            .await
            .context("failed to delete category")?;
        Ok(())
    }
}
