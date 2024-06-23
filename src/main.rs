use actix_web::{
    web::{self, Data},
    App, HttpServer, Responder,
};
use actix_web_lab::middleware;
use deadpool_postgres::{Manager, Pool};
use serde::Serialize;
use tokio_postgres::{Config, NoTls};
use user::jwt;

mod creat_table;
mod user;

#[actix_web::get("/ping")]
async fn ping() -> impl Responder {
    "pong!"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 数据库配置
    // TODO: 别忘记配置线程数
    let db_manager = Manager::new(
        Config::new()
            .host("localhost")
            .user("postgres")
            .password("dev_password")
            .to_owned(),
        NoTls,
    );
    // TODO: 这里的池大小最好也从配置文件中读取
    let pool = Pool::builder(db_manager).max_size(16).build().unwrap();

    creat_table::create_user_table(&pool.get().await.unwrap())
        .await
        .unwrap();

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(pool.clone()))
            .service(
                web::scope("/user")
                    .wrap(middleware::from_fn(jwt::verify_jwt))
                    .service(user::register),
            )
            .service(ping)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

/// 所有响应的返回格式
/// REVIEW: 大家看看这样写成不？
/// 计划error_code使用树形编码，一个十位数，0-255，最低两位（都是十位数）是业务类型，在两位要么是子分类或者是具体错误
/// * 00: 成功，其他位也是0
/// * 01: user相关操作错误
/// 具体错误类型见ERRORLIST.md(还没写)
/// REVIEW: 要不要换个名字？
#[derive(Serialize)]
struct ResJson<T> {
    error_flag: bool,
    error_code: u8,
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
