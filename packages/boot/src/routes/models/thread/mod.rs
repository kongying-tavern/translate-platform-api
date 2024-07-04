mod edit;
mod view;

use anyhow::Result;
use axum::{
    routing::{get, post},
    Router,
};

pub async fn route() -> Result<Router> {
    let router = Router::new()
        .route("/count", get(view::count))
        .route("/list", get(view::list))
        .route("/select/:id", get(view::select))
        .route("/insert", post(edit::insert))
        .route("/update/:id", post(edit::update))
        .route("/delete/:id", post(edit::delete));

    Ok(router)
}
