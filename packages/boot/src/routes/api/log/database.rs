use anyhow::Result;

use axum::{extract::Json, response::IntoResponse};
use hyper::StatusCode;

use super::LogQueryArgs;
use crate::utils::ExtractAuthInfo;
use _database::functions::api::log::database::download_database_log as download_log;

#[tracing::instrument]
pub async fn download_database_log(
    ExtractAuthInfo(auth): ExtractAuthInfo,
    Json(params): Json<LogQueryArgs>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    if !auth
        .ok_or((
            StatusCode::INTERNAL_SERVER_ERROR,
            "No permission".to_string(),
        ))?
        .is_admin()
    {
        return Err((StatusCode::FORBIDDEN, "No permission".to_string()));
    }

    let result = download_log(params.until, params.limit).map_err(|err| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to download database log: {}", err),
        )
    })?;

    Ok(Json(result))
}
