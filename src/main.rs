use axum::{Router, routing::get};
mod views;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/", get(views::home::get))
        .route("/_health", get(health));

    let port = std::env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let bind = format!("0.0.0.0:{}", &port);

    let listener = tokio::net::TcpListener::bind(&bind).await.unwrap();

    tracing::info!("listening on 0.0.0.0:{}", port);
    axum::serve(listener, app).await.expect("server stopped");
}

async fn health() -> &'static str {
    "OK"
}
