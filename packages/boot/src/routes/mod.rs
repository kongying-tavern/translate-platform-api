mod api;
mod models;

use anyhow::Result;
use axum::Router;

pub async fn route() -> Result<Router> {
    let router = Router::new()
        .nest("/", api::route().await?)
        .nest("/", models::route().await?);

    Ok(router)
}
