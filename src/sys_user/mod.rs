use serde::{Deserialize, Serialize};

/// 登陆
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoginRequest {
    /// 用户名
    pub username: String,
    /// 密码
    pub password: String,
}

/// 登陆响应体
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoginResponse {
    /// jwt令牌
    pub token: String,
    /// 密钥
    pub secret: String,
}

/// 注册
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RegisterRequest {
    /// 用户名
    pub username: String,
    /// 密码
    pub password: String,
    /// 角色
    pub role: i32,
    /// 偏好时区
    pub timezone: String,
    /// 偏好语言
    pub locale: String,
}

/// 注册响应体
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RegisterResponse {}

/// 更新请求体
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateRequest {
    /// 目标用户ID
    pub id: i32,
    /// 用户名（可选）
    pub username: Option<String>,
    /// 密码（可选）
    pub password: Option<String>,
    /// 角色（可选）
    pub role: Option<i32>,
    /// 偏好时区（可选）
    pub timezone: Option<String>,
    /// 偏好语言（可选）
    pub locale: Option<String>,
}

/// 更新响应体
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateResponse {}

/// 删除请求体
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteRequest {
    /// 目标用户ID
    pub id: i32,
}

/// 删除响应体
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteResponse {}

/// 查询请求体
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryUserRequest {
    /// 起始索引（包含）
    pub start: usize,
    /// 结束索引（不包含）
    pub end: usize,
}

/// 查询响应体
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryUserResponse {
    /// 用户列表
    pub users: Vec<UserBrief>,
}

/// 用户简要信息
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserBrief {
    /// 用户ID
    pub id: i32,
    /// 用户名
    pub username: String,
    /// 角色（可选）
    pub role: Option<i32>,
    /// 偏好时区（可选）
    pub timezone: Option<String>,
    /// 偏好语言（可选）
    pub locale: Option<String>,
}

/// 条件查询用户请求体
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryUserByConditionRequest {
    /// 用户名（可选）
    pub username: Option<String>,
    /// 角色（可选）
    pub role: Option<i32>,
    /// 偏好时区（可选）
    pub timezone: Option<String>,
    /// 偏好语言（可选）
    pub locale: Option<String>,
    /// 起始索引（包含）
    pub start: usize,
    /// 结束索引（不包含）
    pub end: usize,
}

/// 条件查询用户响应体
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryUserByConditionResponse {
    /// 用户列表
    pub users: Vec<UserBrief>,
}