mod auth;
mod log;

use anyhow::Result;
use axum::Router;

pub async fn route() -> Result<Router> {
    let router = Router::new()
        .nest("/auth", auth::route().await?)
        .nest("/log", log::route().await?);

    Ok(router)
}
