// PayPal adapter types — do not leak into core domain.
//
// These structs model PayPal's REST API request/response shapes and are used
// exclusively inside the `payment-paypal` crate. Core domain types live in
// `payment-core`.

use serde::{Deserialize, Serialize};

/// Request body for creating a PayPal order.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderCreateRequest {
    pub intent: String,
    pub purchase_units: Vec<PurchaseUnit>,
}

/// A single purchase unit within an order.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PurchaseUnit {
    pub reference_id: Option<String>,
    pub amount: Amount,
    pub description: Option<String>,
}

/// Monetary amount with currency code.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Amount {
    pub currency_code: String,
    pub value: String,
}

/// Response from PayPal after creating an order.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderCreateResponse {
    pub id: String,
    pub status: String,
    pub links: Vec<Link>,
}

/// Response from PayPal after capturing an order.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CaptureResponse {
    pub id: String,
    pub status: String,
    pub purchase_units: Vec<CapturedPurchaseUnit>,
}

/// A purchase unit within a capture response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapturedPurchaseUnit {
    pub reference_id: Option<String>,
    pub payments: Option<Payments>,
}

/// Payment details within a captured purchase unit.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Payments {
    pub captures: Vec<Capture>,
}

/// A single capture record.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Capture {
    pub id: String,
    pub status: String,
    pub amount: Amount,
}

/// HATEOAS link returned by PayPal.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Link {
    pub href: String,
    pub rel: String,
    pub method: Option<String>,
}
