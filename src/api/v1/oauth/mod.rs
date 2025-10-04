use axum::{Extension, Json, http::StatusCode, response::IntoResponse};
use tracing::error;

use crate::{
    AppState,
    api::v1::{oauth::db::verify_user, users::utils::check_field},
    sys_user,
};

pub mod db;
pub mod model;

// root:/api/v1/oauth
pub async fn router() -> anyhow::Result<axum::Router> {
    let ret = axum::Router::new()
        // .route("/", axum::routing::get(list))
        ;
    Ok(ret)
}

#[tracing::instrument]
#[utoipa::path(
    post,
    path = "/api/v1/users",
    description = "创建用户",
    request_body = model::LoginRequest,
    responses(
        (status = 201, description = "成功"),
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

    match verify_user(payload, db).await {
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
