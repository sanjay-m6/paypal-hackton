use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Unified error type for the entire platform.
#[derive(Debug, Clone, Error, Serialize, Deserialize)]
pub enum PlatformError {
    #[error("Validation error: {0}")]
    Validation(String),

    #[error("{entity} not found: {id}")]
    NotFound { entity: String, id: String },

    #[error("Unauthorized: {0}")]
    Unauthorized(String),

    #[error("Forbidden: {0}")]
    Forbidden(String),

    #[error("Tenant isolation violation: {0}")]
    TenantIsolation(String),

    #[error("Policy violation: {0}")]
    PolicyViolation(String),

    #[error("Payment error: {0}")]
    PaymentError(String),

    #[error("Agent error [{agent_id}]: {message}")]
    AgentError { agent_id: String, message: String },

    #[error("Database error: {0}")]
    DatabaseError(String),

    #[error("External service error [{service}]: {message}")]
    ExternalService { service: String, message: String },

    #[error("Rate limited")]
    RateLimited,

    #[error("Internal error: {0}")]
    Internal(String),
}

impl PlatformError {
    /// Return a sanitised message safe to send to external clients.
    /// Internal details (DB errors, stack traces, etc.) are never leaked.
    pub fn to_safe_message(&self) -> &str {
        match self {
            Self::Validation(_) => "Invalid request data.",
            Self::NotFound { .. } => "The requested resource was not found.",
            Self::Unauthorized(_) => "Authentication is required.",
            Self::Forbidden(_) => "You do not have permission to perform this action.",
            Self::TenantIsolation(_) => "Access denied.",
            Self::PolicyViolation(_) => "The request was denied by policy.",
            Self::PaymentError(_) => "A payment processing error occurred.",
            Self::AgentError { .. } => "An internal analysis error occurred.",
            Self::DatabaseError(_) => "An internal error occurred.",
            Self::ExternalService { .. } => "An upstream service error occurred.",
            Self::RateLimited => "Too many requests. Please try again later.",
            Self::Internal(_) => "An internal error occurred.",
        }
    }

    /// Convenience: is this an error the caller could potentially fix?
    pub fn is_client_error(&self) -> bool {
        matches!(
            self,
            Self::Validation(_)
                | Self::NotFound { .. }
                | Self::Unauthorized(_)
                | Self::Forbidden(_)
                | Self::RateLimited
        )
    }
}

/// Alias used throughout the platform.
pub type PlatformResult<T> = Result<T, PlatformError>;
