use super::{Error, Result};
use actix_web::{web, HttpMessage, HttpResponse};
use diesel::{
    r2d2::{ConnectionManager, Pool},
    PgConnection,
};

/// 新用户注册的请求处理函数，具体的操作在`register`函数中
#[actix_web::post("/register")]
pub async fn sv_register(
    db_pool: web::Data<Pool<ConnectionManager<PgConnection>>>,
    req: actix_web::HttpRequest,
    req_body: web::Json<super::UserData>,
) -> impl actix_web::Responder {
    match register(db_pool, req, req_body).await {
        Ok(_) => HttpResponse::Ok().json(crate::ResJson::new("注册成功")),
        Err(e) => HttpResponse::Forbidden().json(crate::ResJson::from(e)),
    }
}

async fn register(
    db_pool: web::Data<Pool<ConnectionManager<PgConnection>>>,
    req: actix_web::HttpRequest,
    req_body: web::Json<super::UserData>,
) -> Result<()> {
    // 验证管理员权限
    match req
        .extensions()
        .get::<super::Role>()
        .ok_or(Error::ServerError(crate::Error::ServerLogicError))?
    {
        super::Role::Administrator => (),
        _ => return Err(Error::PermissionDenied),
    };
    let client = db_pool
        .get()
        .map_err(|_| Error::ServerError(crate::Error::DatabaseConnectionFailed))?;

    // client
    //     .execute(
    //         "select * from user_table",
    //         &[], // &[&req_body.name, &req_body.password, &req_body.email],
    //     )
    //     .await
    //     .map_err(|e| Error::DatabaseInsertionFailed(e))?;

    Ok(())
}
