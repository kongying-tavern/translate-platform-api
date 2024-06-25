mod document;
mod global_config;
mod image;
mod thread;
mod user;

use anyhow::Result;
use serde::{Deserialize, Serialize};

use axum::{
    extract::Request,
    middleware::{self, Next},
    response::Response,
    Router,
};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};
use hyper::StatusCode;

use _database::functions::api::auth::verify;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PageArgs {
    pub offset: Option<usize>,
    pub limit: Option<usize>,
}

async fn auth_middleware(
    bearer: TypedHeader<Authorization<Bearer>>,
    request: Request,
    next: Next,
) -> Result<Response, (StatusCode, String)> {
    if let Err(err) = verify(bearer.token().to_string()).await {
        return Err((StatusCode::UNAUTHORIZED, format!("Unauthorized: {}", err)));
    }

    Ok(next.run(request).await)
}

pub async fn route() -> Result<Router> {
    let router = Router::new()
        .nest("/global_config", global_config::route().await?)
        .nest("/image", image::route().await?)
        .nest("/document", document::route().await?)
        .nest("/user", user::route().await?)
        .nest("/thread", thread::route().await?)
        .layer(middleware::from_fn(auth_middleware));

    Ok(router)
}
