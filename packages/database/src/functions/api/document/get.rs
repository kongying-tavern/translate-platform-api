use anyhow::{anyhow, ensure, Result};
use bytes::Bytes;

use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

use crate::{models::*, types::response::api::AuthInfo, DB_CONN, DOCUMENT_RES_DIR};

pub async fn get_file(auth: AuthInfo, hash: String) -> Result<(String, Bytes)> {
    let item = document_file::Entity::find()
        .filter(document_file::Column::Hash.eq(hash))
        .one(
            DB_CONN
                .get()
                .ok_or(anyhow!("Failed to get database connection"))?,
        )
        .await?
        .ok_or(anyhow!("Cannot find the document"))?;

    // 检查权限等级
    if let Some(info) = auth {
        ensure!(info.permission >= item.permission, "No permission");
    }

    let path = DOCUMENT_RES_DIR.clone();
    let path = path.join(format!("{}.dat", item.hash,));

    let file = tokio::fs::read(path).await?;
    let file = Bytes::from(file);

    Ok((item.file_name_raw, file))
}
