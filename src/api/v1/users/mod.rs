use axum::response::IntoResponse;
use reqwest::StatusCode;

// root:/api/v1/user
pub async fn router() -> anyhow::Result<axum::Router> {
    let ret = axum::Router::new()
        .route("/", axum::routing::get(list))
        .route("/:id", axum::routing::get(list_par))
        .route("/", axum::routing::post(create))
        .route("/:id", axum::routing::put(update))
        .route("/:id", axum::routing::delete(delete));
    Ok(ret)
}

/// 得到用户列表
/// GET /api/v1/users/
/// 返回用户们的ID,用户名,角色,偏好时区,偏好语言
// #[tracing::instrument(skip(auth))]
#[tracing::instrument]
pub async fn list(
    // ExtractAuthInfo(auth): ExtractAuthInfo
) -> Result<impl IntoResponse, (StatusCode, String)> {
    Ok(())
}

/// 条件查询用户
/// GET /api/v1/users/:id
/// 返回用户的ID,用户名,角色,偏好时区,偏好语言
// #[tracing::instrument(skip(auth))]
#[tracing::instrument]
pub async fn list_par(
    // ExtractAuthInfo(auth): ExtractAuthInfo
) -> Result<impl IntoResponse, (StatusCode, String)> {
    Ok(())
}

/// 创建用户
/// POST /api/v1/users/
/// 请求体: 用户名,密码,角色,偏好时区,偏好语言
/// 返回: 成功或失败
// #[tracing::instrument(skip(auth))]
#[tracing::instrument]
pub async fn create(
    // ExtractAuthInfo(auth): ExtractAuthInfo
) -> Result<impl IntoResponse, (StatusCode, String)> {
    Ok(())
}

/// 更新用户
/// PUT /api/v1/users/:id
/// 请求体: 用户名(可选),密码(可选),角色(可选),偏好时区(可选),偏好语言(可选)
/// 返回: 成功或失败
// #[tracing::instrument(skip(auth))]
#[tracing::instrument]
pub async fn update(
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
    // ExtractAuthInfo(auth): ExtractAuthInfo
) -> Result<impl IntoResponse, (StatusCode, String)> {
    Ok(())
}
