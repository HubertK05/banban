//! `SeaORM` Entity. Generated by sea-orm-codegen 0.11.3

use sea_orm::entity::prelude::*;
use serde::Serialize;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
#[sea_orm(table_name = "category_tags")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub tag_name: String,
    pub category_id: Option<i32>,
    pub ordinal: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::categories::Entity",
        from = "Column::CategoryId",
        to = "super::categories::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Categories,
}

impl Related<super::categories::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Categories.def()
    }
}

impl Related<super::activities::Entity> for Entity {
    fn to() -> RelationDef {
        super::activity_tags::Relation::Activities.def()
    }
    fn via() -> Option<RelationDef> {
        Some(super::activity_tags::Relation::CategoryTags.def().rev())
    }
}

impl ActiveModelBehavior for ActiveModel {}
