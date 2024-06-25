use serde::{Deserialize, Serialize};

use sea_orm::ActiveValue::{NotSet, Set};

use crate::models::global_config::{ActiveModel, Model};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize, Default)]
pub struct GlobalConfig {
    pub family: String,
    pub label: String,
    pub value: String,
}

impl From<GlobalConfig> for ActiveModel {
    fn from(info: GlobalConfig) -> Self {
        Self {
            id: NotSet,
            family: Set(info.family),
            label: Set(info.label),
            value: Set(info.value),
        }
    }
}

impl From<Model> for GlobalConfig {
    fn from(val: Model) -> Self {
        GlobalConfig {
            family: val.family,
            label: val.label,
            value: val.value,
        }
    }
}
