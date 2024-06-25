use super::{Error, Result};
use crate::entity::sys_user;
use actix_web::{web, HttpMessage, HttpResponse};
use chrono::Utc;
use sea_orm::{ActiveModelTrait, ActiveValue, DbConn};

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Register {
    pub username: String,
    pub password: String,
    pub role: i32,
    pub timezone: String,
    pub locale: String,
}

impl Register {
    fn into_sys_user(&self, creator_id: i32) -> Result<sys_user::ActiveModel> {
        // 检查地区是否合法
        isolang::Language::from_name(&self.locale).ok_or(Error::InvalidLocale)?;

        Ok(sys_user::ActiveModel {
            version: ActiveValue::Set(1),
            creator_id: ActiveValue::Set(creator_id as i64),
            create_time: ActiveValue::Set(Some(Utc::now().naive_utc())),
            updater_id: ActiveValue::Set(creator_id as i64),
            update_time: ActiveValue::Set(Some(Utc::now().naive_utc())),
            del_flag: ActiveValue::Set(false),
            id: ActiveValue::NotSet,
            username: ActiveValue::Set(self.username.clone()),
            password: ActiveValue::Set(bcrypt::hash(&self.password, bcrypt::DEFAULT_COST).unwrap()),
            role: ActiveValue::Set(self.role),
            timezone: ActiveValue::Set(self.timezone.clone()),
            locale: ActiveValue::Set(self.locale.clone()),
        })
    }
}

/// 新用户注册的请求处理函数，具体的操作在`register`函数中
#[actix_web::post("/register")]
pub async fn sv_register(
    db_pool: web::Data<[DbConn; 1]>,
    req: actix_web::HttpRequest,
    req_body: web::Json<Register>,
) -> impl actix_web::Responder {
    match register(db_pool, req, req_body).await {
        Ok(_) => HttpResponse::Ok().json(crate::ResJson::new("注册成功")),
        Err(e) => HttpResponse::Forbidden().json(crate::ResJson::from(e)),
    }
}

async fn register(
    db: web::Data<[DbConn; 1]>,
    req: actix_web::HttpRequest,
    req_body: web::Json<Register>,
) -> Result<()> {
    // 验证管理员权限
    let id = match req
        .extensions()
        .get::<(i32, super::Role)>()
        .ok_or(Error::ServerError(crate::Error::ServerLogicError))?
    {
        (id, super::Role::Administrator) => *id,
        _ => return Err(Error::PermissionDenied),
    };

    req_body
        .into_sys_user(id)?
        .insert(&db[0])
        .await
        .map_err(|e| Error::DatabaseOptFailed(e))?;

    Ok(())
}

#[test]
fn test_bcrypt() {
    let hash = bcrypt::hash("password", bcrypt::DEFAULT_COST).unwrap();
    println!("明文: password, 密文: {}", hash);
    assert!(bcrypt::verify("password", &hash).unwrap());
}
