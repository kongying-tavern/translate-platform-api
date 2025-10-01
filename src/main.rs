use tracing_subscriber::util::SubscriberInitExt;
use translate_platform_api::run;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .with_max_level(tracing::Level::INFO)
        .finish()
        .init();

    // 调用库中的run函数
    run().await?;

    println!("Goodbye!");
    Ok(())
}
