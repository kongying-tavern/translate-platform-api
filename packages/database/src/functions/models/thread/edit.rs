use anyhow::{anyhow, Result};

use sea_orm::{ActiveModelTrait, EntityTrait};

use crate::{
    functions::api::log::database::insert_database_log,
    models::*,
    types::{request::models::Thread as VO, response::api::log::DatabaseLogItemOperation},
    DB_CONN,
};

pub async fn insert(vo: VO, operator: i64) -> Result<thread::Model> {
    let vo: thread::ActiveModel = vo.into();
    let ret: thread::Model = vo
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
    Ok(ret)
}

pub async fn update(id: i64, vo: VO, operator: i64) -> Result<thread::Model> {
    let mut vo: thread::ActiveModel = vo.into();
    vo.set(thread::Column::Id, id.into());

    let ret: thread::Model = vo
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
    Ok(ret)
}

pub async fn delete(id: i64, operator: i64) -> Result<()> {
    thread::Entity::delete_by_id(id)
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
