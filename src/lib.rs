use axum::{Json, Router, routing::get};
use serde::Serialize;
use tower_http::compression::CompressionLayer;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct HealthResponse {
    status: &'static str,
}

async fn health() -> Json<HealthResponse> {
    Json(HealthResponse { status: "ok" })
}

pub fn app() -> Router {
    Router::new()
        .route("/health", get(health))
        .layer(CompressionLayer::new())
}
