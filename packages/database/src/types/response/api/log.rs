use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::net::IpAddr;
use strum::{Display, EnumMessage};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DatabaseLogItemOperation {
    Insert(i64),
    Update(i64),
    Delete(i64),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DatabaseLogItem {
    pub time: DateTime<Utc>,
    pub table: String,
    pub operation: DatabaseLogItemOperation,
    pub operator: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, EnumMessage)]
pub enum UserLogItemLoginFailedReason {
    #[strum(message = "用户名不存在")]
    NameNotFound,
    #[strum(message = "密码错误")]
    PasswordWrong,
    #[strum(message = "过多尝试")]
    TooManyAttempts,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Display)]
pub enum UserLogVisitMethod {
    #[strum(serialize = "GET")]
    Get,
    #[strum(serialize = "POST")]
    Post,
    #[strum(serialize = "PUT")]
    Put,
    #[strum(serialize = "DELETE")]
    Delete,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, EnumMessage)]
pub enum UserLogItemOperation {
    #[strum(message = "登录")]
    Login(i64),
    #[strum(message = "登录失败")]
    LoginFailed(UserLogItemLoginFailedReason),
    #[strum(message = "登出")]
    Logout(i64),
    #[strum(message = "验证")]
    Verify(i64),
    #[strum(message = "验证失败")]
    VerifyFailed,
    #[strum(message = "刷新令牌")]
    RefreshToken(i64),
    #[strum(message = "访问")]
    Visit((UserLogVisitMethod, String)),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UserLogItem {
    pub time: DateTime<Utc>,
    pub ip: IpAddr,
    pub user_agent: String,
    pub operation: UserLogItemOperation,
}
