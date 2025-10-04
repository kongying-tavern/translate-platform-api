use axum::{Extension, Json, http::StatusCode, response::IntoResponse};
use tracing::{debug, error};

use crate::{
    AppState,
    api::v1::users::db::{DbError, delete_user, query_user, update_user},
    sys_user::{self, AuthStatus},
};
use db::{check_name_exists, insert_user};
use utils::{check_field, check_option};

pub mod db;
pub mod middleware;
pub mod model;
pub mod utils;

// root:/api/v1/user
pub async fn router() -> anyhow::Result<axum::Router> {
    let ret = axum::Router::new()
        .route("/", axum::routing::get(query))
        .route("/", axum::routing::post(create))
        .route("/", axum::routing::put(update))
        .route("/", axum::routing::delete(delete))
        .layer(axum::middleware::from_fn(middleware::auth));
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
    Extension(AppState(db)): Extension<AppState>,
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

    match query_user(&payload, db).await {
        Ok((users, total_pages)) => Ok((
            StatusCode::OK,
            Json(model::QueryResponse { users, total_pages }),
        )),
        Err(DbError::NotFound) => {
            error!("分页参数错误, 页码超出范围");
            Err((StatusCode::NOT_FOUND, "分页参数错误".to_string()))
        }
        Err(e) => {
            error!("查询用户时出错: {}", e);
            Err((StatusCode::INTERNAL_SERVER_ERROR, "服务器错误".to_string()))
        }
    }
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
    Extension(auth): Extension<AuthStatus>,
    Json(payload): Json<model::RegisterRequest>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let mut res = String::new();

    if auth.role != sys_user::UserRole::Admin {
        return Err((StatusCode::UNAUTHORIZED, "".to_string()));
    }

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
        Ok(_) => Ok((StatusCode::CREATED, "")),
        Err(e) => {
            error!("插入用户时出错: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "数据库插入错误".to_string(),
            ))
        }
    }
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
    Extension(AppState(db)): Extension<AppState>,
    Extension(auth): Extension<AuthStatus>,
    Json(payload): Json<model::UpdateRequest>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let mut res = String::new();

    if auth.role != sys_user::UserRole::Admin
        && auth.id != sys_user::decode_id(&payload.id).unwrap_or(-1)
    {
        debug!(
            "非管理员用户只能更新自己的信息: {}, {}",
            auth.id, payload.id
        );
        return Err((StatusCode::UNAUTHORIZED, "".to_string()));
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

    if !res.is_empty() {
        return Err((StatusCode::BAD_REQUEST, res));
    }

    match update_user(payload, db).await {
        Ok(_) => Ok((StatusCode::OK, "")),
        Err(e) => {
            error!("更新用户时出错: {}", e);
            Err((StatusCode::INTERNAL_SERVER_ERROR, "服务器错误".to_string()))
        }
    }
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
    Extension(auth): Extension<AuthStatus>,
    Json(payload): Json<model::DeleteRequest>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let mut res = String::new();

    if auth.role != sys_user::UserRole::Admin
        && auth.id != sys_user::decode_id(&payload.id).unwrap_or(-1)
    {
        debug!(
            "非管理员用户只能更新自己的信息: {}, {}",
            auth.id, payload.id
        );
        return Err((StatusCode::UNAUTHORIZED, "".to_string()));
    }

    check_field(&payload.id, sys_user::check_id, "用户ID", &mut res);
    if !res.is_empty() {
        return Err((StatusCode::BAD_REQUEST, res));
    }

    match delete_user(&payload.id, db).await {
        Ok(_) => Ok((StatusCode::OK, "")),
        Err(DbError::NotFound) => {
            error!("用户不存在");
            return Err((StatusCode::NOT_FOUND, "用户不存在".to_string()));
        }
        Err(e) => {
            error!("删除用户时出错: {}", e);
            return Err((StatusCode::INTERNAL_SERVER_ERROR, "服务器错误".to_string()));
        }
    }
}
