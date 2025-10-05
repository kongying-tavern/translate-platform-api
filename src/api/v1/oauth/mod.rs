use axum::{Extension, Json, http::StatusCode, response::IntoResponse};
use tracing::error;

use crate::{
    api::v1::{oauth::db::{delete_token, refresh_token, verify_user, DbError}, users::utils::check_field}, sys_user, AppState
};

pub mod db;
pub mod model;

// root:/api/v1/oauth
pub async fn router() -> anyhow::Result<axum::Router> {
    let ret = axum::Router::new()
        .route("/login", axum::routing::post(login))
        .route("/refresh", axum::routing::post(refresh))
        .route("/logout", axum::routing::post(logout))
        ;
    Ok(ret)
}

#[tracing::instrument]
#[utoipa::path(
    post,
    path = "/api/v1/oauth/login",
    description = "登陆",
    request_body = model::LoginRequest,
    responses(
        (status = 201, description = "成功", body = model::LoginAndRefreshResponse),
        (status = 400, description = "请求错误", body = String),
        (status = 500, description = "服务器错误"),
    ),
)]
pub async fn login(
    Extension(AppState(db)): Extension<AppState>,
    Json(payload): Json<model::LoginRequest>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let mut res = String::new();

    check_field(&payload.name, sys_user::check_name, "用户名", &mut res);
    check_field(
        &payload.password,
        sys_user::check_password,
        "密码",
        &mut res,
    );

    if !res.is_empty() {
        return Err((StatusCode::BAD_REQUEST, res));
    }

    match verify_user(payload, db).await {
        Ok(res) => Ok((StatusCode::OK, Json(res))),
        Err(e) => {
            error!("登陆出错: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "数据库插入错误".to_string(),
            ))
        }
    }
}

#[tracing::instrument]
#[utoipa::path(
    post,
    path = "/api/v1/oauth/refresh",
    description = "刷新令牌",
    request_body = model::RefreshTokenRequest,
    responses(
        (status = 201, description = "成功", body = model::LoginAndRefreshResponse),
        (status = 404, description = "未找到"),
        (status = 500, description = "服务器错误"),
    ),
)]
pub async fn refresh(
    Json(payload): Json<model::RefreshTokenRequest>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    match refresh_token(payload.refresh_token).await {
        Ok(res) => Ok((StatusCode::OK, Json(res))),
        Err(DbError::NotFound) => Err((StatusCode::NOT_FOUND, "".to_string())),
        Err(e) => {
            error!("刷新令牌出错: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "数据库插入错误".to_string(),
            ))
        }
    }
}

#[tracing::instrument]
#[utoipa::path(
    post,
    path = "/api/v1/oauth/logout",
    description = "登出",
    request_body = model::RefreshTokenRequest,
    responses(
        (status = 201, description = "成功"),
        (status = 404, description = "未找到"),
        (status = 500, description = "服务器错误"),
    ),
)]
pub async fn logout(
    Json(payload): Json<model::RefreshTokenRequest>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    match delete_token(payload.refresh_token).await {
        Ok(_) => Ok((StatusCode::OK, "")),
        Err(DbError::NotFound) => Err((StatusCode::NOT_FOUND, "".to_string())),
        Err(e) => {
            error!("登出出错: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "数据库插入错误".to_string(),
            ))
        }
    }
}