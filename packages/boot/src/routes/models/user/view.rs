use anyhow::Result;

use axum::{
    extract::{Json, Path, Query},
    response::IntoResponse,
};
use hyper::StatusCode;

use crate::routes::models::PageArgs;
use _database::functions::models::user::{count as do_count, list as do_list, select as do_select};

#[tracing::instrument]
pub async fn count() -> Result<impl IntoResponse, (StatusCode, String)> {
    let ret = do_count()
        .await
        .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?;
    Ok(Json(ret))
}

#[tracing::instrument]
pub async fn list(args: Query<PageArgs>) -> Result<impl IntoResponse, (StatusCode, String)> {
    let ret = do_list(args.offset.unwrap_or(0), args.limit.unwrap_or(10))
        .await
        .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?;
    Ok(Json(ret))
}

#[tracing::instrument]
pub async fn select(Path(id): Path<i64>) -> Result<impl IntoResponse, (StatusCode, String)> {
    let ret = do_select(id)
        .await
        .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?;
    Ok(Json(ret))
}
