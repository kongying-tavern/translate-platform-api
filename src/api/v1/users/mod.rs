use axum::{Extension, Json, http::StatusCode, response::IntoResponse};
use tracing::{debug, error};

use crate::{
    AppState,
    api::v1::users::db::{DeleteError, delete_user},
    sys_user,
};
use db::{check_name_exists, insert_user};
use utils::{check_field, check_option};

pub mod db;
pub mod model;
pub mod utils;

// root:/api/v1/user
pub async fn router() -> anyhow::Result<axum::Router> {
    let ret = axum::Router::new()
        .route("/", axum::routing::get(query))
        .route("/", axum::routing::post(create))
        .route("/", axum::routing::put(update))
        .route("/", axum::routing::delete(delete));
    Ok(ret)
}

#[tracing::instrument]
#[utoipa::path(
    get,
    path = "/api/v1/users",
    description = "查询用户",
    request_body = model::QueryRequest,
    responses(
        (status = 200, description = "成功", body = model::QueryResponse),
        (status = 400, description = "请求错误", body = String),
        (status = 401, description = "未授权"),
        (status = 500, description = "服务器错误"),
    ),
)]
pub async fn query(
    Json(payload): Json<model::QueryRequest>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let mut res = String::new();
    if payload.page < 1 || payload.per_page < 1 || payload.per_page > 100 {
        debug!(
            "分页参数错误: page={}, per_page={}",
            payload.page, payload.per_page
        );
        res.push_str("分页参数错误\n");
    }
    check_option(&payload.name, sys_user::check_name, "用户名", &mut res);
    check_option(&payload.role, sys_user::check_role, "角色", &mut res);
    check_option(&payload.locale, sys_user::check_lang, "语言", &mut res);
    check_option(&payload.timezone, sys_user::check_tz, "时区", &mut res);

    if !res.is_empty() {
        return Err((StatusCode::BAD_REQUEST, res));
    }
    // TODO: 查询数据库，构造响应
    let response = model::QueryResponse::default();
    Ok((StatusCode::OK, Json(response)))
}

#[tracing::instrument]
#[utoipa::path(
    post,
    path = "/api/v1/users",
    description = "创建用户",
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
    Extension(AppState(db)): Extension<AppState>,
    Json(payload): Json<model::RegisterRequest>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let mut res = String::new();

    match check_name_exists(&payload.name, db).await {
        Ok(exists) => {
            if exists {
                res.push_str("用户名已存在\n");
            }
        }
        Err(e) => {
            error!("检查用户名是否存在时出错: {}", e);
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "数据库查询错误".to_string(),
            ));
        }
    }

    check_field(&payload.name, sys_user::check_name, "用户名", &mut res);
    check_field(
        &payload.password,
        sys_user::check_password,
        "密码",
        &mut res,
    );
    check_field(&payload.role, sys_user::check_role, "角色", &mut res);
    check_field(&payload.timezone, sys_user::check_tz, "时区", &mut res);
    check_field(&payload.locale, sys_user::check_lang, "语言", &mut res);

    if !res.is_empty() {
        return Err((StatusCode::BAD_REQUEST, res));
    }

    match insert_user(payload, db).await {
        Ok(_) => {}
        Err(e) => {
            error!("插入用户时出错: {}", e);
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "数据库插入错误".to_string(),
            ));
        }
    }

    Ok((StatusCode::CREATED, ""))
}

#[tracing::instrument]
#[utoipa::path(
    put,
    path = "/api/v1/users",
    description = "更新用户数据",
    request_body = model::RegisterRequest,
    responses(
        (status = 200, description = "成功"),
        (status = 400, description = "请求错误", body = String),
        (status = 401, description = "未授权"),
        (status = 409, description = "用户不存在"),
        (status = 500, description = "服务器错误"),
    ),
)]
pub async fn update(
    Json(payload): Json<model::UpdateRequest>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let mut res = String::new();
    if !sys_user::check_id(&payload.id) {
        debug!("用户ID格式错误: {}", payload.id);
        res.push_str("用户ID格式错误\n");
    }

    check_field(&payload.id, sys_user::check_id, "用户ID", &mut res);
    check_option(
        &payload.password,
        sys_user::check_password,
        "密码",
        &mut res,
    );
    check_option(&payload.name, sys_user::check_name, "用户名", &mut res);
    check_option(&payload.role, sys_user::check_role, "角色", &mut res);
    check_option(&payload.locale, sys_user::check_lang, "语言", &mut res);
    check_option(&payload.timezone, sys_user::check_tz, "时区", &mut res);

    // TODO: 数据库干活

    if !res.is_empty() {
        return Err((StatusCode::BAD_REQUEST, res));
    }
    Ok((StatusCode::OK, ""))
}

#[tracing::instrument]
#[utoipa::path(
    delete,
    path = "/api/v1/users",
    description = "删除用户",
    request_body = model::RegisterRequest,
    responses(
        (status = 200, description = "成功"),
        (status = 400, description = "请求错误", body = String),
        (status = 401, description = "未授权"),
        (status = 404, description = "用户不存在"),
        (status = 500, description = "服务器错误"),
    ),
)]
pub async fn delete(
    Extension(AppState(db)): Extension<AppState>,
    Json(payload): Json<model::DeleteRequest>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let mut res = String::new();
    check_field(&payload.id, sys_user::check_id, "用户ID", &mut res);
    if !res.is_empty() {
        return Err((StatusCode::BAD_REQUEST, res));
    }

    match delete_user(&payload.id, db).await {
        Ok(_) => Ok((StatusCode::OK, "")),
        Err(DeleteError::NotFound) => {
            error!("用户不存在");
            return Err((
                StatusCode::NOT_FOUND,
                "用户不存在".to_string(),
            ));
        }
        Err(e) => {
            error!("删除用户时出错: {}", e);
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "服务器错误".to_string(),
            ));
        }
    }
}
