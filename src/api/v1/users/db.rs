use anyhow::{Error, Result};
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, ModelTrait, QueryFilter, QuerySelect};

use crate::{
    DB,
    api::v1::users::model::RegisterRequest,
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
        Ok(Some(user)) => {
            user.delete(db)
                .await
                .map_err(|e| DeleteError::DbError(e.into()))?;
            Ok(())
        }
        Ok(None) => Err(DeleteError::NotFound.into()),
        Err(e) => Err(DeleteError::DbError(e.into())),
    }
}
