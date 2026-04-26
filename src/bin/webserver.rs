
use axum::{
    routing::get,
    Router,
    Json
};

use serde::Serialize;

#[tokio::main]
async fn main() {
    let tracing_worker_guard = plantplot::core::tracing::initialize_tracing("webserver");
    info!("Successfully initialized tracing module!");

    let app = Router::new()
        .route("/", get(root));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("Server running on http://127.0.0.1:3000");
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "Hello from Axum!"
}
