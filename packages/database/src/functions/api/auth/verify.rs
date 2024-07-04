use anyhow::{anyhow, ensure, Context, Result};

use jsonwebtoken::{decode, Validation};
use sea_orm::EntityTrait;

use super::{Claims, JWT_SECRET};
use crate::{models::*, types::response::api::UserInfo, DB_CONN};

pub async fn verify(token: String) -> Result<UserInfo> {
    let token_raw = token.clone();
    let token = decode::<Claims>(&token, &JWT_SECRET.decoding, &Validation::default())
        .context("Invalid token")?;

    let user = user::Entity::find_by_id(token.claims.user_id)
        .one(
            DB_CONN
                .get()
                .ok_or(anyhow!("Failed to get database connection"))?,
        )
        .await?
        .ok_or(anyhow!("Cannot find the user"))?;

    let iat = token.claims.iat;
    let updated_at = user.clone().updated_at;
    ensure!(iat >= updated_at, "Token expired");

    Ok(UserInfo {
        token: token_raw,
        id: user.id,
        name: user.name,
        permission: user.permission,
        updated_at,
    })
}
