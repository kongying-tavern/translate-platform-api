use anyhow::Result;
use once_cell::sync::Lazy;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use tokio::runtime::Handle;
use tracing::{info, instrument};

use crate::get_app_env;

static DB_CONNECTION: Lazy<DatabaseConnection> = Lazy::new(init_database);

pub fn get_ref() -> &'static DatabaseConnection {
    &DB_CONNECTION
}

pub fn get_clone() -> DatabaseConnection {
    DB_CONNECTION.clone()
}

/// 初始化数据库连接池
///
/// # Parameters
/// * `database_url` - 数据库连接字符串，格式：postgres://user:password@host:port/database
///
/// # Returns
/// * `Result<(), DbErr>` - 成功返回()，失败返回数据库错误
///
/// # Example
/// ```rust
/// use crate::database::init_database;
///
/// let database_url = "postgres://user:password@localhost:5432/mydb";
/// init_database(database_url).await?;
/// ```
#[instrument]
pub fn init_database() -> DatabaseConnection {
    let handle = Handle::current();
    let url = get_app_env();
    info!("数据库URL: {}", mask_password(&url));

    // 创建数据库连接
    let mut db = ConnectOptions::new(url);
    db.max_connections(10)
        .min_connections(1)
        .connect_timeout(std::time::Duration::from_secs(5))
        .idle_timeout(std::time::Duration::from_secs(60))
        .sqlx_logging(true);
    let db = handle.block_on(async {
        let db = Database::connect(db).await.expect("数据库连接失败");

        // 测试连接是否有效
        match db.ping().await {
            Ok(_) => {
                info!("数据库连接测试成功");
                db
            }
            Err(e) => {
                panic!("数据库连接测试失败: {}", e);
            }
        }
    });
    db
}

/// 屏蔽数据库URL中的密码信息（用于日志输出）
///
/// # Parameters
/// * `url` - 原始数据库URL
///
/// # Returns
/// * `String` - 屏蔽密码后的URL
fn mask_password(url: &str) -> String {
    if let Some(at_pos) = url.find('@') {
        if let Some(colon_pos) = url[..at_pos].rfind(':') {
            if let Some(protocol_end) = url.find("://") {
                let protocol_part = &url[..protocol_end + 3];
                let user_part = &url[protocol_end + 3..colon_pos];
                let host_part = &url[at_pos..];
                return format!("{}{}:****{}", protocol_part, user_part, host_part);
            }
        }
    }
    url.to_string()
}

/// 执行数据库健康检查
///
/// # Returns
/// * `Result<(), DbErr>` - 成功返回()，失败返回数据库错误
pub async fn health_check() -> Result<()> {
    DB_CONNECTION.ping().await?;
    Ok(())
}
