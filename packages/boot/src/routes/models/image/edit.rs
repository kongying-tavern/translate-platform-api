use anyhow::Result;

use axum::{
    extract::{Json, Multipart, Path},
    response::IntoResponse,
};
use hyper::StatusCode;

use crate::utils::ExtractAuthInfo;
use _database::functions::models::image::{delete as do_delete, insert as do_insert};

#[tracing::instrument]
pub async fn insert(
    ExtractAuthInfo(auth): ExtractAuthInfo,
    mut multipart: Multipart,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    if let Some(auth) = auth {
        if !auth.is_admin() {
            return Err((StatusCode::FORBIDDEN, "No permission".to_string()));
        }

        let data = multipart
            .next_field()
            .await
            .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?
            .ok_or((StatusCode::BAD_REQUEST, "No file".to_string()))?;
        let data = data
            .bytes()
            .await
            .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?;

        let ret = do_insert(auth.id, data)
            .await
            .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?;
        Ok(Json(ret))
    } else {
        Err((StatusCode::FORBIDDEN, "No permission".to_string()))
    }
}

#[tracing::instrument]
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
