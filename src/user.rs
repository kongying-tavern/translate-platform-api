//! 包含用户的注册，登陆，注销，更新邮箱，更新密码
//! 之后想到的操作应该先写在这里
//!
//! 用户表为：

use std::collections::HashMap;

use actix_web::{self, web, HttpResponse, Responder};
use chrono::Utc;
use jsonwebtoken::{EncodingKey, Header};
use serde::{Deserialize, Serialize};

/// 用户的基本注册信息
#[derive(Debug, Deserialize, Serialize)]
struct UserData {
    name: String,
    email: String,
    password: String,
}

#[actix_web::post("/register")]
pub async fn register(
    db_pool: web::Data<deadpool_postgres::Pool>,
    req_body: web::Json<UserData>,
) -> impl Responder {
    // 注册
    let client = db_pool.get().await.unwrap();
    if let Err(e)=client
    .execute(
        "INSERT INTO user_table (username, password, email, create_time) VALUES ($1, $2, $3, CURRENT_TIMESTAMP)",
        &[&req_body.name, &req_body.password, &req_body.email],
    )
    .await {
            // TODO 之后有了日志再修改
            eprintln!("注册失败: {}", e);
            return HttpResponse::Forbidden().finish();
        }

    let now = Utc::now().timestamp() as usize;
    // REVIEW: 感觉这个值需要暴露出来
    let exp = now + 60 * 60; // 一小时有效期

    let mut claims = HashMap::new();
    claims.insert("sub", req_body.name.clone());
    claims.insert("exp", exp.to_string());

    // REVIEW: ；这里的secret也是一个问题，这里我先拿用户的三个属性异或出一个值
    let secret = xor_user_data(&req_body.0);
    let token = jsonwebtoken::encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.to_be_bytes().as_ref()),
    )
    .unwrap();

    let refresh_exp = now + 60 * 60 * 24 * 7; // 1 周有效期
    claims.get_mut("exp").replace(&mut refresh_exp.to_string());
    let refresh_token = jsonwebtoken::encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.to_be_bytes().as_ref()),
    )
    .unwrap();

    let mut res = HashMap::new();
    res.insert("token", token);
    res.insert("refresh_token", refresh_token);

    HttpResponse::Ok().json(res)
}

/// 将用户的三个属性异或出一个值，用来当作JWT的secret
fn xor_user_data(data: &UserData) -> u64 {
    data.name.as_bytes().iter().map(|&a| a as u64).sum::<u64>()
        ^ data.email.as_bytes().iter().map(|&a| a as u64).sum::<u64>()
        ^ data
            .password
            .as_bytes()
            .iter()
            .map(|&a| a as u64)
            .sum::<u64>()
}
