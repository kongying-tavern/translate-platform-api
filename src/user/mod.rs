//! 包含用户的注册，登陆，注销，更新邮箱，更新密码
//! 之后想到的操作应该先写在这里

use crate::ResJson;
use chrono::Utc;
use isolang::Language;
use jsonwebtoken::errors::Error as JWTPkgError;
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
    DatabaseOptFailed,
    /// 生成JWT失败
    FailedToProduceJWT(JWTPkgError),
    /// JWT格式错误
    JWTFormatError(jwt::JWTErrorCase),
    /// JWT鉴权失败
    JWTVerificationFailed(JWTPkgError),
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
                Error::ServerError(e) => {
                    eprintln!("服务器错误：{:?}", e);
                    0
                }
                Error::DatabaseOptFailed => {
                    eprintln!("数据库操作错误：{:?}", e);
                    1
                }
                Error::FailedToProduceJWT(e) => {
                    eprintln!("生成JWT失败{:?}", e);
                    2
                }
                Error::JWTFormatError(e) => {
                    eprintln!("JWT格式错误{:?}", e);
                    3
                }
                Error::JWTVerificationFailed(e) => {
                    eprintln!("JWT验证失败{:?}", e);
                    4
                }
                Error::PermissionDenied => 5,
                Error::LoginError(e) => {
                    eprintln!("登陆失败：{:?}", e);
                    6
                }
                Error::InvalidLocale => {
                    eprintln!("非法地区字串");
                    7
                }
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
