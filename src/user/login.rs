use crate::user::Role;

use super::{jwt, Error, Result};
use actix_web::{web, HttpResponse};

#[derive(serde::Deserialize, serde::Serialize, Debug)]
struct Login {
    username: String,
    password: String,
}

#[derive(serde::Serialize)]
struct LoginRes {
    role: i32,
    timezone: String,
    locale: String,
    token: String,
    refresh_token: String,
}

/// 登陆行为的具体错误，在前端不会看到这些细节，只会在日志中看到
#[derive(Debug)]
pub enum LoginError {
    UserNotFound,
    PasswordIncorrect,
}

#[actix_web::post("/login")]
pub async fn sv_login(
    db_pool: web::Data<deadpool_postgres::Pool>,
    req_body: web::Json<Login>,
) -> impl actix_web::Responder {
    println!("{:?}", req_body);
    match login(db_pool, req_body).await {
        Ok(token) => HttpResponse::Ok().json(crate::ResJson::new(token)),
        Err(e) => HttpResponse::Forbidden().json(crate::ResJson::from(e)),
    }
}

async fn login(
    db_pool: web::Data<deadpool_postgres::Pool>,
    req_body: web::Json<Login>,
) -> Result<LoginRes> {
    let client = db_pool
        .get()
        .await
        .map_err(|_| Error::ServerError(crate::Error::DatabaseConnectionFailed))?;

    let statement = [
        client
            .prepare("select password, id from sys_user where username = $1 and del_flag = false")
            .await
            .map_err(|e| Error::DatabaseOptFailed(e))?,
        client
            .prepare("select role, timezone, locale from sys_user where id = $1")
            .await
            .map_err(|e| Error::DatabaseOptFailed(e))?,
    ];

    let rows = client
        .query(&statement[0], &[&req_body.username])
        .await
        .map_err(|e| {
            println!("{e:?}");
            Error::LoginError(LoginError::UserNotFound)
        })?;

    println!("{:?}", rows);

    match rows.iter().find_map(|row| {
        let hashed = row.get::<_, String>(0);
        match bcrypt::verify(req_body.password.clone(), &hashed).unwrap() {
            true => Some(row),
            false => None,
        }
    }) {
        Some(row) => {
            println!("{:?}", row);
            let id = row.get::<_, i32>(1) as i32;
            println!("{:?}", id);
            let row = client.query_one(&statement[1], &[&id]).await.map_err(|e| {
                println!("{e:?}");
                Error::LoginError(LoginError::UserNotFound)
            })?;
            let role = row.get::<_, i32>(0);
            let token = jwt::get_jwt(id, Role::from(role))?;
            Ok(LoginRes {
                role,
                timezone: row.get::<_, String>(1),
                locale: row.get::<_, String>(2),
                token: token.0,
                refresh_token: token.1,
            })
        }
        None => Err(Error::LoginError(LoginError::PasswordIncorrect)),
    }
}
