mod model;

// root:/api/v1/oauth
pub async fn router() -> anyhow::Result<axum::Router> {
    let ret = axum::Router::new()
        // .route("/", axum::routing::get(list))
        ;
    Ok(ret)
}