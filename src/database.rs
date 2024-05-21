//! 包含了所有逻辑中直接与数据库操作的方法

use crate::Command;

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

// /// 注册用户，msg只能是Command::Register变体
// async fn register_user(msg: Command, client: Client) {
//     assert!(matches!(msg, Command::Register(_, _)));
// }

/// 从postgresql_embedded::PostgreSQL连接到tokio_postgres::Client会检查连接是否成功，成功就返回Client
pub async fn connect(
    postgresql: &postgresql_embedded::PostgreSQL,
) -> Result<tokio_postgres::Client> {
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
        tokio_postgres::NoTls,
    )
    .await
    .unwrap();
    if let Err(e) = connection.await {
        Err(Error::FailAtConnectDB(e))
    } else {
        Ok(client)
    }
}

pub mod create {
    /// 创建用户表
    pub async fn create_user_table(client: &tokio_postgres::Client) -> super::Result<()> {
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
        Ok(())
    }
}

pub mod write {
    use super::*;
    /// 注册用户，msg只能是Command::Register变体
    pub async fn insert_user(client: &tokio_postgres::Client, msg: Command) -> super::Result<()> {
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
