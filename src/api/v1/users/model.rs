use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// 注册
#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct RegisterRequest {
    /// 用户名
    #[schema(example = "newuser")]
    pub name: String,
    /// 密码
    #[schema(example = "securepassword")]
    pub password: String,
    /// 角色
    #[schema(example = 1)]
    pub role: i32,
    /// 偏好时区
    #[schema(example = "Asia/Shanghai")]
    pub timezone: String,
    /// 偏好语言
    #[schema(example = "zh-CN")]
    pub locale: String,
}

/// 更新请求体
#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UpdateRequest {
    /// 目标用户ID
    #[schema(example = 1)]
    pub id: i32,
    /// 用户名（可选）
    #[schema(example = "updateduser")]
    pub username: Option<String>,
    /// 密码（可选）
    #[schema(example = "newpassword")]
    pub password: Option<String>,
    /// 角色（可选）
    #[schema(example = 2)]
    pub role: Option<i32>,
    /// 偏好时区（可选）
    #[schema(example = "America/New_York")]
    pub timezone: Option<String>,
    /// 偏好语言（可选）
    #[schema(example = "en-US")]
    pub locale: Option<String>,
}

/// 删除请求体
#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct DeleteRequest {
    /// 目标用户ID
    #[schema(example = 1)]
    pub id: i32,
}

/// 查询响应体
#[derive(Debug, Clone, PartialEq, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct QueryResponse {
    /// 用户列表
    pub users: Vec<UserBrief>,
}

/// 用户简要信息
#[derive(Debug, Clone, PartialEq, Serialize, ToSchema)]
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

/// 查询请求体
#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct QueryRequest {
    /// 分页页码
    #[schema(example = 1)]
    pub page: u64,
    /// 每页大小
    #[schema(example = 10)]
    pub per_page: u64,
    /// 用户名查询条件（可选）
    #[schema(example = "admin")]
    pub username: Option<String>,
    /// 角色查询条件（可选）
    #[schema(example = 1)]
    pub role: Option<i32>,
    /// 偏好时区（可选）
    pub timezone: Option<String>,
    /// 偏好语言（可选）
    pub locale: Option<String>,
}
