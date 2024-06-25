use super::{jwt, Error, Result};
use crate::{
    entity::{sys_user, SysUser},
    user::Role,
};
use actix_web::{web, HttpResponse};
use sea_orm::{ColumnTrait, DbConn, EntityTrait, QueryFilter, QuerySelect};

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

#[derive(Debug)]
pub enum LoginError {
    UserNotFound,
    PasswordIncorrect,
}

impl From<LoginError> for Error {
    fn from(e: LoginError) -> Self {
        Error::LoginError(e)
    }
}

/// 登陆行为的具体错误，在前端不会看到这些细节，只会在日志中看到
#[actix_web::post("/login")]
pub async fn sv_login(
    db: web::Data<[DbConn; 1]>,
    req_body: web::Json<Login>,
) -> impl actix_web::Responder {
    println!("{:?}", req_body);
    match login(db, req_body).await {
        Ok(token) => HttpResponse::Ok().json(crate::ResJson::new(token)),
        Err(e) => HttpResponse::Forbidden().json(crate::ResJson::from(e)),
    }
}

async fn login(db: web::Data<[DbConn; 1]>, req_body: web::Json<Login>) -> Result<LoginRes> {
    let users = SysUser::find()
        .select_only()
        .column(sys_user::Column::Password)
        .column(sys_user::Column::Id)
        .filter(sys_user::Column::Username.eq(req_body.username.clone()))
        .all(&db[0])
        .await
        .map_err(|e| Error::DatabaseOptFailed(e))?;

    let id = users
        .iter()
        .find_map(
            |user| match bcrypt::verify(&req_body.password, &user.password) {
                Ok(true) => Some(user.id),
                _ => None,
            },
        )
        .ok_or(LoginError::UserNotFound)?;

    let user = SysUser::find_by_id(id)
        .one(&db[0])
        .await
        .map_err(|e| Error::DatabaseOptFailed(e))?
        .ok_or(LoginError::PasswordIncorrect)?;

    let token = jwt::get_jwt(id, Role::from(user.role))?;

    Ok(LoginRes {
        role: user.role,
        timezone: user.timezone,
        locale: user.locale,
        token: token.0,
        refresh_token: token.1,
    })
}
