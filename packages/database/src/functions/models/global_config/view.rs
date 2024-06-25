use anyhow::{anyhow, Result};

use sea_orm::{EntityTrait, PaginatorTrait, QuerySelect};

use crate::{models::*, DB_CONN};

pub async fn count() -> Result<u64> {
    let count = global_config::Entity::find()
        .count(
            DB_CONN
                .get()
                .ok_or(anyhow!("Failed to get database connection"))?,
        )
        .await?;

    Ok(count)
}

pub async fn list(offset: usize, limit: usize) -> Result<Vec<global_config::Model>> {
    let ret = global_config::Entity::find()
        .offset(offset as u64)
        .limit(limit as u64)
        .all(
            DB_CONN
                .get()
                .ok_or(anyhow!("Failed to get database connection"))?,
        )
        .await?;

    Ok(ret)
}

pub async fn select(id: i64) -> Result<Option<global_config::Model>> {
    let ret = global_config::Entity::find_by_id(id)
        .one(
            DB_CONN
                .get()
                .ok_or(anyhow!("Failed to get database connection"))?,
        )
        .await?;

    Ok(ret)
}
