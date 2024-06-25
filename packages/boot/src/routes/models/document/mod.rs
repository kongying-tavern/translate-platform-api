mod edit;
mod view;

use anyhow::Result;
use axum::{
    extract::DefaultBodyLimit,
    routing::{get, post},
    Router,
};

pub async fn route() -> Result<Router> {
    let router = Router::new()
        .route("/count", get(view::count))
        .route("/list", get(view::list))
        .route("/select/:id", get(view::select))
        .route(
            "/insert",
            post(edit::insert).layer(DefaultBodyLimit::max(1024 * 1024 * 8)), // 8 MiB
        )
        .route("/delete/:id", post(edit::delete));

    Ok(router)
}
