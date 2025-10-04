use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// 注册
#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct RegisterRequest {
    /// 用户名
    /// 长度8-32，只能包含字母、数字、下划线，且必须以字母开头
    #[schema(example = "newuser")]
    pub name: String,
    /// 密码
    /// 长度8-32，必须大写字母、小写字母、数字三者同时存在，可以有!@#$%^&*
    #[schema(example = "securepassword")]
    pub password: String,
    /// 角色
    /// 目前1（管理员）和2（普通用户）
    #[schema(example = "1")]
    pub role: String,
    /// 偏好时区
    /// IANA时区标识符，如Asia/Shanghai、America/New_York
    #[schema(example = "Asia/Shanghai")]
    pub timezone: String,
    /// 偏好语言
    /// BCP 47语言标签，如zh-CN、en-US、ja-JP
    #[schema(example = "zh-CN")]
    pub locale: String,
}

/// 更新请求体
#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UpdateRequest {
    /// 目标用户ID
    /// 在[这里](https://sqids.org/zh/playground#minLength=8)得到主键和ID的映射，你
    /// 可以用一个数组编码但我为了简单就只管第一个数了。比如4096应该编码为VZW1XmZF
    #[schema(example = "VZW1XmZF")]
    pub id: String,
    /// 用户名（可选）
    /// 长度8-32，只能包含字母、数字、下划线，且必须以字母开头
    #[schema(example = "updateduser")]
    pub name: Option<String>,
    /// 密码（可选）
    /// 长度8-32，必须大写字母、小写字母、数字三者同时存在，可以有!@#$%^&*
    #[schema(example = "newpassword")]
    pub password: Option<String>,
    /// 角色（可选）
    /// 目前1（管理员）和2（普通用户）
    #[schema(example = "2")]
    pub role: Option<String>,
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
    /// 在[这里](https://sqids.org/zh/playground#minLength=8)得到主键和ID的映射，你
    /// 可以用一个数组编码但我为了简单就只管第一个数了。比如4096应该编码为VZW1XmZF
    #[schema(example = "VZW1XmZF")]
    pub id: String,
}

/// 查询响应体
#[derive(Debug, Clone, PartialEq, Serialize, ToSchema, Default)]
#[serde(rename_all = "camelCase")]
pub struct QueryResponse {
    /// 用户列表
    /// 根据分页参数返回的用户简要信息列表
    #[schema(example = "[]")]
    pub users: Vec<UserBrief>,
    /// 总数
    #[schema(example = 0)]
    pub total_page: u64,
    // REVIEW: 还缺分页信息吗？
}

/// 用户简要信息
#[derive(Debug, Clone, PartialEq, Serialize, ToSchema, Default)]
#[serde(rename_all = "camelCase")]
pub struct UserBrief {
    /// 目标用户ID
    /// 在[这里](https://sqids.org/zh/playground#minLength=8)得到主键和ID的映射，你
    /// 可以用一个数组编码但我为了简单就只管第一个数了。比如4096应该编码为VZW1XmZF
    #[schema(example = "VZW1XmZF")]
    pub id: String,
    /// 用户名
    /// 长度8-32，只能包含字母、数字、下划线，且必须以字母开头
    #[schema(example = "updateduser")]
    pub name: String,
    /// 角色
    /// 目前1（管理员）和2（普通用户）
    #[schema(example = "1")]
    pub role: String,
    /// 偏好时区
    /// IANA时区标识符，如Asia/Shanghai、America/New_York
    #[schema(example = "Asia/Shanghai")]
    pub timezone: String,
    /// 偏好语言
    /// BCP 47语言标签，如zh-CN、en-US、ja-JP
    #[schema(example = "zh-CN")]
    pub locale: String,
}

/// 查询请求体
#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct QueryRequest {
    /// 分页页码
    /// 从1开始，最小值为1
    #[schema(example = 1)]
    pub page: u64,
    /// 每页大小
    /// 范围1-100，推荐10-50
    #[schema(example = 10)]
    pub per_page: u64,
    /// 用户名查询条件（可选）
    /// 支持模糊匹配，长度8-32
    #[schema(example = "admin")]
    pub name: Option<String>,
    /// 角色查询条件（可选）
    /// 目前1（管理员）和2（普通用户）
    #[schema(example = "1")]
    pub role: Option<String>,
    /// 偏好时区（可选）
    /// IANA时区标识符筛选，如Asia/Shanghai
    #[schema(example = "Asia/Shanghai")]
    pub timezone: Option<String>,
    /// 偏好语言（可选）
    /// BCP 47语言标签筛选，如zh-CN
    #[schema(example = "zh-CN")]
    pub locale: Option<String>,
}
