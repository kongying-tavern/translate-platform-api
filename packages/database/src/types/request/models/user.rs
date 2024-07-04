use anyhow::Context;
use serde::{Deserialize, Serialize};

use sea_orm::ActiveValue::{NotSet, Set};

use crate::{
    functions::api::auth::generate_hash,
    models::user::{ActiveModel, Model, Permission},
};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct User {
    pub id: Option<i64>,
    pub name: String,
    pub password_raw: Option<String>,

    pub permission: Permission,
    pub extra_profile: ExtraProfile,
}

impl Default for User {
    fn default() -> Self {
        Self {
            id: None,
            name: "".to_string(),
            password_raw: None,
            permission: Permission::Guest,
            extra_profile: ExtraProfile::Guest(GuestExtraProfile::default()),
        }
    }
}

impl From<User> for ActiveModel {
    fn from(info: User) -> Self {
        Self {
            id: if let Some(id) = info.id {
                Set(id)
            } else {
                NotSet
            },
            updated_at: Set(chrono::Utc::now()),
            name: Set(info.name),
            password_hash: {
                if let Some(password) = info.password_raw.clone() {
                    Set(Some(
                        generate_hash(password)
                            .context("Failed to generate password hash for user model")
                            .unwrap(),
                    ))
                } else {
                    Set(None)
                }
            },
            permission: Set(info.permission),
            extra_profile: Set(serde_json::to_string(&info.extra_profile)
                .context("Failed to serialize extra profile for user model")
                .unwrap()),
        }
    }
}

impl From<Model> for User {
    fn from(val: Model) -> Self {
        User {
            id: Some(val.id),
            name: val.name,
            password_raw: None,
            permission: val.permission,
            extra_profile: serde_json::from_str(&val.extra_profile)
                .context("Failed to deserialize extra profile for user model")
                .unwrap(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub enum ExtraProfile {
    User(UserExtraProfile),
    Guest(GuestExtraProfile),
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Default)]
pub struct UserExtraProfile {
    pub group: Vec<String>,
    pub email: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Default)]
pub struct GuestExtraProfile {
    pub organization: Option<String>,
    pub name: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub address: Option<String>,
}
