use anyhow::{anyhow, ensure, Context, Result};

use jsonwebtoken::{decode, Validation};
use sea_orm::EntityTrait;

use super::{generate_token, Claims, JWT_SECRET};
use crate::{models::*, types::response::api::UserInfo, DB_CONN};

pub async fn refresh(token: String) -> Result<UserInfo> {
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
    let updated_at_db = user.clone().updated_at - chrono::Duration::minutes(1);
    ensure!(iat >= updated_at_db, "Token expired");

    let (token, updated_at) = generate_token(user.clone()).await?;

    Ok(UserInfo {
        token,
        id: user.id,
        name: user.name,
        permission: user.permission,
        updated_at,
    })
}
