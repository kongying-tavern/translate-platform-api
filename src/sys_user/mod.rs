use serde::{Deserialize, Serialize};
use strum::EnumString;

mod utils;

pub use utils::*;

// REVIEW: 前端请求中我想校验用户主键是否合法，方法是解码uuid（前端看到的是8长的字符串）后得
//         到一个自增主键。如果这个自增主键小于最大用户数就认为合法，否则不合法。
const MAX_USER: u64 = 4096;

#[derive(Debug, Clone, PartialEq, EnumString, Deserialize, Serialize)]
pub enum UserRole {
    Admin = 0,
    User = 1,
}

/// jwt 认证载荷
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AuthClaims {
    pub id: String,
    pub role: u8,
    pub exp: usize,
}

/// 认证状态
#[derive(Debug, Clone, PartialEq)]
pub struct AuthStatus {
    pub role: UserRole,
    pub id: i32,
}
