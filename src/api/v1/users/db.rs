use anyhow::{Error, Result, anyhow};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, EntityTrait, IntoActiveModel, PaginatorTrait, QueryFilter,
    QueryOrder, QuerySelect, QueryTrait, Set,
};

use crate::{
    DB,
    api::v1::users::model::{QueryRequest, RegisterRequest, UpdateRequest, UserBrief},
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
        .filter(Column::DelFlag.eq(false))
        .limit(1)
        .one(db)
        .await?
        .is_some())
}

/// 插入用户数据
pub async fn insert_user(payload: RegisterRequest, db: &DB) -> Result<()> {
    TryInto::<ActiveModel>::try_into(payload)?
        .insert(db)
        .await?;
    Ok(())
}

#[derive(thiserror::Error, Debug)]
pub enum DbError {
    #[error("用户不存在")]
    NotFound,
    #[error("数据库错误: {0}")]
    DbError(Error),
}

/// 删除用户数据
pub async fn delete_user(id: &str, db: &DB) -> Result<(), DbError> {
    match SysUser::find_by_id(decode_id(id).map_err(|e| DbError::DbError(e.into()))?)
        .one(db)
        .await
    {
        Ok(Some(mut user)) => {
            if user.del_flag {
                return Err(DbError::NotFound);
            }
            user.del_flag = true;
            user.update_time = Some(chrono::Utc::now().naive_utc());
            let active_model: ActiveModel = user.into();
            active_model
                .update(db)
                .await
                .map_err(|e| DbError::DbError(e.into()))?;
            Ok(())
        }
        Ok(None) => Err(DbError::NotFound.into()),
        Err(e) => Err(DbError::DbError(e.into())),
    }
}

/// 更新用户数据
pub async fn update_user(payload: UpdateRequest, db: &DB) -> Result<(), DbError> {
    let user = SysUser::find_by_id(decode_id(&payload.id).map_err(|e| DbError::DbError(e.into()))?)
        .one(db)
        .await
        .map_err(|e| DbError::DbError(e.into()))?
        .ok_or_else(|| DbError::NotFound)?;

    if user.del_flag {
        return Err(DbError::NotFound);
    }

    let mut user: ActiveModel = user.into_active_model();
    if let Some(name) = payload.name {
        user.name = Set(name);
    }
    if let Some(password) = payload.password {
        user.password = Set(password);
    }
    if let Some(role) = payload.role {
        user.role = Set(role
            .parse()
            .map_err(|e| DbError::DbError(anyhow!("panic: {e}")))?);
    }
    if let Some(timezone) = payload.timezone {
        user.timezone = Set(timezone);
    }
    if let Some(locale) = payload.locale {
        user.locale = Set(locale);
    }
    user.updater_id = Set(0); // TODO
    user.update_time = Set(Some(chrono::Utc::now().naive_utc()));
    user.update(db)
        .await
        .map_err(|e| DbError::DbError(e.into()))?;
    Ok(())
}

/// 查询用户数据
pub async fn query_user(payload: &QueryRequest, db: &DB) -> Result<(Vec<UserBrief>, u64)> {
    let role = match &payload.role {
        Some(role) => Some(role.parse::<i32>()?),
        None => None,
    };
    let paginator = SysUser::find()
        .apply_if(payload.name.as_ref(), |q, v| q.filter(Column::Name.eq(v)))
        .apply_if(role, |q, v| q.filter(Column::Role.eq(v)))
        .apply_if(payload.timezone.as_ref(), |q, v| {
            q.filter(Column::Timezone.eq(v))
        })
        .apply_if(payload.locale.as_ref(), |q, v| {
            q.filter(Column::Locale.eq(v))
        })
        .order_by_asc(Column::Id)
        .paginate(db, payload.per_page);

    let total_pages = paginator
        .num_pages()
        .await
        .map_err(|e| DbError::DbError(e.into()))?;
    let items = paginator
        .fetch_page(payload.page.saturating_sub(1))
        .await
        .map_err(|e| DbError::DbError(e.into()))?
        .par_iter()
        .map(TryInto::<UserBrief>::try_into)
        .collect::<Result<Vec<UserBrief>, Error>>()?;

    Ok((items, total_pages))
}
