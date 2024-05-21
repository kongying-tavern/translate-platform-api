#[tokio::test]
async fn test_postgresql_embedded() {
    use crate::create;
    use postgresql_embedded::PostgreSQL;
    use tokio_postgres::NoTls;

    let mut postgresql = PostgreSQL::default();
    postgresql.setup().await.unwrap();
    postgresql.start().await.unwrap();

    let database_name = "test";
    postgresql.create_database(database_name).await.unwrap();
    postgresql.database_exists(database_name).await.unwrap();
    // postgresql.drop_database(database_name).await?;

    // postgresql.stop().await

    let settings = postgresql.settings();

    let (client, connection) = tokio_postgres::connect(
        format!(
            "host={host} port={port} user={username} password={password}",
            host = settings.host,
            port = settings.port,
            username = settings.username,
            password = settings.password
        )
        .as_str(),
        NoTls,
    )
    .await
    .unwrap();
    tokio::spawn(async move {
        // 检查数据库连接是否成功，成功就创建表。这里多开一个线程是因为文档里就这么做的
        // 看起来，似乎文档不希望我们在除了检查链接错误之外使用connection
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        } else {
            // 创建用户表
            create::create_user_table(&client).await.unwrap();
        }
    });
}
