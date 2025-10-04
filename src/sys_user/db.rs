use anyhow::Result;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, QuerySelect};

use crate::{
    api::v1::users::model::RegisterRequest, entities::{
        prelude::SysUser,
        sys_user::{ActiveModel, Column},
    }, DB
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
