use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// 登陆
#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct LoginRequest {
    /// 用户名
    #[schema(example = "admin")] // 可选：为文档添加示例值
    pub name: String,
    /// 密码
    #[schema(example = "Password123")] // 可选：为文档添加示例值
    pub password: String,
}

/// 登陆响应体
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct LoginResponse {
    /// jwt令牌
    #[schema(example = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...")]
    pub access_token: String,
}

/// 刷新令牌请求
#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct RefreshTokenRequest {
    /// 用于获取新令牌的旧刷新令牌
    #[schema(example = "refresh_token_string_from_previous_login")]
    pub refresh_token: String,
}