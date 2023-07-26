use sea_orm::prelude::*;

use entity::tags::{Entity as Tag, Model};

pub struct Query;

impl Query {
    pub async fn get_all_tags(db: &DbConn) -> Result<Vec<Model>, DbErr> {
        let res = Tag::find().into_model().all(db).await?;
        Ok(res)
    }
}

pub struct Mutation;

impl Mutation {}
