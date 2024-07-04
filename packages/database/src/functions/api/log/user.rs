use anyhow::Result;
use chrono::{DateTime, Utc};
use once_cell::sync::Lazy;
use std::{
    collections::BTreeMap,
    net::{IpAddr, Ipv4Addr},
};

use crate::types::response::api::log::{UserLogItem as VO, UserLogItemOperation};

pub static DB: Lazy<sled::Db> = Lazy::new(|| {
    // TODO: 得想办法做成每天变换的，以便备份与清理缓存

    sled::open({
        let mut path = (*crate::LOG_DIR).clone();
        path.push("user");
        path
    })
    .unwrap()
});

// 不公开到外部路由，仅供内部调用
pub fn insert_user_log(
    operation: UserLogItemOperation,
    ip: Option<IpAddr>,
    user_agent: Option<String>,
) -> Result<()> {
    let data = VO {
        time: Utc::now(),
        ip: ip.unwrap_or(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0))),
        user_agent: user_agent.unwrap_or_default(),
        operation,
    };

    let key = Utc::now().to_rfc3339();
    let value = postcard::to_allocvec(&data)?;
    DB.insert(key, value)?;
    Ok(())
}

pub fn download_user_log(
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
