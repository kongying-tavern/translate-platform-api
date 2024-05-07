mod error;
mod user;
use error::Result;
use user::register;

use tokio::net::{self, TcpListener};

#[tokio::main]
async fn main() -> Result<()> {
    println!("log begin"); // TODO: 之后需要换成log库
    let listener = TcpListener::bind("127.0.0.1:6379").await?;
    let mut net_tasks = Vec::new();
    loop {
        //这层循环捕捉链接会话
        let (mut stream, _) = listener.accept().await?;
        net_tasks.push(tokio::spawn(async move {
            let command = to_command(&mut stream).await;
            loop {
                match command {
                    Ok(Command::Register(_, _)) => {
                        let _ = register::register().await; //TODO: 处理注册错误发送到客户端
                        return ();
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

enum Command {
    Register(String, String), // email, password，一次收发结束
}
