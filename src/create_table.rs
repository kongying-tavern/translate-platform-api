//! 这里会编写一些初始化表的操作
//! 在初始化时检查表是否存在，如果不存在则创建表

use tokio_postgres::error::Error;

pub async fn create_user_table(client: &deadpool_postgres::Object) -> Result<(), Error> {
    client
        .execute(
            "CREATE TABLE IF NOT EXISTS user_table (
                id SERIAL PRIMARY KEY,
                username VARCHAR NOT NULL,
                password VARCHAR NOT NULL,
                email VARCHAR NOT NULL,
                create_time TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
            )",
            &[],
        )
        .await?;
    Ok(())
}
