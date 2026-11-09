// SECURITY: Only this service calls money-moving endpoints.
// Credentials never shared with agents.

use payment_core::PlatformError;

use crate::types::{CaptureResponse, OrderCreateRequest, OrderCreateResponse};

/// HTTP client for the PayPal Orders API.
#[derive(Debug, Clone)]
pub struct PayPalClient {
    base_url: String,
    client: reqwest::Client,
}

impl PayPalClient {
    /// Create a client pointing at the PayPal sandbox environment.
    pub fn new_sandbox() -> Self {
        Self {
            base_url: "https://api-m.sandbox.paypal.com".to_string(),
            client: reqwest::Client::new(),
        }
    }

    /// Create a client pointing at the PayPal production environment.
    pub fn new_production() -> Self {
        Self {
            base_url: "https://api-m.paypal.com".to_string(),
            client: reqwest::Client::new(),
        }
    }

    /// Create an order. Stub — to be implemented in Phase 31.
    pub async fn create_order(
        &self,
        _request: &OrderCreateRequest,
    ) -> Result<OrderCreateResponse, PlatformError> {
        tracing::warn!("PayPal create_order is a stub");
        Err(PlatformError::Internal(
            "PayPal create_order not yet configured".to_string(),
        ))
    }

    /// Capture a previously approved order. Stub — to be implemented in Phase 31.
    pub async fn capture_order(&self, _order_id: &str) -> Result<CaptureResponse, PlatformError> {
        tracing::warn!("PayPal capture_order is a stub");
        Err(PlatformError::Internal(
            "PayPal capture_order not yet configured".to_string(),
        ))
    }

    /// Returns the configured base URL.
    pub fn base_url(&self) -> &str {
        &self.base_url
    }
}
