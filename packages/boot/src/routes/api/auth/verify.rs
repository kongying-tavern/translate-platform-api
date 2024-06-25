use anyhow::Result;

use axum::response::IntoResponse;
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};
use hyper::{HeaderMap, StatusCode};

use crate::utils::ExtractIP;
use _database::{
    functions::api::{auth::verify as do_verify, log::user::insert_user_log},
    types::response::api::log::UserLogItemOperation,
};

#[tracing::instrument]
pub async fn verify(
    headers: HeaderMap,
    ExtractIP(ip): ExtractIP,
    bearer: TypedHeader<Authorization<Bearer>>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let user_agent = headers["user-agent"].to_str().map(|s| s.to_string()).ok();

    if let Ok(user_info) = do_verify(bearer.token().to_string()).await.map_err(|err| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Cannot verify: {}", err),
        )
    }) {
        insert_user_log(UserLogItemOperation::Verify(user_info.id), ip, user_agent).map_err(
            |err| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Cannot insert user log: {}", err),
                )
            },
        )?;
        return Ok(());
    }

    insert_user_log(UserLogItemOperation::VerifyFailed, ip, user_agent).map_err(|err| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Cannot insert user log: {}", err),
        )
    })?;
    Err((StatusCode::UNAUTHORIZED, "Cannot verify".to_string()))
}
