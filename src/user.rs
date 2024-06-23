//! 包含用户的注册，登陆，注销，更新邮箱，更新密码
//! 之后想到的操作应该先写在这里

use crate::ResJson;
use actix_web::{self, web, HttpResponse, Responder};
use isolang::Language;
use jsonwebtoken::{errors::Error as JWTPkgError, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use tokio_postgres::error::Error as PostgresPkgError;

#[derive(Debug)]
pub enum Error {
    /// 数据库插入失败
    DatabaseInsertionFailed(PostgresPkgError),
    /// 生成JWT失败
    FailedToProduceJWT(JWTPkgError),
    /// JWT格式错误
    JWTFormatError(jwt::JWTErrorCase),
    /// JWT鉴权失败
    JWTVerificationFailed(JWTPkgError),
}

impl From<Error> for ResJson<()> {
    fn from(e: Error) -> Self {
        // TODO: 之后把错误输出搬到这里
        ResJson {
            error_flag: true,
            error_code: match e {
                Error::DatabaseInsertionFailed(_) => 1,
                Error::FailedToProduceJWT(_) => 2,
                Error::JWTFormatError(_) => 3,
                Error::JWTVerificationFailed(_) => 4,
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

/// 新用户注册的响应。
#[actix_web::post("/register")]
pub async fn register(
    db_pool: web::Data<deadpool_postgres::Pool>,
    req_body: web::Json<UserData>,
) -> impl Responder {
    // TODO: 用户注册之前应该有一个检查条件

    let client = db_pool.get().await.unwrap();
    if let Err(e) = client
        .execute(
            "select * from user_table",
            &[], // &[&req_body.name, &req_body.password, &req_body.email],
        )
        .await
    {
        // TODO 之后有了日志再修改
        let error = Error::DatabaseInsertionFailed(e);
        eprintln!("注册失败: {:?}", error);
        return HttpResponse::Forbidden().json(ResJson::from(error));
    }
    println!("{:?}", req_body);
    let res = ResJson::new(req_body.get_jwt().await.unwrap());
    HttpResponse::Ok().json(res)
}

pub mod jwt {

    use actix_web::{self, body, dev, http::header, HttpMessage, HttpResponse};
    use actix_web_lab::middleware::Next;
    use chrono::Utc;
    use jsonwebtoken::{Algorithm, DecodingKey, Validation};
    use serde::{Deserialize, Serialize};

    pub(super) const SECRET: &[u8] = "固定的secret".as_bytes();

    /// JWT操作中更加具体的错误细节
    /// 这里的错误我认为不应该暴露给前端，只应该留存在后端的日志里
    #[derive(Debug)]
    pub(super) enum JWTErrorCase {
        /// 令牌无法转换为字符串
        MayNotString,
        /// 令牌字段可能为空
        MayEmpty,
        /// 字串格式非法
        InvalidFormat,
    }

    #[derive(Serialize, Deserialize)]
    pub struct Claims {
        /// 所有者
        sub: usize,
        /// 超时时间
        exp: usize,
        /// 用户角色
        role: super::Role,
    }

    impl Claims {
        /// 得到刷新用的令牌
        pub fn get_refresh_token(&mut self) -> &Self {
            self.exp += 60 * 60 * 24 * 7; // 1 周
            self
        }
        pub fn new(sub: usize) -> Self {
            let now = Utc::now().timestamp() as usize;
            let exp = now + 60 * 60; // 一小时有效期
            Self {
                sub,
                exp,
                role: super::Role::User,
            }
        }
    }

    /// JWT鉴权中间件，验证成功会在请求头中将JWT字段替换为用户角色
    /// 鉴权的具体步骤位于`verify_jwt`函数中
    pub async fn mw_verify_jwt(
        req: dev::ServiceRequest,
        next: Next<impl body::MessageBody + 'static>,
    ) -> Result<dev::ServiceResponse<impl body::MessageBody + 'static>, actix_web::Error> {
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
    async fn verify_jwt(jwt: Option<&header::HeaderValue>) -> super::Result<super::Role> {
        let claims = jsonwebtoken::decode::<Claims>(
            jwt.ok_or(super::Error::JWTFormatError(JWTErrorCase::MayEmpty))?
                .to_str()
                .map_err(|_| super::Error::JWTFormatError(JWTErrorCase::MayNotString))?
                .strip_prefix("Bearer ")
                .ok_or(super::Error::JWTFormatError(JWTErrorCase::InvalidFormat))?,
            &DecodingKey::from_secret(SECRET),
            &Validation::new(Algorithm::HS256),
        )
        .map_err(|e| super::Error::JWTVerificationFailed(e))?;
        Ok(claims.claims.role)
    }
}
