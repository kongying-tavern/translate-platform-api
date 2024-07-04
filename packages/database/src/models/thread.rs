use serde::{Deserialize, Serialize};
use strum::EnumIter;

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "threads")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,

    pub title: String,
    pub tag: String,
    #[sea_orm(indexed)]
    pub author: i64,
    pub reviewer: i64,

    pub updated: ChronoDateTimeUtc,
    #[sea_orm(column_type = "Text")]
    pub extra: Option<String>,
    #[sea_orm(column_type = "Text")]
    pub content: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::Author",
        to = "super::user::Column::Id"
    )]
    Author,
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::Reviewer",
        to = "super::user::Column::Id"
    )]
    Reviewer,
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Author.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
