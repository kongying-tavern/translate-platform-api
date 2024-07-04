use anyhow::{anyhow, ensure, Result};

use bcrypt::{hash, verify as do_verify, DEFAULT_COST};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

use super::generate_token;
use crate::{
    models::{user::Permission, *},
    types::response::api::UserInfo,
    DB_CONN,
};

pub fn verify_hash(input_raw: impl ToString, storage_hash: impl ToString) -> Result<()> {
    if let Ok(ret) = do_verify(input_raw.to_string(), storage_hash.to_string().as_str()) {
        if ret {
            return Ok(());
        }
    }
    Err(anyhow!("Failed to verify the password hash"))
}

pub fn generate_hash(password_raw: impl ToString) -> Result<String> {
    Ok(hash(password_raw.to_string(), DEFAULT_COST)?.to_string())
}

pub async fn login(user_name: String, password_hash: String) -> Result<UserInfo> {
    let user = user::Entity::find()
        .filter(user::Column::Name.eq(user_name))
        .one(
            DB_CONN
                .get()
                .ok_or(anyhow!("Failed to get database connection"))?,
        )
        .await?
        .ok_or(anyhow!("Cannot find the user"))?;

    ensure!(
        user.permission != Permission::Guest,
        "Guest user cannot login by password"
    );

    ensure!(
        verify_hash(
            password_hash,
            user.password_hash
                .clone()
                .ok_or(anyhow!("The password hash is not set"))?
        )
        .is_ok(),
        "Wrong password"
    );

    let (token, updated_at) = generate_token(user.clone()).await?;

    Ok(UserInfo {
        token,
        id: user.id,
        name: user.name,
        permission: user.permission,
        updated_at,
    })
}

#[cfg(test)]
mod test {
    #[test]
    fn generate_hash() {
        let list = vec!["admin"];
        for name in list.iter() {
            let hash = super::generate_hash(format!("S1nap@{}", name)).unwrap();
            println!("{}", hash);
        }
    }
}
