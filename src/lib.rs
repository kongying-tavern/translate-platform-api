use axum::{
    extract::{DefaultBodyLimit, Extension}, Json, Router
};
use serde_json::{Value, json};
use std::{env, net::SocketAddr};
use tokio::net::TcpListener;
use tracing::{error, info};

pub mod api;
pub mod database;
pub mod entities;

// 实例
pub mod sys_user;

pub type Result<T> = anyhow::Result<T>;
pub type DB = sea_orm::DatabaseConnection;

#[derive(Clone)]
pub struct AppState(&'static DB);

pub async fn router() -> Result<Router> {
    let db = AppState(database::get_ref());
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
        .layer(Extension(db))
        .layer(DefaultBodyLimit::max(1024 * 1024 * 16)); // 16 MiB
        

    Ok(ret)
}

async fn health_check(Extension(AppState(db)): Extension<AppState>) -> Json<Value> {
    Json(json!({
        "status": match db.ping().await {
            Ok(_) => "ok",
            Err(e) => {
                error!("数据库连接失败: {}", e);
                "error"
            }
        },
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "version": env!("CARGO_PKG_VERSION")
    }))
}

pub fn get_db_env() -> String {
    let postgres_db = env::var("POSTGRES_DB").expect("POSTGRES_DB is not set");
    let postgres_user = env::var("POSTGRES_USER").expect("POSTGRES_USER is not set");
    let postgres_password = env::var("POSTGRES_PASSWORD").expect("POSTGRES_PASSWORD is not set");
    let postgres_host = env::var("POSTGRES_HOST").unwrap_or_else(|_| "localhost".to_string());
    let postgres_port = env::var("POSTGRES_PORT").unwrap_or_else(|_| "5432".to_string());
    let postgres_url = format!(
        "postgres://{}:{}@{}:{}/{}",
        postgres_user, postgres_password, postgres_host, postgres_port, postgres_db
    );
    postgres_url
}

pub fn get_app_env() -> String {
    env::var("APP_PORT").unwrap_or_else(|_| "3000".to_string())
}

type RouterService =
    axum::extract::connect_info::IntoMakeServiceWithConnectInfo<Router, SocketAddr>;

/// 启动服务器，is_test 为 true 时使用端口 0
pub async fn run(is_test: bool) -> anyhow::Result<(TcpListener, RouterService)> {
    let app_port = get_app_env();

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
