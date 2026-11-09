use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// ---------------------------------------------------------------------------
// Newtype wrappers
// ---------------------------------------------------------------------------

/// Tenant identifier — guarantees tenant-isolation at the type level.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TenantId(pub Uuid);

/// User identifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct UserId(pub Uuid);

/// Transaction identifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TransactionId(pub Uuid);

/// Correlation identifier for distributed tracing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CorrelationId(pub Uuid);

/// UTC timestamp newtype.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Timestamp(pub DateTime<Utc>);

impl Timestamp {
    /// Return the current UTC time.
    pub fn now() -> Self {
        Self(Utc::now())
    }
}

// ---------------------------------------------------------------------------
// AgentId — one variant per agent in the platform
// ---------------------------------------------------------------------------

/// Identifies one of the 13 platform agents.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[repr(u8)]
pub enum AgentId {
    MasterOrchestrator = 0,
    TransactionAnalysis = 1,
    IdentityVerification = 2,
    MerchantRisk = 3,
    DeviceSession = 4,
    BehavioralAnalysis = 5,
    PaymentAnalysis = 6,
    CheckoutOptimization = 7,
    CustomerIntelligence = 8,
    Recommendations = 9,
    RefundAnalysis = 10,
    DisputeInvestigation = 11,
    BusinessOps = 12,
}

// ---------------------------------------------------------------------------
// Currency & Money
// ---------------------------------------------------------------------------

/// ISO 4217 currency codes supported by the platform.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Currency {
    Usd,
    Eur,
    Gbp,
    Inr,
    Jpy,
    Aud,
    Cad,
    Sgd,
}

/// Typed monetary value — prevents untyped numeric amounts from leaking
/// through the system.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Money {
    pub amount: Decimal,
    pub currency: Currency,
}

impl Money {
    pub fn new(amount: Decimal, currency: Currency) -> Self {
        Self { amount, currency }
    }
}

// ---------------------------------------------------------------------------
// Trace context (distributed tracing)
// ---------------------------------------------------------------------------

/// Carries correlation / span metadata through every request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TraceContext {
    pub correlation_id: CorrelationId,
    pub span_id: Uuid,
    pub parent_span_id: Option<Uuid>,
    pub tenant_id: TenantId,
    pub user_id: UserId,
    pub timestamp: Timestamp,
}

impl TraceContext {
    /// Create a new root trace context.
    pub fn new(tenant_id: TenantId, user_id: UserId) -> Self {
        Self {
            correlation_id: CorrelationId(Uuid::new_v4()),
            span_id: Uuid::new_v4(),
            parent_span_id: None,
            tenant_id,
            user_id,
            timestamp: Timestamp::now(),
        }
    }

    /// Derive a child span from this context.
    pub fn child_span(&self) -> Self {
        Self {
            correlation_id: self.correlation_id,
            span_id: Uuid::new_v4(),
            parent_span_id: Some(self.span_id),
            tenant_id: self.tenant_id,
            user_id: self.user_id,
            timestamp: Timestamp::now(),
        }
    }
}
