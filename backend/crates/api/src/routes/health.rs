use axum::Json;
use serde_json::{json, Value};

/// Health check endpoint.
///
/// Returns a simple JSON payload confirming the service is running.
pub async fn health_check() -> Json<Value> {
    Json(json!({
        "status": "healthy",
        "version": "0.1.0"
    }))
}
