use actix_web::{
    web::{self, Data},
    App, HttpServer, Responder,
};
use actix_web_lab::middleware;
use chrono::{DateTime, Duration, Utc};
use migration::{Migrator, MigratorTrait};
use sea_orm::SqlxPostgresConnector;
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgPoolOptions;
use user::{jwt, login, register};

mod entity;
mod user;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 数据库初始化
    // TODO: 之后需要从args中导出
    let config = PgPoolOptions::new()
        .max_connections(128)
        .min_connections(16)
        .acquire_timeout(Duration::seconds(8).to_std().unwrap())
        .idle_timeout(Duration::seconds(8).to_std().unwrap())
        .max_lifetime(Duration::seconds(8).to_std().unwrap());

    let pool = config
        .connect("postgres://postgres:dev_password@localhost:5432")
        .await
        .unwrap();

    let db = SqlxPostgresConnector::from_sqlx_postgres_pool(pool.clone());

    // 创建表
    Migrator::up(&db, None).await.unwrap();

    // log初始化
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_test_writer()
        .init();

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new([db.clone()])) // 这里必须要用一个类型包起来，不然传参会报错，所以用数组吧
            .service(
                web::scope("/user")
                    .wrap(middleware::from_fn(jwt::mw_verify_jwt))
                    .service(register::sv_register),
            )
            .service(login::sv_login)
            .service(ping)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

#[actix_web::get("/ping")]
async fn ping() -> impl Responder {
    "pong!"
}

/// 所有响应的返回格式
/// 计划error_code使用树形编码，一个十进制数，最低两位是业务分类，往前两位要么是子分类要么是具体错误类型
/// * 00: 成功，其他位也是0
/// * 01: user相关操作错误
/// 具体错误类型见ERRORLIST.md(还没写)
/// 其中只有含有低两位的错误为服务器错误，这种情况下，程序理论上应该抛出panic的地方但为了让前端知晓所以还是返回了
#[derive(Serialize)]
struct ResJson<T> {
    error_flag: bool,
    error_code: u16,
    data: Option<T>,
}

impl<T: Serialize> ResJson<T> {
    fn new(content: T) -> Self {
        Self {
            error_flag: false,
            error_code: 0,
            data: Some(content),
        }
    }
}

/// 通用字段
#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
struct UniversalField {
    /// ID为数据库自增，不会进入迭代器
    id: i32,
    /// 乐观锁
    version: u32,
    /// 创建人
    create_by: Option<u64>,
    /// 创建时间
    create_time: Option<DateTime<Utc>>,
    /// 更新人
    update_by: Option<u64>,
    /// 更新时间
    update_time: Option<DateTime<Utc>>,
    /// 是否删除，默认为false
    del_flag: bool,
}

impl IntoIterator for UniversalField {
    type Item = Option<String>;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        vec![
            Some(self.version.to_string()),
            self.create_by.map(|id| id.to_string()),
            self.create_time
                .map(|time| time.timestamp_millis().to_string()),
            self.update_by.map(|id| id.to_string()),
            self.update_time
                .map(|time| time.timestamp_millis().to_string()),
            Some(self.del_flag.to_string()),
        ]
        .into_iter()
    }
}

/// 服务器错误
/// 一些应该panic的地方为了能让前端知道，就用这个
#[derive(Debug)]
enum Error {
    /// 服务器逻辑错误
    ServerLogicError,
    /// 数据库连接失败
    DatabaseConnectionFailed,
}
