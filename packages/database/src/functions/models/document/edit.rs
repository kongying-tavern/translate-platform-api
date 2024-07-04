use anyhow::{anyhow, ensure, Result};
use base64::prelude::*;
use bytes::Bytes;
use sha3::{Digest, Sha3_256};

use sea_orm::{
    ActiveModelTrait,
    ActiveValue::{NotSet, Set},
    ColumnTrait, EntityTrait, QueryFilter,
};

use crate::{
    functions::api::log::database::insert_database_log,
    models::{user::Permission, *},
    types::response::api::log::DatabaseLogItemOperation,
    DB_CONN, DOCUMENT_RES_DIR,
};

pub async fn insert(
    uploader_id: i64,
    data: Bytes,
    file_name_raw: String,
) -> Result<document_file::Model> {
    let hash = Sha3_256::digest(&data).to_vec();
    let hash = BASE64_URL_SAFE_NO_PAD.encode(&hash);
    let size = data.len() as u64;

    let file_name = format!("{}.dat", hash,);
    let file_path = DOCUMENT_RES_DIR.clone().join(&file_name);

    ensure!(!file_path.exists(), "Document already exists: {}", hash);
    tokio::fs::write(&file_path, data).await?;

    if let Some(item) = document_file::Entity::find()
        .filter(document_file::Column::Hash.eq(hash.clone()))
        .one(
            DB_CONN
                .get()
                .ok_or(anyhow!("Failed to get database connection"))?,
        )
        .await?
    {
        Ok(item)
    } else {
        let vo: document_file::ActiveModel = document_file::ActiveModel {
            id: NotSet,
            uploader: Set(uploader_id),
            permission: Set(Permission::Guest),
            hash: Set(hash),
            size: Set(size),
            file_name_raw: Set(file_name_raw),
        };
        let ret: document_file::Model = vo
            .insert(
                DB_CONN
                    .get()
                    .ok_or(anyhow!("Failed to get database connection"))?,
            )
            .await?;
        insert_database_log(
            super::TABLE_NAME,
            DatabaseLogItemOperation::Insert(ret.id),
            uploader_id,
        )?;
        Ok(ret)
    }
}

pub async fn delete(id: i64, operator: i64) -> Result<()> {
    document_file::Entity::delete_by_id(id)
        .exec(
            DB_CONN
                .get()
                .ok_or(anyhow!("Failed to get database connection"))?,
        )
        .await?;

    insert_database_log(
        super::TABLE_NAME,
        DatabaseLogItemOperation::Delete(id),
        operator,
    )?;
    Ok(())
}
