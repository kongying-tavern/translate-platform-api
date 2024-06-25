use anyhow::{anyhow, Result};

use sea_orm::{EntityTrait, PaginatorTrait, QueryOrder, QuerySelect};

use crate::{models::*, types::request::models::User as VO, DB_CONN};

pub async fn count() -> Result<u64> {
    let count = user::Entity::find()
        .count(
            DB_CONN
                .get()
                .ok_or(anyhow!("Failed to get database connection"))?,
        )
        .await?;

    Ok(count)
}

pub async fn list(offset: usize, limit: usize) -> Result<Vec<VO>> {
    let ret = user::Entity::find()
        .offset(offset as u64)
        .limit(limit as u64)
        .order_by_asc(user::Column::Id)
        .all(
            DB_CONN
                .get()
                .ok_or(anyhow!("Failed to get database connection"))?,
        )
        .await?;

    Ok(ret
        .iter()
        .map(|item| item.clone().into())
        .collect::<Vec<_>>())
}

pub async fn select(id: i64) -> Result<Option<VO>> {
    let ret = user::Entity::find_by_id(id)
        .one(
            DB_CONN
                .get()
                .ok_or(anyhow!("Failed to get database connection"))?,
        )
        .await?;

    Ok(ret.map(|item| item.into()))
}
