use actix_web::{get, web::Data, App, HttpServer, Responder};
use deadpool_postgres::{Manager, Pool};
use tokio_postgres::{Config, NoTls};

mod creat_table;
mod user;

#[get("/ping")]
async fn ping() -> impl Responder {
    "pong!"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 数据库配置

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
            .service(user::register)
            .service(ping)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
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
