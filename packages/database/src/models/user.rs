use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use strum::{Display, EnumIter, EnumMessage, EnumString};

use sea_orm::entity::prelude::*;

#[derive(
    Clone,
    Debug,
    PartialEq,
    EnumIter,
    EnumString,
    EnumMessage,
    Display,
    DeriveActiveEnum,
    Deserialize,
    Serialize,
    Default,
)]
#[serde(rename_all = "snake_case")]
#[sea_orm(rs_type = "i64", db_type = "Integer")]
pub enum Permission {
    #[sea_orm(num_value = 0)]
    #[strum(message = "访客")]
    #[default]
    Guest,
    #[sea_orm(num_value = 1)]
    #[strum(message = "用户")]
    User,
    #[sea_orm(num_value = 2)]
    #[strum(message = "管理员")]
    Manager,
    #[sea_orm(num_value = 3)]
    #[strum(message = "超级管理员")]
    Root,
}

impl std::cmp::PartialOrd for Permission {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let self_num = match self {
            Permission::Guest => 0,
            Permission::User => 1,
            Permission::Manager => 2,
            Permission::Root => 3,
        };
        let other_num = match other {
            Permission::Guest => 0,
            Permission::User => 1,
            Permission::Manager => 2,
            Permission::Root => 3,
        };

        self_num.partial_cmp(&other_num)
    }
}

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    #[sea_orm(default = "now()")]
    pub updated_at: DateTime<Utc>,

    #[sea_orm(indexed)]
    pub name: String,
    pub password_hash: Option<String>,

    #[sea_orm(indexed)]
    pub permission: Permission,
    #[sea_orm(column_type = "Text")]
    pub extra_profile: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
