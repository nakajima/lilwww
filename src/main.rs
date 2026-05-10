mod error;
mod views;

use anyhow::Context;
use axum::{Router, routing::get};
use error::AppResult;
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/", get(views::home::get))
        .route("/_health", get(health))
        .nest_service("/assets", ServeDir::new("assets"));

    let port = std::env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let bind = format!("0.0.0.0:{}", &port);

    let listener = tokio::net::TcpListener::bind(&bind)
        .await
        .with_context(|| format!("binding to {bind}"))?;

    tracing::info!("listening on 0.0.0.0:{}", port);
    axum::serve(listener, app).await.context("serving app")?;

    Ok(())
}

async fn health() -> AppResult<&'static str> {
    Ok("OK")
}
