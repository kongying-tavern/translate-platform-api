use anyhow::Result;

use axum::{
    extract::{Json, Path},
    response::IntoResponse,
};
use hyper::StatusCode;

use crate::utils::ExtractAuthInfo;
use _database::{
    functions::models::user::{
        delete as do_delete, insert as do_insert, select as do_select, update as do_update,
    },
    types::request::models::User as VO,
};

#[tracing::instrument]
pub async fn insert(
    ExtractAuthInfo(auth): ExtractAuthInfo,
    Json(vo): Json<VO>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    if let Some(auth) = auth {
        // 不允许插入比自己权限等级更高或者同级的用户
        if auth.permission <= vo.permission {
            return Err((StatusCode::FORBIDDEN, "No permission".to_string()));
        }

        let ret = do_insert(vo, auth.id)
            .await
            .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?;
        return Ok(Json(ret));
    }

    Err((StatusCode::FORBIDDEN, "No permission".to_string()))
}

#[tracing::instrument]
pub async fn update(
    ExtractAuthInfo(auth): ExtractAuthInfo,
    Path(id): Path<i64>,
    Json(vo): Json<VO>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    if let Some(auth) = auth {
        let item = do_select(id)
            .await
            .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?
            .ok_or((
                StatusCode::NOT_FOUND,
                "The user does not exist or has been deleted".to_string(),
            ))?;

        // 不允许操作比自己权限等级更高或者同级的用户，自己除外
        if auth.id != id && auth.permission <= item.permission {
            return Err((StatusCode::FORBIDDEN, "No permission".to_string()));
        }

        // 如果是操作自己，不允许修改自己的权限等级
        if auth.id == id && vo.permission != item.permission {
            return Err((StatusCode::FORBIDDEN, "No permission".to_string()));
        }

        let ret = do_update(id, vo, auth.id)
            .await
            .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?;
        return Ok(Json(ret));
    }

    Err((StatusCode::FORBIDDEN, "No permission".to_string()))
}

pub async fn delete(
    ExtractAuthInfo(auth): ExtractAuthInfo,
    Path(id): Path<i64>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    if let Some(auth) = auth {
        let item = do_select(id)
            .await
            .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?
            .ok_or((
                StatusCode::NOT_FOUND,
                "The user does not exist or has been deleted".to_string(),
            ))?;

        // 不允许操作比自己权限等级更高或者同级的用户
        if auth.permission <= item.permission {
            return Err((StatusCode::FORBIDDEN, "No permission".to_string()));
        }

        do_delete(id, auth.id)
            .await
            .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?;
        return Ok(());
    }

    Err((StatusCode::FORBIDDEN, "No permission".to_string()))
}
