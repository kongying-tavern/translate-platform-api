use anyhow::Result;

use axum::{
    extract::{Json, Path},
    response::IntoResponse,
};
use hyper::StatusCode;

use crate::utils::ExtractAuthInfo;
use _database::{
    functions::models::thread::{delete as do_delete, insert as do_insert, update as do_update},
    types::request::models::Thread as VO,
};

#[tracing::instrument]
pub async fn insert(
    ExtractAuthInfo(auth): ExtractAuthInfo,
    Json(vo): Json<VO>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    if let Some(auth) = auth {
        if !auth.is_admin() {
            return Err((StatusCode::FORBIDDEN, "No permission".to_string()));
        }

        let ret = do_insert(vo, auth.id)
            .await
            .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?;
        return Ok(Json(ret));
    }

    Err((StatusCode::FORBIDDEN, "No permission".to_string()))
}

#[tracing::instrument]
pub async fn update(
    ExtractAuthInfo(auth): ExtractAuthInfo,
    Path(id): Path<i64>,
    Json(vo): Json<VO>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    if let Some(auth) = auth {
        if !auth.is_admin() {
            return Err((StatusCode::FORBIDDEN, "No permission".to_string()));
        }

        let ret = do_update(id, vo, auth.id)
            .await
            .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?;
        return Ok(Json(ret));
    }

    Err((StatusCode::FORBIDDEN, "No permission".to_string()))
}

pub async fn delete(
    ExtractAuthInfo(auth): ExtractAuthInfo,
    Path(id): Path<i64>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    if let Some(auth) = auth {
        if !auth.is_admin() {
            return Err((StatusCode::FORBIDDEN, "No permission".to_string()));
        }

        do_delete(id, auth.id)
            .await
            .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?;
        return Ok(());
    }

    Err((StatusCode::FORBIDDEN, "No permission".to_string()))
}
