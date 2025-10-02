use axum::serve;
use std::env;
use tokio::net::TcpListener;
use tracing_subscriber::util::SubscriberInitExt;
use translate_platform_api::run;

#[tokio::main(flavor = "multi_thread")]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .with_max_level(tracing::Level::INFO)
        .finish()
        .init();

    let postgres_db = env::var("POSTGRES_DB").expect("POSTGRES_DB is not set");
    let postgres_user = env::var("POSTGRES_USER").expect("POSTGRES_USER is not set");
    let postgres_password = env::var("POSTGRES_PASSWORD").expect("POSTGRES_PASSWORD is not set");
    let postgres_host = env::var("POSTGRES_HOST").unwrap_or_else(|_| "localhost".to_string());
    let postgres_port = env::var("POSTGRES_PORT").unwrap_or_else(|_| "5432".to_string());
    let postgres_url = format!(
        "postgres://{}:{}@{}:{}/{}",
        postgres_user, postgres_password, postgres_host, postgres_port, postgres_db
    );

    let app_port = env::var("APP_PORT").unwrap_or_else(|_| "3000".to_string());

    tracing::info!(
        postgres_url = %postgres_url,
        app_port = %app_port,
        "已加载环境变量"
    );

    let router = router()
        .await?
        .into_make_service_with_connect_info::<SocketAddr>();

    let listener = TcpListener::bind(format!("0.0.0.0:{}", app_port)).await?;

    serve(listener, router).await?;

    println!("Goodbye!");
    Ok(())
}
