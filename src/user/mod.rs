//! 包含用户的注册，登陆，注销，更新邮箱，更新密码
//! 之后想到的操作应该先写在这里

use crate::ResJson;
use isolang::Language;
use jsonwebtoken::{errors::Error as JWTPkgError, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use tokio_postgres::error::Error as PostgresPkgError;

pub mod jwt;
pub mod register;

#[derive(Debug)]
pub enum Error {
    /// 通用错误
    ServerError(crate::Error),
    /// 数据库插入失败
    DatabaseInsertionFailed(PostgresPkgError),
    /// 生成JWT失败
    FailedToProduceJWT(JWTPkgError),
    /// JWT格式错误
    JWTFormatError(jwt::JWTErrorCase),
    /// JWT鉴权失败
    JWTVerificationFailed(JWTPkgError),
    /// 操作权限不足
    PermissionDenied,
}

impl From<Error> for ResJson<()> {
    fn from(e: Error) -> Self {
        // TODO: 之后把错误输出搬到这里
        ResJson {
            error_flag: true,
            error_code: match e {
                Error::ServerError(_) => 0,
                Error::DatabaseInsertionFailed(_) => 1,
                Error::FailedToProduceJWT(_) => 2,
                Error::JWTFormatError(_) => 3,
                Error::JWTVerificationFailed(_) => 4,
                Error::PermissionDenied => 5,
            } * 100
                + 1,
            data: None,
        }
    }
}

pub type Result<T> = std::result::Result<T, Error>;

/// 角色，用户类型
#[derive(Debug, Deserialize, Serialize)]
enum Role {
    /// 匿名用户
    AnonymousUser = -1,
    /// 管理员
    Administrator = 0,
    /// 用户
    User = 1,
}

/// 用户的基本注册信息，用于生成jwt令牌
#[derive(Debug, Deserialize, Serialize)]
struct UserData {
    /// 用户名
    username: String,
    /// 密码
    password: String,
    /// 偏好时区
    timezone: i8,
    /// 角色
    role: Role,
    /// 偏好语言
    locale: Language,
    /// 通用字段
    inner: crate::UniversalField,
}

impl UserData {
    /// 生成jwt令牌，返回两个令牌，第一个是普通令牌，第二个是刷新令牌。
    /// 默认情况下，普通令牌有效期为1小时，刷新令牌有效期为1周
    /// REVIEW: 我们需要可变的有效期吗？
    async fn get_jwt(&self) -> Result<(String, String)> {
        let mut claims = jwt::Claims::new(self.inner.id);

        // TODO: 之后从env里面读

        let token = jsonwebtoken::encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(jwt::SECRET),
        )
        .map_err(|e| Error::FailedToProduceJWT(e))?;

        let refresh_token = jsonwebtoken::encode(
            &Header::default(),
            &claims.get_refresh_token(),
            &EncodingKey::from_secret(jwt::SECRET),
        )
        .map_err(|e| Error::FailedToProduceJWT(e))?;
        Ok((token, refresh_token))
    }
}
