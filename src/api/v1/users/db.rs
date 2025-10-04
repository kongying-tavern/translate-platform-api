use anyhow::{Error, Result};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, EntityTrait, IntoActiveModel, QueryFilter, QuerySelect, Set
};

use crate::{
    DB,
    api::v1::users::model::{RegisterRequest, UpdateRequest},
    entities::{
        prelude::SysUser,
        sys_user::{ActiveModel, Column},
    },
    sys_user::decode_id,
};

/// 检查用户名是否存在
pub async fn check_name_exists(name: &str, db: &DB) -> Result<bool> {
    Ok(SysUser::find()
        .filter(Column::Name.eq(name))
        .limit(1)
        .one(db)
        .await?
        .is_some())
}

/// 插入用户数据
pub async fn insert_user(payload: RegisterRequest, db: &DB) -> Result<()> {
    // let data = (payload)?;
    // ActiveModel::try_from(payload)?.insert(db).await?;
    TryInto::<ActiveModel>::try_into(payload)?
        .insert(db)
        .await?;
    Ok(())
}

#[derive(thiserror::Error, Debug)]
pub enum DeleteError {
    #[error("用户不存在")]
    NotFound,
    #[error("数据库错误: {0}")]
    DbError(Error),
}

/// 删除用户数据
pub async fn delete_user(id: &str, db: &DB) -> Result<(), DeleteError> {
    match SysUser::find_by_id(decode_id(id).map_err(|e| DeleteError::DbError(e.into()))?)
        .one(db)
        .await
    {
        Ok(Some(mut user)) => {
            user.del_flag = true;
            user.update_time = Some(chrono::Utc::now().naive_utc());
            let active_model: ActiveModel = user.into();
            active_model.update(db).await.map_err(|e| DeleteError::DbError(e.into()))?;
            Ok(())
        }
        Ok(None) => Err(DeleteError::NotFound.into()),
        Err(e) => Err(DeleteError::DbError(e.into())),
    }
}

pub async fn update_user(payload: UpdateRequest, db: &DB) -> Result<()> {
    let mut user = SysUser::find_by_id(decode_id(&payload.id)?)
        .one(db)
        .await?
        .ok_or_else(|| Error::msg("用户不存在"))?
        .into_active_model();

    if let Some(name) = payload.name {
        user.name = Set(name);
    }
    if let Some(password) = payload.password {
        user.password = Set(password);
    }
    if let Some(role) = payload.role {
        user.role = Set(role.parse()?);
    }
    if let Some(timezone) = payload.timezone {
        user.timezone = Set(timezone);
    }
    if let Some(locale) = payload.locale {
        user.locale = Set(locale);
    }
    user.updater_id = Set(0); // TODO
    user.update_time = Set(Some(chrono::Utc::now().naive_utc()));
    user.update(db).await?;
    Ok(())
}
