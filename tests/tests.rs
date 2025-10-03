use axum::serve;
use once_cell::sync::Lazy;
use std::net::SocketAddr;
use tokio::{spawn, sync::RwLock};
use tracing::debug;
use tracing_subscriber::util::SubscriberInitExt;

use translate_platform_api::{Result, run};

mod api;

static SERVER: Lazy<RwLock<Option<SocketAddr>>> = Lazy::new(|| RwLock::new(None));

/// 得到一个给测例用的 URL
async fn get_addr() -> Result<SocketAddr> {
    match *SERVER.read().await {
        Some(addr) => Ok(addr),
        None => {
            tracing_subscriber::fmt()
                .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
                .with_max_level(tracing::Level::INFO)
                .finish()
                .init();
            let (listener, make_service) = run(true).await?;
            let addr = listener.local_addr()?;
            spawn(serve(listener, make_service).into_future());
            debug!("服务器启动在: {}", addr);
            *SERVER.write().await = Some(addr);
            Ok(addr)
        }
    }
}
