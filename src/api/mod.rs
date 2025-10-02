pub mod v1;

pub async fn router() -> anyhow::Result<axum::Router> {
    let ret = axum::Router::new()
        .nest("/v1", v1::router().await?)
        ;
    Ok(ret)
}