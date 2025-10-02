mod users;

pub async fn router() -> anyhow::Result<axum::Router> {
    let ret = axum::Router::new()
        .nest("/users", users::router().await?)
        ;
    Ok(ret)
}