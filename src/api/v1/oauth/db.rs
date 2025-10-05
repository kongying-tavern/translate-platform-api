use anyhow::Error;
use jsonwebtoken::{EncodingKey, Header, encode};
use once_cell::sync::Lazy;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use tracing::debug;

use crate::{
    api::v1::oauth::model::{
        LoginAndRefreshResponse, LoginRequest, ACCESS_TOKEN_LIFETIME, REFRESH_TOKENS, REFRESH_TOKEN_LIFETIME
    }, entities::{prelude::SysUser, sys_user::Column}, sys_user::{decode_id, encode_id, AuthClaims}, DB
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

/// 登陆时校验用户并生成令牌
pub async fn verify_user(
    payload: LoginRequest,
    db: &DB,
) -> Result<LoginAndRefreshResponse, DbError> {
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
    let mut claims = AuthClaims {
        id: encode_id(user.id).map_err(|e| DbError::DbError(e.into()))?,
        role: user.role as u8,
        exp: (chrono::Utc::now() + ACCESS_TOKEN_LIFETIME).timestamp() as usize,
    };

    let access_token = encode(&Header::default(), &claims, &ENCODING_KEY)
        .map_err(|e| DbError::DbError(e.into()))?;

    claims.exp = (chrono::Utc::now() + REFRESH_TOKEN_LIFETIME).timestamp() as usize;

    // 生成刷新 token
    let refresh_token = encode(&Header::default(), &claims, &ENCODING_KEY)
        .map_err(|e| DbError::DbError(e.into()))?;

    REFRESH_TOKENS
        .lock()
        .await
        .insert(refresh_token.clone(), claims);

    Ok(LoginAndRefreshResponse {
        access_token,
        refresh_token,
    })
}

/// 刷新令牌
pub async fn refresh_token(old_refresh_token: String) -> Result<LoginAndRefreshResponse, DbError> {
    let mut tokens = REFRESH_TOKENS.lock().await;
    if let Some(mut claims) = tokens.remove(&old_refresh_token) {
        debug!("用户 {:?} 刷新令牌", decode_id(&claims.id));

        // 生成新的访问令牌
        claims.exp = (chrono::Utc::now() + ACCESS_TOKEN_LIFETIME).timestamp() as usize;
        let access_token = encode(&Header::default(), &claims, &ENCODING_KEY)
            .map_err(|e| DbError::DbError(e.into()))?;

        // 生成新的刷新令牌
        claims.exp = (chrono::Utc::now() + REFRESH_TOKEN_LIFETIME).timestamp() as usize;
        let refresh_token = encode(&Header::default(), &claims, &ENCODING_KEY)
            .map_err(|e| DbError::DbError(e.into()))?;
        tokens.insert(refresh_token.clone(), claims);

        Ok(LoginAndRefreshResponse {
            access_token,
            refresh_token,
        })
    } else {
        Err(DbError::NotFound)
    }
}

/// 删除令牌
pub async fn delete_token(old_refresh_token: String) -> Result<(), DbError> {
    let mut tokens = REFRESH_TOKENS.lock().await;
    if let Some(claims) = tokens.remove(&old_refresh_token) {
        debug!("用户 {:?} 登出，删除令牌", decode_id(&claims.id));
        Ok(())
    } else {
        Err(DbError::NotFound)
    }
}
