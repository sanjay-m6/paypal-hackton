use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::policy::PolicyOutcome;
use crate::types::{Money, TenantId, Timestamp, TransactionId, UserId};

// ---------------------------------------------------------------------------
// Payment status (state machine)
// ---------------------------------------------------------------------------

/// Lifecycle status of a payment.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PaymentStatus {
    Created,
    Authorized,
    Captured,
    Refunded,
    PartiallyRefunded,
    Voided,
    Failed,
    Disputed,
}

impl PaymentStatus {
    /// Return the set of states that are valid transitions **from** `self`.
    ///
    /// This is the single source of truth for the payment state machine.
    pub fn valid_transitions(&self) -> &'static [PaymentStatus] {
        match self {
            Self::Created => &[Self::Authorized, Self::Failed],
            Self::Authorized => &[Self::Captured, Self::Voided, Self::Failed],
            Self::Captured => &[
                Self::Refunded,
                Self::PartiallyRefunded,
                Self::Disputed,
            ],
            Self::PartiallyRefunded => &[
                Self::Refunded,
                Self::PartiallyRefunded,
                Self::Disputed,
            ],
            Self::Refunded => &[],
            Self::Voided => &[],
            Self::Failed => &[],
            Self::Disputed => &[Self::Refunded, Self::Captured],
        }
    }

    /// Check whether transitioning from `self` to `target` is allowed.
    pub fn can_transition_to(&self, target: PaymentStatus) -> bool {
        self.valid_transitions().contains(&target)
    }
}

// ---------------------------------------------------------------------------
// Payment method
// ---------------------------------------------------------------------------

/// Supported payment methods.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PaymentMethod {
    #[serde(rename = "PAYPAL", alias = "PAY_PAL")]
    PayPal,
    Card,
    BankTransfer,
}

// ---------------------------------------------------------------------------
// Payment request & record
// ---------------------------------------------------------------------------

/// Inbound payment request from a tenant.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentRequest {
    pub transaction_id: TransactionId,
    pub tenant_id: TenantId,
    pub user_id: UserId,
    pub amount: Money,
    pub method: PaymentMethod,
    pub idempotency_key: Uuid,
    pub metadata: serde_json::Value,
}

/// Persisted payment record.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentRecord {
    pub id: TransactionId,
    pub tenant_id: TenantId,
    pub status: PaymentStatus,
    pub amount: Money,
    pub method: PaymentMethod,
    pub external_id: Option<String>,
    pub policy_outcome: Option<PolicyOutcome>,
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_payment_state_transitions() {
        assert!(PaymentStatus::Created.can_transition_to(PaymentStatus::Authorized));
        assert!(PaymentStatus::Created.can_transition_to(PaymentStatus::Failed));
        assert!(!PaymentStatus::Created.can_transition_to(PaymentStatus::Captured));

        assert!(PaymentStatus::Authorized.can_transition_to(PaymentStatus::Captured));
        assert!(PaymentStatus::Authorized.can_transition_to(PaymentStatus::Voided));
        assert!(!PaymentStatus::Authorized.can_transition_to(PaymentStatus::Refunded));

        assert!(PaymentStatus::Captured.can_transition_to(PaymentStatus::Refunded));
        assert!(PaymentStatus::Captured.can_transition_to(PaymentStatus::Disputed));
        assert!(!PaymentStatus::Captured.can_transition_to(PaymentStatus::Authorized));

        // Terminal states
        assert!(PaymentStatus::Refunded.valid_transitions().is_empty());
        assert!(PaymentStatus::Voided.valid_transitions().is_empty());
        assert!(PaymentStatus::Failed.valid_transitions().is_empty());
    }
}


