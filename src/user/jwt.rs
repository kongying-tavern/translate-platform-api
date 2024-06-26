use super::{Error, Result};
use actix_web::{self, body, dev, http::header, HttpMessage, HttpResponse};
use actix_web_lab::middleware::Next;
use chrono::Utc;
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

// TODO: 之后从env里面读
pub(super) const SECRET: &[u8] = "固定的secret".as_bytes();

/// JWT操作中更加具体的错误细节
/// 这里的错误我认为不应该暴露给前端，只应该留存在后端的日志里
#[derive(Debug)]
pub enum JWTErrorCase {
    /// 令牌无法转换为字符串
    MayNotString,
    /// 令牌字段可能为空
    MayEmpty,
    /// 字串格式非法
    InvalidFormat,
    /// 认证失败
    VerificationFailed(jsonwebtoken::errors::Error),
    /// 生成失败
    ProduceFailed(jsonwebtoken::errors::Error),
}

impl From<JWTErrorCase> for Error {
    fn from(value: JWTErrorCase) -> Self {
        Error::JWTError(value)
    }
}

#[derive(Serialize, Deserialize)]
pub struct Claims {
    /// 所有者
    sub: i32,
    /// 超时时间
    exp: usize,
    /// 用户角色
    role: super::Role,
}

impl Claims {
    /// 得到刷新用的令牌
    pub fn get_refresh_token(&mut self) -> &Self {
        self.exp += 60 * 60 * 60 * 24 * 7; // 1 周
        self
    }
    pub fn new(sub: i32, role: super::Role) -> Self {
        let now = Utc::now().timestamp() as usize;
        let exp = now + 60 * 60 * 60; // 一小时有效期
        Self { sub, exp, role }
    }
}

/// JWT鉴权中间件，验证成功会在请求头中将JWT字段替换为用户角色
/// 鉴权的具体步骤位于`verify_jwt`函数中
pub async fn mw_verify_jwt(
    req: dev::ServiceRequest,
    next: Next<impl body::MessageBody + 'static>,
) -> std::result::Result<dev::ServiceResponse<impl body::MessageBody + 'static>, actix_web::Error> {
    println!("catch mw");
    // 测试是否由中间件拦截请求。应该在具体函数中处理Option，整个鉴权过程会有至少三种错误，所以封装在另一个函数中
    let jwt = req.headers().get(header::AUTHORIZATION);
    println!("header::AUTHORIZATION: {:?}", jwt);
    let res = match verify_jwt(jwt).await {
        Ok(role) => {
            println!("role: {:?}", role);
            req.extensions_mut().insert(role);
            next.call(req).await?.map_into_right_body()
        }
        Err(e) => {
            println!("error: {:?}", e);
            req.into_response(HttpResponse::Forbidden().json(super::ResJson::from(e)))
                .map_into_left_body()
        }
    };
    Ok(res)
}

/// JWT的具体鉴权操作
async fn verify_jwt(jwt: Option<&header::HeaderValue>) -> super::Result<(i32, super::Role)> {
    let claims = jsonwebtoken::decode::<Claims>(
        jwt.ok_or(JWTErrorCase::MayEmpty)?
            .to_str()
            .map_err(|_| JWTErrorCase::MayNotString)?
            .strip_prefix("Bearer ")
            .ok_or(JWTErrorCase::InvalidFormat)?,
        &DecodingKey::from_secret(SECRET),
        &Validation::new(Algorithm::HS256),
    )
    .map_err(|e| JWTErrorCase::VerificationFailed(e))?;
    Ok((claims.claims.sub, claims.claims.role))
}

/// 生成jwt令牌，返回两个令牌，第一个是普通令牌，第二个是刷新令牌。
/// 默认情况下，普通令牌有效期为1小时，刷新令牌有效期为1周
/// REVIEW: 我们需要可变的有效期吗？
pub fn get_jwt(id: i32, role: super::Role) -> Result<(String, String)> {
    let mut claims = Claims::new(id, role);
    let token = jsonwebtoken::encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(SECRET),
    )
    .map_err(|e| JWTErrorCase::ProduceFailed(e))?;

    let refresh_token = jsonwebtoken::encode(
        &Header::default(),
        &claims.get_refresh_token(),
        &EncodingKey::from_secret(SECRET),
    )
    .map_err(|e| JWTErrorCase::ProduceFailed(e))?;
    Ok((token, refresh_token))
}

#[test]
fn test_encode_decode_jwt() {
    // Generate a JWT token
    let id = 123;
    let role = super::Role::Administrator;
    let (token, _) = get_jwt(id, role).unwrap();

    println!("token: {}", token);

    // Decode the JWT token
    let decoded = jsonwebtoken::decode::<Claims>(
        &token,
        &DecodingKey::from_secret(SECRET),
        &Validation::new(Algorithm::HS256),
    )
    .map_err(|e| JWTErrorCase::VerificationFailed(e))
    .unwrap();

    // Verify the decoded claims
    assert_eq!(decoded.claims.sub, id);
    assert_eq!(decoded.claims.role, role);
}
