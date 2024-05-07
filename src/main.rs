mod error;
use error::Result;

use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<()> {
    println!("log begin"); // TODO: 之后需要换成log库
    let listener = TcpListener::bind("127.0.0.1:6379").await?;
    let mut net_tasks = Vec::new();
    loop {
        let (stream, _) = listener.accept().await?;
        net_tasks.push(tokio::spawn(async move {
            let _handle = stream;
        }));
    }
}
