//! 包含用户的注册，登陆，注销，更新邮箱，更新密码
//! 之后想到的操作应该先写在这里

use crate::ResJson;
use chrono::Utc;
use isolang::Language;
use jsonwebtoken::errors::Error as JWTPkgError;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use tokio_postgres::error::Error as PostgresPkgError;

pub mod jwt;
pub mod login;
pub mod register;

#[derive(Debug)]
pub enum Error {
    /// 通用错误
    ServerError(crate::Error),
    /// 数据库操作失败
    DatabaseOptFailed(PostgresPkgError),
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
                Error::DatabaseOptFailed(e) => {
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
#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
pub enum Role {
    /// 匿名用户
    AnonymousUser = -1,
    /// 管理员
    Administrator = 0,
    /// 用户
    User = 1,
}

impl From<i8> for Role {
    fn from(value: i8) -> Self {
        match value {
            -1 => Role::AnonymousUser,
            0 => Role::Administrator,
            _ => Role::User,
        }
    }
}

/// 用户的基本注册信息，用于生成jwt令牌
#[derive(Debug, Deserialize, Serialize, Clone)]
struct UserData {
    /// 用户名
    username: String,
    /// 密码
    password: String,
    /// 偏好时区
    timezone: String,
    /// 角色
    role: Role,
    /// 偏好语言
    locale: Language,
    /// 通用字段
    inner: crate::UniversalField,
}

impl UserData {
    /// 仅用于注册后进行插入数据库
    fn from_register(data: register::Register, builder: u64) -> Result<Self> {
        let inner = crate::UniversalField {
            id: 0,
            version: 1,
            create_by: Some(builder),
            create_time: Some(Utc::now()),
            update_by: Some(builder),
            update_time: Some(Utc::now()),
            del_flag: false,
        };

        let locale = match Language::from_autonym(data.locale.as_str()) {
            Some(locale) => locale,
            None => return Err(Error::InvalidLocale),
        };

        let password = bcrypt::hash(data.password.as_str(), bcrypt::DEFAULT_COST)
            .map_err(|_| Error::ServerError(crate::Error::ServerLogicError))?;

        Ok(Self {
            username: data.username,
            password,
            timezone: data.timezone,
            role: data.role.into(),
            locale,
            inner,
        })
    }
}

impl IntoIterator for UserData {
    type Item = Option<String>;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        let user = vec![
            Some(self.username),
            Some(self.password),
            Some(self.timezone),
            Some((self.role as i8).to_string()),
            Some(self.locale.to_string()),
        ];
        // REVIEW: 这看起来有一些损耗，对...对吗？
        self.inner
            .into_iter()
            .chain(user.into_iter())
            .collect::<Vec<Option<String>>>()
            .into_iter()
    }
}
