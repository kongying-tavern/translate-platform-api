use serde::{Deserialize, Serialize};

use sea_orm::entity::prelude::*;

use super::user::Permission;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "image_files")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,

    #[sea_orm(indexed)]
    pub uploader: i64,
    pub permission: Permission,

    #[sea_orm(indexed)]
    pub hash: String,
    pub size: u64,
    pub mime: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::Uploader",
        to = "super::user::Column::Id"
    )]
    Uploader,
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Uploader.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
