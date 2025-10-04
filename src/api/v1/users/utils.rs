use anyhow::{Error, Result};
use chrono::Utc;
use sea_orm::ActiveValue::NotSet;
use tracing::debug;

use crate::{entities::sys_user, sys_user::encode_id};

use super::model;

pub fn check_option<T, F>(field: &Option<T>, check: F, field_name: &str, res: &mut String)
where
    T: AsRef<str>,
    F: Fn(&str) -> bool,
{
    if let Some(value) = field {
        if !check(value.as_ref()) {
            debug!("{}格式错误: {}", field_name, value.as_ref());
            res.push_str(&format!("{}格式错误\n", field_name));
        }
    }
}

pub fn check_field<T, F>(field: &T, check: F, field_name: &str, res: &mut String)
where
    T: AsRef<str>,
    F: Fn(&str) -> bool,
{
    if !check(field.as_ref()) {
        debug!("{}格式错误: {}", field_name, field.as_ref());
        res.push_str(&format!("{}格式错误\n", field_name));
    }
}

impl TryInto<sys_user::ActiveModel> for model::RegisterRequest {
    type Error = Error;

    fn try_into(self) -> Result<sys_user::ActiveModel> {
        Ok(sys_user::ActiveModel {
            id: NotSet,
            name: sea_orm::Set(self.name),
            password: sea_orm::Set(self.password),
            role: sea_orm::Set(self.role.parse()?),
            timezone: sea_orm::Set(self.timezone),
            locale: sea_orm::Set(self.locale),
            del_flag: sea_orm::Set(false),
            version: sea_orm::Set(0),
            creator_id: sea_orm::Set(0),
            create_time: sea_orm::Set(Some(Utc::now().naive_utc())),
            updater_id: sea_orm::Set(0),
            update_time: sea_orm::Set(None),
        })
    }
}

impl TryInto<model::UserBrief> for &sys_user::Model {
    type Error = Error;

    fn try_into(self) -> Result<model::UserBrief> {
        Ok(model::UserBrief {
            id: encode_id(self.id)?,
            name: self.name.clone(),
            role: self.role.to_string(),
            timezone: self.timezone.clone(),
            locale: self.locale.clone(),
        })
    }
}
