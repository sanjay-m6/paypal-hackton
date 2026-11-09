use axum::{
    Json,
    extract::rejection::JsonRejection,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use payment_core::PlatformError;
use serde::Serialize;

#[derive(Debug)]
pub struct ApiError(pub PlatformError);

#[derive(Serialize)]
struct ErrorResponse {
    error: &'static str,
    message: String,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let status = match &self.0 {
            PlatformError::Validation(_) => StatusCode::BAD_REQUEST,
            PlatformError::Unauthorized(_) => StatusCode::UNAUTHORIZED,
            PlatformError::Forbidden(_) | PlatformError::TenantIsolation(_) => {
                StatusCode::FORBIDDEN
            }
            PlatformError::NotFound { .. } => StatusCode::NOT_FOUND,
            PlatformError::RateLimited => StatusCode::TOO_MANY_REQUESTS,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        };

        let code = status.canonical_reason().unwrap_or("Internal Server Error");
        let message = self.0.to_safe_message().to_owned();

        (
            status,
            Json(ErrorResponse {
                error: code,
                message,
            }),
        )
            .into_response()
    }
}

impl From<PlatformError> for ApiError {
    fn from(error: PlatformError) -> Self {
        Self(error)
    }
}

impl From<JsonRejection> for ApiError {
    fn from(_: JsonRejection) -> Self {
        Self(PlatformError::Validation(
            "The request body is invalid or missing".into(),
        ))
    }
}
