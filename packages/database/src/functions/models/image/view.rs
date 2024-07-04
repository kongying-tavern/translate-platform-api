use anyhow::{anyhow, Result};

use sea_orm::{EntityTrait, PaginatorTrait, QueryOrder, QuerySelect};

use crate::{models::*, DB_CONN};

pub async fn count() -> Result<u64> {
    let count = image_file::Entity::find()
        .count(
            DB_CONN
                .get()
                .ok_or(anyhow!("Failed to get database connection"))?,
        )
        .await?;

    Ok(count)
}

pub async fn list(offset: usize, limit: usize) -> Result<Vec<image_file::Model>> {
    let ret = image_file::Entity::find()
        .offset(offset as u64)
        .limit(limit as u64)
        .order_by_desc(image_file::Column::Id)
        .all(
            DB_CONN
                .get()
                .ok_or(anyhow!("Failed to get database connection"))?,
        )
        .await?;

    Ok(ret)
}

pub async fn select(id: i64) -> Result<Option<image_file::Model>> {
    let ret = image_file::Entity::find_by_id(id)
        .one(
            DB_CONN
                .get()
                .ok_or(anyhow!("Failed to get database connection"))?,
        )
        .await?;

    Ok(ret)
}
