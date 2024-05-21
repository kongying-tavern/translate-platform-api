//! 包含了所有逻辑中直接与数据库操作的方法

use crate::Command;
use log::debug;
use postgresql_embedded::PostgreSQL;
type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Postgres(tokio_postgres::Error),
    FailAtConnectDB(tokio_postgres::Error),
    WrongCommand,
}

impl From<tokio_postgres::Error> for Error {
    fn from(value: tokio_postgres::Error) -> Self {
        Error::Postgres(value)
    }
}

/// 使用嵌入式postgres创建数据库，包括一些基础的检查
pub async fn build(database_name: &str) -> postgresql_embedded::PostgreSQL {
    let mut postgresql = PostgreSQL::default();
    postgresql.setup().await.unwrap();
    postgresql.start().await.unwrap();

    if !postgresql.database_exists(database_name).await.unwrap() {
        debug!("新建数据库: {}", database_name);
        postgresql.create_database(database_name).await.unwrap();
    };

    postgresql
}

/// 从postgresql_embedded::PostgreSQL连接到tokio_postgres::Client会检查连接是否成功，成功就返回Client
pub async fn connect(
    postgresql: &postgresql_embedded::PostgreSQL,
) -> Result<tokio_postgres::Client> {
    let settings = postgresql.settings();

    debug!("连接到数据库: {:?}", settings);

    let (client, connection) = tokio_postgres::connect(
        format!(
            "host={host} port={port} user={username} password={password}",
            host = settings.host,
            port = settings.port,
            username = settings.username,
            password = settings.password
        )
        .as_str(),
        tokio_postgres::NoTls,
    )
    .await
    .unwrap();
    if let Err(e) = connection.await {
        Err(Error::FailAtConnectDB(e))
    } else {
        debug!("连接成功");
        Ok(client)
    }
}

#[tokio::test]
async fn test_postgresql_embedded() {
    // 初始化数据库
    let postgresql = build("main").await;
    connect(&postgresql).await.unwrap();
}

pub mod create {
    use super::*;
    /// 创建用户表
    pub async fn create_user_table(client: &tokio_postgres::Client) -> Result<()> {
        client
            .execute(
                "CREATE TABLE IF NOT EXISTS users (
                    id SERIAL PRIMARY KEY,
                    email VARCHAR(255) NOT NULL UNIQUE,
                    password VARCHAR(255) NOT NULL
                )",
                &[],
            )
            .await?;
        debug!("创建用户表成功");
        Ok(())
    }
}

pub mod write {
    use super::*;
    /// 注册用户，msg只能是Command::Register变体
    pub async fn insert_user(client: &tokio_postgres::Client, msg: Command) -> Result<()> {
        match msg {
            Command::Register(email, password) => {
                client
                    .execute(
                        "INSERT INTO users (email, password) VALUES ($1, $2)",
                        &[&email, &password],
                    )
                    .await?;
                Ok(())
            }
            _ => Err(super::Error::WrongCommand),
        }
    }

    // 更新现有用户密码函数
    async fn _update_user_password() -> super::Result<()> {
        unimplemented!("更新现有用户密码")
    }
}
