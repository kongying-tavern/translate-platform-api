use anyhow::Error;
use jsonwebtoken::{EncodingKey, Header, encode};
use once_cell::sync::Lazy;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

use crate::{
    DB,
    api::v1::oauth::model::{LoginRequest, LoginResponse},
    entities::{prelude::SysUser, sys_user::Column},
    sys_user::{AuthClaims, encode_id},
};

// TODO: 之后换个环境变量
static PRIVATE_KEY: &[u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/private.pem"));
static ENCODING_KEY: Lazy<EncodingKey> = Lazy::new(|| {
    EncodingKey::from_rsa_pem(PRIVATE_KEY).expect("Failed to create EncodingKey from private.pem")
});

#[derive(thiserror::Error, Debug)]
pub enum DbError {
    #[error("用户不存在")]
    NotFound,
    #[error("数据库错误: {0}")]
    DbError(Error),
}

pub async fn verify_user(payload: LoginRequest, db: &DB) -> Result<LoginResponse, DbError> {
    let user = SysUser::find()
        .filter(Column::Name.eq(&payload.name))
        .filter(Column::DelFlag.eq(false))
        .one(db)
        .await
        .map_err(|e| DbError::DbError(e.into()))?
        .ok_or(DbError::NotFound)?;

    if !crate::sys_user::verify_password(&payload.password, &user.password)
        .map_err(|e| DbError::DbError(e.into()))?
    {
        return Err(DbError::NotFound);
    }

    // 生成JWT token
    let claims = AuthClaims {
        id: encode_id(user.id).map_err(|e| DbError::DbError(e.into()))?,
        role: user.role as u8,
        exp: (chrono::Utc::now() + chrono::Duration::hours(24)).timestamp() as usize,
    };

    let access_token = encode(&Header::default(), &claims, &ENCODING_KEY)
        .map_err(|e| DbError::DbError(e.into()))?;

    Ok(LoginResponse { access_token })
}
