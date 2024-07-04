pub mod database;
pub mod user;

use anyhow::Result;

use axum::{routing::post, Router};

use _database::types::request::api::LogQueryArgs;

pub async fn route() -> Result<Router> {
    let router = Router::new()
        .route("/database", post(database::download_database_log))
        .route("/user", post(user::download_user_log));

    Ok(router)
}
