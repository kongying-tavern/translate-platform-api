//! 包含了所有逻辑中直接与数据库操作的方法

use crate::Command;

type Result<T> = std::result::Result<T, Error>;

enum Error {
    Postgres(tokio_postgres::Error),
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

mod create {
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

mod write {
    use super::*;
    /// 注册用户，msg只能是Command::Register变体
    async fn insert_user(client: &tokio_postgres::Client, msg: Command) -> super::Result<()> {
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
    async fn update_user_password(
        client: &tokio_postgres::Client,
        email: &str,
        new_password: &str,
    ) -> super::Result<()> {
        unimplemented!("更新现有用户密码")
    }
}
