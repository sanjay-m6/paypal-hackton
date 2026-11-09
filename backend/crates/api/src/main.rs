use axum::{
    routing::{get, post},
    Router,
};
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "info".into()),
        )
        .init();

    tracing::info!("Starting Multi-Agent Payment Security Platform API (Axum)");

    // Configure CORS for frontend access
    let cors = CorsLayer::permissive();

    // Build router with Phase 1 & Phase 2 endpoints
    let app = Router::new()
        .route("/health", get(payment_api::routes::health::health_check))
        .route("/api/v1/health", get(payment_api::routes::health::health_check))
        .route("/api/v1/agents", get(payment_api::routes::agents::list_agents))
        .route(
            "/api/v1/payments/evaluate",
            post(payment_api::routes::payments::evaluate_payment),
        )
        .layer(cors)
        .layer(TraceLayer::new_for_http());

    // Start server
    let host = std::env::var("API_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = std::env::var("API_PORT").unwrap_or_else(|_| "3001".to_string());
    let addr = format!("{host}:{port}");

    tracing::info!("Listening on {addr}");
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .expect("Failed to bind to address");
    axum::serve(listener, app)
        .await
        .expect("Server error");
}
