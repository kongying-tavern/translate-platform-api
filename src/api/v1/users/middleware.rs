use axum::{
    extract::Request,
    http::{self, StatusCode},
    middleware::Next,
    response::Response,
};
use jsonwebtoken::{DecodingKey, Validation, decode};
use once_cell::sync::Lazy;
use tracing::error;

use crate::sys_user::{AuthClaims, AuthStatus, UserRole, decode_id};

// TODO: 之后换个环境变量
// static PRIVATE_KEY: &[u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/private.pem"));
static PUBLIC_KEY: &[u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/public.pem"));
static DECODING_KEY: Lazy<DecodingKey> = Lazy::new(|| {
    DecodingKey::from_rsa_pem(PUBLIC_KEY).expect("Failed to create DecodingKey from public.pem")
});

pub async fn auth(mut req: Request, next: Next) -> Result<Response, StatusCode> {
    let auth_header = req
        .headers()
        .get(http::header::AUTHORIZATION)
        .and_then(|header| header.to_str().ok());

    let token = if let Some(auth_header) = auth_header {
        if auth_header.starts_with("Bearer ") {
            &auth_header["Bearer ".len()..]
        } else {
            return Err(StatusCode::BAD_REQUEST);
        }
    } else {
        return Err(StatusCode::UNAUTHORIZED);
    };

    let claims = decode::<AuthClaims>(token, &DECODING_KEY, &Validation::default())
        .map_err(|e| {
            error!("JWT 解码失败: {}", e);
            StatusCode::UNAUTHORIZED
        })?
        .claims;

    let auth = AuthStatus {
        id: decode_id(claims.id.to_string()).map_err(|e| {
            error!("ID 解码失败: {}", e);
            StatusCode::BAD_REQUEST
        })?,
        role: match claims.role {
            0 => UserRole::Admin,
            1 => UserRole::User,
            _ => return Err(StatusCode::UNAUTHORIZED),
        },
    };

    // 后续需要secret校验再说
    // let secret = SECRET_MAP.lock().await.get(&auth.id).ok_or(StatusCode::UNAUTHORIZED)?;

    req.extensions_mut().insert(auth);
    Ok(next.run(req).await)
}
