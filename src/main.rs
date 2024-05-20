mod database;
mod error;
mod user;
use error::Result;
use postgresql_embedded::PostgreSQL;
use tokio_postgres::NoTls;
// use user::register; 完了，忘记之前自己写的时候怎么想的了
mod tests;
use tokio::{
    io::AsyncWriteExt,
    net::{self, TcpListener},
};

use database::{create, write};

#[tokio::main]
async fn main() -> Result<()> {
    println!("log begin"); // TODO: 之后需要换成log库

    // 初始化socket
    let listener = TcpListener::bind("127.0.0.1:6379").await?;
    let mut net_tasks = Vec::new();

    // 初始化数据库
    let mut postgresql = PostgreSQL::default();
    postgresql.setup().await.unwrap();
    postgresql.start().await.unwrap();

    let database_name = "main";
    postgresql.create_database(database_name).await.unwrap();
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
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        } else {
            create::create_user_table(&client).await.unwrap();
        }
    });

    loop {
        //这层循环捕捉链接会话
        let (mut stream, _) = listener.accept().await?;
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

        net_tasks.push(tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("数据库链接错误: {}", e);
                stream
                    .write_all(format!("数据库链接错误: {}", e).as_bytes())
                    .await
                    .unwrap();
            }
            let command = to_command(&mut stream).await;
            loop {
                match &command {
                    Ok(Command::Register(_, _)) => {
                        write::insert_user(&client, command.unwrap()).await.unwrap();
                        return ();
                    }
                    Ok(Command::Ping) => {
                        stream.write_all(b"pong\r\n").await.unwrap();
                    }
                    Err(_) => unimplemented!("将错误抛给客户端"),
                }
            }
            //这层循环保持和一个客户端会话的收发信
        }));
    }
}

async fn to_command(_stream: &mut net::TcpStream) -> Result<Command> {
    unimplemented!("解析客户端发来的数据，解析为具体的命令")
}

pub enum Command {
    /// Ping，用于测试连接
    Ping,
    /// email, password
    Register(String, String),
}
