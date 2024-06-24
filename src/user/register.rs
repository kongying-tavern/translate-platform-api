use super::{Error, Result};
use actix_web::{web, HttpMessage, HttpResponse};
use deadpool_postgres::GenericClient;

/// 新用户注册的请求处理函数，具体的操作在`register`函数中
#[actix_web::post("/register")]
pub async fn sv_register(
    db_pool: web::Data<deadpool_postgres::Pool>,
    req: actix_web::HttpRequest,
    req_body: web::Json<super::UserData>,
) -> impl actix_web::Responder {
    match register(db_pool, req, req_body).await {
        Ok(_) => HttpResponse::Ok().json(crate::ResJson::new("注册成功")),
        Err(e) => HttpResponse::Forbidden().json(crate::ResJson::from(e)),
    }
}

async fn register(
    db_pool: web::Data<deadpool_postgres::Pool>,
    req: actix_web::HttpRequest,
    user_data: web::Json<super::UserData>,
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
        .await
        .map_err(|_| Error::ServerError(crate::Error::DatabaseConnectionFailed))?;

    let statement = client
        .prepare(
            "inset into sys_user (
                    id, username, password, 
                    role, timezone, locale, 
                    version, create_time, 
                    update_by, update_time, 
                    del_flag) 
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)",
        )
        .await
        .map_err(|e| Error::DatabaseInsertionFailed(e))?;

    client
        .execute_raw(&statement, user_data.0.into_iter())
        .await
        .map_err(|e| Error::DatabaseInsertionFailed(e))?;

    Ok(())
}
