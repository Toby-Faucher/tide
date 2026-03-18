#[global_allocator]
static GLOBAL: tikv_jemallocator::Jemalloc = tikv_jemallocator::Jemalloc;

use axum::{Json, Router, routing::get};
use serde::Serialize;
use tokio::net::TcpListener;
use tower_http::compression::CompressionLayer;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct HealthResponse {
    status: &'static str,
}

async fn health() -> Json<HealthResponse> {
    Json(HealthResponse { status: "ok" })
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/health", get(health))
        .layer(CompressionLayer::new());

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
