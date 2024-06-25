//! 包含用户的注册，登陆，注销，更新邮箱，更新密码
//! 之后想到的操作应该先写在这里

use crate::ResJson;
use sea_orm::DbErr;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

pub mod jwt;
pub mod login;
pub mod register;

#[derive(Debug)]
pub enum Error {
    /// 通用错误
    ServerError(crate::Error),
    /// 数据库操作失败
    DatabaseOptFailed(DbErr),
    /// JWT鉴权失败
    JWTError(jwt::JWTErrorCase),
    /// 操作权限不足
    PermissionDenied,
    /// 登陆错误
    LoginError(login::LoginError),
    /// 非法地区字串
    InvalidLocale,
}

impl From<Error> for ResJson<()> {
    fn from(e: Error) -> Self {
        // TODO: 之后把错误输出搬到这里
        ResJson {
            error_flag: true,
            error_code: match e {
                Error::ServerError(_) => 0,
                Error::DatabaseOptFailed(_) => 1,
                Error::JWTError(_) => 2,
                Error::PermissionDenied => 3,
                Error::LoginError(_) => 4,
                Error::InvalidLocale => 5,
            } * 100
                + 1,
            data: None,
        }
    }
}

pub type Result<T> = std::result::Result<T, Error>;

/// 角色，用户类型
#[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq)]
pub enum Role {
    /// 匿名用户
    AnonymousUser = -1,
    /// 管理员
    Administrator = 0,
    /// 用户
    User = 1,
}

impl From<i32> for Role {
    fn from(value: i32) -> Self {
        match value {
            -1 => Role::AnonymousUser,
            0 => Role::Administrator,
            _ => Role::User,
        }
    }
}
