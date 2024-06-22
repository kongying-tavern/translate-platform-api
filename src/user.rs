//! 包含用户的注册，登陆，注销，更新邮箱，更新密码
//! 之后想到的操作应该先写在这里

use std::collections::HashMap;

use actix_web::{self, web, HttpResponse, Responder};
use chrono::Utc;
use jsonwebtoken::{errors::Error as JWTPkgError, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use tokio_postgres::error::Error as PostgresPkgError;

#[derive(Debug)]
pub enum Error {
    IncorrectUserInformation(PostgresPkgError), //错误的用户信息
    FailedToProduceJWT(JWTPkgError),            //生成JWT失败
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

#[actix_web::post("/register")]
pub async fn register(
    db_pool: web::Data<deadpool_postgres::Pool>,
    req_body: web::Json<UserData>,
) -> impl Responder {
    // TODO: 用户注册之前应该有一个检查条件

    let client = db_pool.get().await.unwrap();
    if let Err(e) = client
        .execute(
            "**SQL语句待填充**",
            &[&req_body.name, &req_body.password, &req_body.email],
        )
        .await
    {
        // TODO 之后有了日志再修改
        eprintln!("注册失败: {:?}", Error::IncorrectUserInformation(e));
        return HttpResponse::Forbidden().finish();
    }

    HttpResponse::Ok().json(req_body.get_jwt().await.unwrap())
}
