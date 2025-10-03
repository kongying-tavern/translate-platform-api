use axum::{response::IntoResponse, Json, http::StatusCode};
use tracing::debug;

use crate::sys_user;

mod model;

// root:/api/v1/user
pub async fn router() -> anyhow::Result<axum::Router> {
    let ret = axum::Router::new()
        .route("/", axum::routing::get(query))
        .route("/", axum::routing::post(create))
        .route("/:id", axum::routing::put(update))
        .route("/:id", axum::routing::delete(delete));
    Ok(ret)
}

// #[tracing::instrument(skip(auth))]
#[tracing::instrument]
#[utoipa::path(
    get,
    path = "/api/v1/users",
    responses(
        (status = 200, description = "成功", body = model::QueryResponse),
        (status = 400, description = "请求错误"),
        (status = 401, description = "未授权"),
        (status = 500, description = "服务器错误"),
    ),
    // security(
    //     ("bearerAuth" = [])
    // )
)]
pub async fn query(
    Json(payload): Json<model::QueryRequest>,
    // ExtractAuthInfo(auth): ExtractAuthInfo
) -> Result<impl IntoResponse, (StatusCode, String)> {
    Ok(())
}

// #[tracing::instrument(skip(auth))]
#[tracing::instrument]
#[utoipa::path(
    post,
    path = "/api/v1/users",
    request_body = model::RegisterRequest,
    responses(
        (status = 201, description = "成功"),
        (status = 400, description = "请求错误", body = String),
        (status = 401, description = "未授权"),
        (status = 409, description = "用户已存在"),
        (status = 500, description = "服务器错误"),
    ),
)]
pub async fn create(
    Json(payload): Json<model::RegisterRequest>,
    // ExtractAuthInfo(auth): ExtractAuthInfo
) -> Result<impl IntoResponse, (StatusCode, String)> {
    if !sys_user::check_password(&payload.password) {
        debug!("密码格式错误: {}", payload.password);
        return Err((StatusCode::BAD_REQUEST, "密码格式错误".to_string()));
    }
    if !sys_user::check_username(&payload.name) {
        debug!("用户名格式错误: {}", payload.name);
        return Err((StatusCode::BAD_REQUEST, "用户名格式错误".to_string()));
    }
    if !sys_user::check_role(payload.role) {
        debug!("角色格式错误: {}", payload.role);
        return Err((StatusCode::BAD_REQUEST, "角色格式错误".to_string()));
    }

    // TODO: 数据库干活

    // TODO: 检查用户重复

    Ok(())
}

/// 更新用户
/// PUT /api/v1/users/:id
/// 请求体: 用户名(可选),密码(可选),角色(可选),偏好时区(可选),偏好语言(可选)
/// 返回: 成功或失败
// #[tracing::instrument(skip(auth))]
#[tracing::instrument]
pub async fn update(
    Json(payload): Json<model::UpdateRequest>,
    // ExtractAuthInfo(auth): ExtractAuthInfo
) -> Result<impl IntoResponse, (StatusCode, String)> {
    Ok(())
}

/// 删除用户
/// DELETE /api/v1/users/:id
/// 返回: 成功或失败
// #[tracing::instrument(skip(auth))]
#[tracing::instrument]
pub async fn delete(
    Json(payload): Json<model::DeleteRequest>,
    // ExtractAuthInfo(auth): ExtractAuthInfo
) -> Result<impl IntoResponse, (StatusCode, String)> {
    Ok(())
}
