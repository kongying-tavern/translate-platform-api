use axum::{Router, extract::DefaultBodyLimit};
use std::{env, net::SocketAddr};
use tokio::net::TcpListener;
use tracing::info;

pub mod api;
pub mod database;
pub mod entities;

// 实例
pub mod sys_user;

pub type Result<T> = anyhow::Result<T>;

pub async fn router() -> Result<Router> {
    let ret = Router::new()
        // OpenAPI 文档路由
        .route("/", axum::routing::get(|| async { "翻译平台 API" }))
        .route("/health", axum::routing::get(health_check))
        // .route("/openapi.json", axum::routing::get(openapi_spec)) // ai 说的，但没实现
        // .route("/docs", axum::routing::get(swagger_ui))
        .nest("/api", api::router().await?)
        // .fallback(|| async { (StatusCode::NOT_IMPLEMENTED, "功能还为实现").into_response() })
        // .layer(from_extractor::<crate::middlewares::ExtractUserAgent>())
        // .layer(from_extractor::<crate::middlewares::ExtractIP>())
        .layer(DefaultBodyLimit::max(1024 * 1024 * 16)); // 16 MiB

    Ok(ret)
}

async fn health_check() -> axum::Json<serde_json::Value> {
    axum::Json(serde_json::json!({
        "status": "ok",
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "version": env!("CARGO_PKG_VERSION")
    }))
}

pub fn get_env() -> (String, String) {
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

    info!(
        postgres_url = %postgres_url,
        app_port = %app_port,
        "已加载环境变量"
    );

    (postgres_url, app_port)
}

type RouterService =
    axum::extract::connect_info::IntoMakeServiceWithConnectInfo<Router, SocketAddr>;

/// 启动服务器，is_test 为 true 时使用端口 0
pub async fn run(is_test: bool) -> anyhow::Result<(TcpListener, RouterService)> {
    let (_postgres_url, app_port) = get_env();

    let router = router()
        .await?
        .into_make_service_with_connect_info::<SocketAddr>();

    let listener = TcpListener::bind(format!(
        "0.0.0.0:{}",
        if is_test { "0" } else { app_port.as_str() }
    ))
    .await?;
    let addr = listener.local_addr()?;
    info!("服务器启动在: {}", addr);

    Ok((listener, router))
}
