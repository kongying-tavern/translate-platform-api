use tracing_subscriber::util::SubscriberInitExt;
use translate_platform_api::run;
use axum::serve;

#[tokio::main(flavor = "multi_thread")]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .with_max_level(tracing::Level::INFO)
        .finish()
        .init();

    let (listener, router) = run(false).await?;
    serve(listener, router).await?;

    println!("Goodbye!");
    Ok(())
}
