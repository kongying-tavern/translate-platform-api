use anyhow::Result;
use chrono::{DateTime, Utc};
use once_cell::sync::Lazy;
use std::collections::BTreeMap;

use crate::types::response::api::log::{DatabaseLogItem as VO, DatabaseLogItemOperation};

pub static DB: Lazy<sled::Db> = Lazy::new(|| {
    // TODO: 得想办法做成每天变换的，以便备份与清理缓存

    sled::open({
        let mut path = (*crate::LOG_DIR).clone();
        path.push("database");
        path
    })
    .unwrap()
});

// 不公开到外部路由，仅供内部调用
pub fn insert_database_log(
    table: impl ToString,
    operation: DatabaseLogItemOperation,
    operator: i64,
) -> Result<()> {
    let data = VO {
        time: Utc::now(),
        table: table.to_string(),
        operation,
        operator,
    };
    let key = Utc::now().to_rfc3339();
    let value = postcard::to_allocvec(&data)?;
    DB.insert(key, value)?;
    Ok(())
}

pub fn download_database_log(
    until: DateTime<Utc>,
    limit: usize,
) -> Result<BTreeMap<DateTime<Utc>, VO>> {
    let mut result = BTreeMap::new();
    for item in DB.range(..until.to_rfc3339()).rev().take(limit) {
        let (timestamp_raw, value) = item?;
        let timestamp = DateTime::parse_from_rfc3339(&String::from_utf8(timestamp_raw.to_vec())?)?
            .with_timezone(&Utc);
        let data: VO = postcard::from_bytes(&value)?;
        result.insert(timestamp, data);
    }
    Ok(result)
}
