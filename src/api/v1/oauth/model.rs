use std::{collections::HashMap, sync::Arc};

use chrono::Duration;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;
use utoipa::ToSchema;

use crate::sys_user::AuthClaims;

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

/// 刷新
#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct RefreshRequest {
    /// 刷新令牌
    #[schema(example = "refresh_token_string")]
    pub refresh_token: String,
}

/// 登陆响应体
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct LoginAndRefreshResponse {
    /// jwt令牌
    #[schema(example = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...")]
    pub access_token: String,
    /// 刷新令牌
    #[schema(example = "refresh_token_string")]
    pub refresh_token: String,
}

/// 刷新令牌请求
#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct RefreshTokenRequest {
    /// 用于获取新令牌的旧刷新令牌
    #[schema(example = "refresh_token_string_from_previous_login")]
    pub refresh_token: String,
}

type TokenStore = Arc<Mutex<HashMap<String, AuthClaims>>>;

/// 全局、惰性、線程安全的刷新令牌存儲
pub static REFRESH_TOKENS: Lazy<TokenStore> = Lazy::new(|| {
    Arc::new(Mutex::new(HashMap::with_capacity(1024)))
});

// Refresh Token 和 Access Token 的有效期
pub const REFRESH_TOKEN_LIFETIME: Duration = Duration::days(7);
pub const ACCESS_TOKEN_LIFETIME: Duration = Duration::days(1);
