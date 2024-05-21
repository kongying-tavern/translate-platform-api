mod database;
mod error;
// mod user;
use error::Result;
// use user::register; 完了，忘记之前自己写的时候怎么想的了
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
    let postgresql = database::build("main").await;
    let client = database::connect(&postgresql).await.unwrap();

    // 初始化表，目前只有用户表，后面有PDManger再说
    create::create_user_table(&client).await.unwrap();
    let mut database_init_count = 0;

    loop {
        //这层循环捕捉链接会话，理论上会在链接到来之前准备好数据库对象
        let client = match database::connect(&postgresql).await {
            Ok(client) => client,
            Err(e) => {
                eprintln!("数据库连接失败: {:?}", e);
                database_init_count += 1;
                if database_init_count <= 10 {
                    continue;
                } else {
                    panic!("数据库连接失败次数过多，程序退出")
                }
            }
        };
        let (mut stream, _) = listener.accept().await?;

        net_tasks.push(tokio::spawn(async move {
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
