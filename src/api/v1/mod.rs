pub mod users;
pub mod oauth;

pub async fn router() -> anyhow::Result<axum::Router> {
    let ret = axum::Router::new()
        .nest("/users", users::router().await?)
        .nest("/oauth", oauth::router().await?);
    Ok(ret)
}