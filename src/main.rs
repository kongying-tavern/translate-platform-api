use actix_web::{
    web::{self, Data},
    App, HttpServer, Responder,
};
use actix_web_lab::middleware;
use chrono::{DateTime, Local};

use diesel::r2d2;
use serde::{Deserialize, Serialize};
use user::{jwt, register};

mod schema;
mod user;

pub fn get_connection_pool() -> r2d2::Pool<r2d2::ConnectionManager<diesel::PgConnection>> {
    let url = "postgres://postgres:dev_password@localhost";
    let manager = r2d2::ConnectionManager::<diesel::PgConnection>::new(url);
    // Refer to the `r2d2` documentation for more methods to use
    // when building a connection pool
    r2d2::Pool::builder()
        .test_on_check_out(true)
        .build(manager)
        .expect("Could not build connection pool")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = get_connection_pool();

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(pool.clone()))
            .service(
                web::scope("/user")
                    .wrap(middleware::from_fn(jwt::mw_verify_jwt))
                    .service(register::sv_register),
            )
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
/// REVIEW: 大家看看这样写成不？
/// 计划error_code使用树形编码，一个十进制数，最低两位是业务分类，往前两位要么是子分类要么是具体错误类型
/// * 00: 成功，其他位也是0
/// * 01: user相关操作错误
/// 具体错误类型见ERRORLIST.md(还没写)
/// 其中只有含有低两位的错误为服务器错误，这种情况下，程序理论上应该抛出panic的地方但为了让前端知晓所以还是返回了
/// REVIEW: 要不要换个名字？
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
#[derive(Serialize, Deserialize, Debug)]
struct UniversalField {
    id: usize,
    /// 乐观锁
    version: usize,
    /// 创建人
    create_by: Option<usize>,
    /// 创建时间
    create_time: Option<DateTime<Local>>,
    /// 更新人
    update_by: Option<usize>,
    /// 更新时间
    update_time: Option<DateTime<Local>>,
    /// 是否删除，默认为false
    del_flag: bool,
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

#[actix_web::test]
/// 测试正常访问postgres
async fn test_tokio_postgres() {
    use actix_web::rt;
    use tokio_postgres::NoTls;
    // TODO：之后将密码设置为读取内置文件填写，开发阶段就先这样吧
    let (client, connection) =
        tokio_postgres::connect("host=localhost user=postgres password=dev_password", NoTls)
            .await
            .unwrap();

    rt::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    // Now we can execute a simple statement that just returns its parameter.
    let rows = client
        .query("SELECT $1::TEXT", &[&"hello world"])
        .await
        .unwrap();

    // And then check that we got back the same string we sent over.
    let value: &str = rows[0].get(0);
    assert_eq!(value, "hello world");
}
