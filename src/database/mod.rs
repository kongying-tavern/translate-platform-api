use sea_orm::{ConnectOptions, ConnectionTrait, Database, DatabaseConnection, DbErr};
use std::time::Duration;
use tracing::{error, info};

/// 数据库连接工厂
pub struct DatabaseConfig {
    url: String,
    max_connections: u32,
    min_connections: u32,
    connect_timeout: Duration,
    idle_timeout: Duration,
    max_lifetime: Duration,
}

impl DatabaseConfig {
    /// 创建新的数据库配置构建器
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置数据库连接URL
    pub fn url<S: Into<String>>(mut self, url: S) -> Self {
        self.url = url.into();
        self
    }

    /// 设置最大连接数
    pub fn max_connections(mut self, max: u32) -> Self {
        self.max_connections = max;
        self
    }

    /// 设置最小连接数
    pub fn min_connections(mut self, min: u32) -> Self {
        self.min_connections = min;
        self
    }

    /// 设置连接超时时间
    pub fn connect_timeout(mut self, timeout: Duration) -> Self {
        self.connect_timeout = timeout;
        self
    }

    /// 设置空闲超时时间
    pub fn idle_timeout(mut self, timeout: Duration) -> Self {
        self.idle_timeout = timeout;
        self
    }

    /// 设置连接最大生命周期
    pub fn max_lifetime(mut self, lifetime: Duration) -> Self {
        self.max_lifetime = lifetime;
        self
    }

    /// 尝试初始化数据库连接
    pub async fn try_init(self) -> Result<DatabaseConnection, DbErr> {
        info!("正在连接数据库: {}", mask_password(&self.url));

        let mut opt = ConnectOptions::new(self.url);
        opt.max_connections(self.max_connections)
            .min_connections(self.min_connections)
            .connect_timeout(self.connect_timeout)
            .idle_timeout(self.idle_timeout)
            .max_lifetime(self.max_lifetime)
            .sqlx_logging(true) // 启用SQL日志
            .sqlx_logging_level(log::LevelFilter::Debug);

        match Database::connect(opt).await {
            Ok(db) => {
                info!("数据库连接成功建立");
                Ok(db)
            }
            Err(err) => {
                error!("数据库连接失败: {}", err);
                Err(err)
            }
        }
    }
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self {
            url: std::env::var("DATABASE_URL")
                .unwrap_or_else(|_| "postgresql://translate_user:translate_password@localhost:5432/translate_platform".to_string()),
            max_connections: 100,
            min_connections: 5,
            connect_timeout: Duration::from_secs(8),
            idle_timeout: Duration::from_secs(8),
            max_lifetime: Duration::from_secs(3600), // 1小时
        }
    }
}

/// 遮蔽数据库URL中的密码
fn mask_password(url: &str) -> String {
    if let Ok(parsed) = url::Url::parse(url) {
        let mut masked = parsed.clone();
        if parsed.password().is_some() {
            let _ = masked.set_password(Some("***"));
        }
        masked.to_string()
    } else {
        url.to_string()
    }
}

/// 测试数据库连接
pub async fn test_connection(db: &DatabaseConnection) -> Result<(), DbErr> {
    use sea_orm::Statement;

    info!("测试数据库连接...");

    let result = db
        .execute(Statement::from_string(
            sea_orm::DatabaseBackend::Postgres,
            "SELECT 1".to_string(),
        ))
        .await?;

    info!("数据库连接测试成功: {:?}", result);
    Ok(())
}
