//! 包含用户的注册，登陆，注销，更新邮箱，更新密码
//! 之后想到的操作应该先写在这里

use crate::ResJson;
use actix_web::{self, web, HttpResponse, Responder};
use chrono::Utc;
use jsonwebtoken::{errors::Error as JWTPkgError, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fmt::Debug};
use tokio_postgres::error::Error as PostgresPkgError;

#[derive(Debug)]
pub enum Error {
    /// 数据库插入失败
    DatabaseInsertionFailed(PostgresPkgError),
    /// 生成JWT失败
    FailedToProduceJWT(JWTPkgError),
    /// JWT格式错误
    JWTFormatError(actix_web::Error),
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
            } * 100
                + 1,
            data: None,
        }
    }
}

pub type Result<T> = std::result::Result<T, Error>;

/// 用户的基本注册信息，用于生成jwt令牌
#[derive(Debug, Deserialize, Serialize)]
pub struct UserData {
    name: String,
    email: String,
    password: String,
}

impl UserData {
    /// 生成jwt令牌，返回两个令牌，第一个是普通令牌，第二个是刷新令牌。
    /// 默认情况下，普通令牌有效期为1小时，刷新令牌有效期为1周
    /// REVIEW: 我们需要可变的有效期吗？
    /// REVIEW: 这里的secret需要改变生成方法吗？
    async fn get_jwt(&self) -> Result<(String, String)> {
        let now = Utc::now().timestamp() as usize;
        let exp = now + 60 * 60; // 一小时有效期

        let mut claims = HashMap::new();
        claims.insert("sub", self.name.clone());
        claims.insert("exp", exp.to_string());

        let secret = self.xor_user_data();
        let token = jsonwebtoken::encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(&secret),
        )
        .map_err(|e| Error::FailedToProduceJWT(e))?;

        let refresh_exp = now + 60 * 60 * 24 * 7; // 1 周有效期
        claims.get_mut("exp").replace(&mut refresh_exp.to_string());
        let refresh_token = jsonwebtoken::encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(&secret),
        )
        .map_err(|e| Error::FailedToProduceJWT(e))?;
        Ok((token, refresh_token))
    }

    /// 将用户的三个属性异或出一个值，用来当作JWT的secret
    fn xor_user_data(&self) -> [u8; 8] {
        (self.name.as_bytes().iter().map(|&a| a as u64).sum::<u64>()
            ^ self.email.as_bytes().iter().map(|&a| a as u64).sum::<u64>()
            ^ self
                .password
                .as_bytes()
                .iter()
                .map(|&a| a as u64)
                .sum::<u64>())
        .to_be_bytes()
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
    use super::UserData;
    use actix_web::{self, body, dev, web, HttpResponse};
    use actix_web_lab::middleware::Next;
    use serde::Serialize;

    #[derive(Serialize)]
    struct Jwt {
        // Jwt；令牌
        token: String,
    }

    pub async fn verify_jwt(
        mut req: dev::ServiceRequest,
        next: Next<impl body::MessageBody + 'static>,
    ) -> Result<dev::ServiceResponse<impl body::MessageBody + 'static>, actix_web::Error> {
        // 测试是否由中间件拦截请求
        let json = req
            .extract::<web::Json<UserData>>()
            .await
            .map_err(|e| super::Error::JWTFormatError(e));
        let res = match json {
            Ok(json) => next.call(req).await?.map_into_left_body(),
            Err(e) => req
                .into_response(HttpResponse::Forbidden().json(super::ResJson::from(e)))
                .map_into_right_body(),
        };
        Ok(res)
        // post-processing
    }
}
