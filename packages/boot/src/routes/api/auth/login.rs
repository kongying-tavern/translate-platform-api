use anyhow::Result;
use chrono::{DateTime, Utc};
use once_cell::sync::Lazy;
use std::{
    net::SocketAddr,
    sync::{Arc, Mutex},
};

use axum::{
    extract::{ConnectInfo, Json},
    response::IntoResponse,
};
use hyper::{HeaderMap, StatusCode};

use crate::utils::ExtractIP;
use _database::{
    functions::api::{auth::login as do_login, log::user::insert_user_log},
    types::{
        request::api::LoginInfo,
        response::api::log::{UserLogItemLoginFailedReason, UserLogItemOperation},
    },
};

type LogItem = (SocketAddr, DateTime<Utc>);
static LOGIN_CHECK_CACHE: Lazy<Arc<Mutex<Vec<LogItem>>>> =
    Lazy::new(|| Arc::new(Mutex::new(Vec::new())));

#[tracing::instrument]
pub async fn login(
    headers: HeaderMap,
    ConnectInfo(real_ip): ConnectInfo<SocketAddr>,
    ExtractIP(ip): ExtractIP,
    args: Json<LoginInfo>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let user_agent = headers["user-agent"].to_str().map(|s| s.to_string()).ok();

    // Write the login log
    let now = Utc::now();
    LOGIN_CHECK_CACHE
        .lock()
        .map_err(|err| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Cannot lock login log: {}", err),
            )
        })?
        .push((real_ip, now));

    // Clear the login log that is older than 1 day
    LOGIN_CHECK_CACHE
        .lock()
        .map_err(|err| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Cannot lock login log: {}", err),
            )
        })?
        .retain(|(_, time)| now.signed_duration_since(*time).num_hours() < 24);

    // Check if the user is trying to login too frequently
    // Limit to 3 times per day
    let count = LOGIN_CHECK_CACHE
        .lock()
        .map_err(|err| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Cannot lock login log: {}", err),
            )
        })?
        .iter()
        .filter(|(ip, time)| ip == &real_ip && now.signed_duration_since(*time).num_hours() < 24)
        .count();
    if count > 3 {
        insert_user_log(
            UserLogItemOperation::LoginFailed(UserLogItemLoginFailedReason::TooManyAttempts),
            ip,
            user_agent,
        )
        .map_err(|err| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Cannot insert user log: {}", err),
            )
        })?;
        return Err((
            StatusCode::TOO_MANY_REQUESTS,
            "Too many requests".to_string(),
        ));
    }

    if let Ok(ret) = do_login(args.name.clone(), args.password_raw.clone())
        .await
        .map_err(|err| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Cannot login: {}", err),
            )
        })
    {
        insert_user_log(UserLogItemOperation::Login(ret.id), ip, user_agent.clone()).map_err(
            |err| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Cannot insert user log: {}", err),
                )
            },
        )?;
        return Ok(Json(ret));
    }

    insert_user_log(
        UserLogItemOperation::LoginFailed(UserLogItemLoginFailedReason::PasswordWrong),
        ip,
        user_agent.clone(),
    )
    .map_err(|err| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Cannot insert user log: {}", err),
        )
    })?;
    Err((StatusCode::UNAUTHORIZED, "Wrong password".to_string()))
}
