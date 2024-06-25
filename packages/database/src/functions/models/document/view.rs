use anyhow::{anyhow, Result};

use sea_orm::{EntityTrait, PaginatorTrait, QueryOrder, QuerySelect};

use crate::{models::*, DB_CONN};

pub async fn count() -> Result<u64> {
    let count = document_file::Entity::find()
        .count(
            DB_CONN
                .get()
                .ok_or(anyhow!("Failed to get database connection"))?,
        )
        .await?;

    Ok(count)
}

pub async fn list(offset: usize, limit: usize) -> Result<Vec<document_file::Model>> {
    let ret = document_file::Entity::find()
        .offset(offset as u64)
        .limit(limit as u64)
        .order_by_desc(document_file::Column::Id)
        .all(
            DB_CONN
                .get()
                .ok_or(anyhow!("Failed to get database connection"))?,
        )
        .await?;

    Ok(ret)
}

pub async fn select(id: i64) -> Result<Option<document_file::Model>> {
    let ret = document_file::Entity::find_by_id(id)
        .one(
            DB_CONN
                .get()
                .ok_or(anyhow!("Failed to get database connection"))?,
        )
        .await?;

    Ok(ret)
}
