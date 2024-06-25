use anyhow::{anyhow, Result};

use sea_orm::{ActiveModelTrait, EntityTrait};

use crate::{
    functions::api::log::database::insert_database_log,
    models::*,
    types::{request::models::User as VO, response::api::log::DatabaseLogItemOperation},
    DB_CONN,
};

pub async fn insert(vo: VO, operator: i64) -> Result<VO> {
    let vo: user::ActiveModel = vo.into();
    let ret: user::Model = vo
        .insert(
            DB_CONN
                .get()
                .ok_or(anyhow!("Failed to get database connection"))?,
        )
        .await?;

    insert_database_log(
        super::TABLE_NAME,
        DatabaseLogItemOperation::Insert(ret.id),
        operator,
    )?;
    Ok(ret.into())
}

pub async fn update(id: i64, vo: VO, operator: i64) -> Result<VO> {
    let mut vo: user::ActiveModel = vo.into();
    vo.set(user::Column::Id, id.into());

    let ret: user::Model = vo
        .update(
            DB_CONN
                .get()
                .ok_or(anyhow!("Failed to get database connection"))?,
        )
        .await?;

    insert_database_log(
        super::TABLE_NAME,
        DatabaseLogItemOperation::Update(ret.id),
        operator,
    )?;
    Ok(ret.into())
}

pub async fn delete(id: i64, operator: i64) -> Result<()> {
    user::Entity::delete_by_id(id)
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
