use anyhow::Context;
use entity::category_tags;
use sea_orm::{DbConn, EntityTrait, ActiveModelTrait, Set, IntoActiveModel};
use crate::{errors::AppError, commands::tags::{CreateTagInput, AttachTagToCategoryInput, UpdateTagNameInput}};

pub struct Mutation;

impl Mutation {
    pub async fn create_tag(db: &DbConn, data: CreateTagInput) -> Result<category_tags::Model, AppError> {
        let tag_model = category_tags::ActiveModel {
            tag_name: Set(data.tag_name),
            category_id: Set(data.category_id),
            ..Default::default()
        };
        
        let res = tag_model.insert(db).await.context("failed to insert tag")?;
        Ok(res)
    }

    pub async fn update_category_tag(db: &DbConn, data: AttachTagToCategoryInput) -> Result<(), AppError> {
        let mut tag_model = category_tags::Entity::find_by_id(data.category_tag_id)
            .one(db)
            .await
            .context("failed to get category_tags model")?
            .ok_or(AppError::RowNotFound)?
            .into_active_model();

        tag_model.category_id = Set(data.category_id);
        category_tags::Entity::update(tag_model).exec(db).await.context("failed to update category_tags row")?;
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
        category_tags::Entity::update(tag_model).exec(db).await.context("failed to update category_tags row")?;
        Ok(())
    }
    
    pub async fn delete_tag_by_id(db: &DbConn, id: i32) -> Result<(), AppError> {
        category_tags::Entity::delete_by_id(id).exec(db).await.context("failed to delete category_tag")?;
        Ok(())
    }
}
