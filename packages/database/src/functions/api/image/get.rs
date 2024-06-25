use anyhow::{anyhow, ensure, Result};
use bytes::Bytes;

use image::ImageFormat;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

use crate::{models::*, types::response::api::AuthInfo, DB_CONN, IMAGE_RES_DIR};

pub async fn get_redirect_hash(hash: String) -> Result<String> {
    let config = global_config::Entity::find()
        .filter(global_config::Column::Family.eq("图片别名"))
        .filter(global_config::Column::Label.eq(hash))
        .one(
            DB_CONN
                .get()
                .ok_or(anyhow!("Failed to get database connection"))?,
        )
        .await?
        .ok_or(anyhow!("Cannot find the config"))?;

    Ok(config.value)
}

pub async fn get_file(auth: AuthInfo, hash: String) -> Result<(String, Bytes)> {
    let item = image_file::Entity::find()
        .filter(image_file::Column::Hash.eq(hash))
        .one(
            DB_CONN
                .get()
                .ok_or(anyhow!("Failed to get database connection"))?,
        )
        .await?
        .ok_or(anyhow!("Cannot find the image"))?;

    // 检查权限等级
    if let Some(info) = auth {
        ensure!(info.permission >= item.permission, "No permission");
    }

    let mime = ImageFormat::from_mime_type(&item.mime)
        .ok_or(anyhow!("Failed to get MIME type from image"))?;
    let path = IMAGE_RES_DIR.clone();
    let path = path.join(format!(
        "{}.{}",
        item.hash,
        mime.extensions_str().first().ok_or(anyhow!(
            "Failed to get extension from MIME type: {}",
            mime.to_mime_type()
        ))?
    ));

    let file = tokio::fs::read(path).await?;
    let file = Bytes::from(file);

    Ok((item.mime, file))
}
