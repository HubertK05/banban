//! `SeaORM` Entity. Generated by sea-orm-codegen 0.11.3

use sea_orm::entity::prelude::*;
use serde::Serialize;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize)]
#[sea_orm(table_name = "activities")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
    pub body: Option<String>,
    pub column_id: Option<i32>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::columns::Entity",
        from = "Column::ColumnId",
        to = "super::columns::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Columns,
}

impl Related<super::columns::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Columns.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}