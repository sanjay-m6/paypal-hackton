pub mod middleware;
pub mod routes;
pub mod state;

use axum::{
    Router,
    extract::DefaultBodyLimit,
    routing::{get, post},
};
use tower_http::{cors::CorsLayer, trace::TraceLayer};

use state::AppState;

pub fn build_router(state: AppState) -> Router {
    let cors = CorsLayer::new()
        .allow_origin([
            axum::http::HeaderValue::from_static("http://localhost:5173"),
            axum::http::HeaderValue::from_static("http://127.0.0.1:5173"),
        ])
        .allow_methods([axum::http::Method::GET, axum::http::Method::POST])
        .allow_headers([
            axum::http::header::AUTHORIZATION,
            axum::http::header::CONTENT_TYPE,
        ]);

    Router::new()
        .route("/health", get(routes::health::health_check))
        .route("/api/v1/health", get(routes::health::health_check))
        .route("/api/v1/auth/login", post(routes::auth::login))
        .route("/api/v1/agents", get(routes::agents::list_agents))
        .route(
            "/api/v1/payments/evaluate",
            post(routes::payments::evaluate_payment),
        )
        .with_state(state)
        .layer(cors)
        .layer(DefaultBodyLimit::max(32 * 1024))
        .layer(TraceLayer::new_for_http())
}
