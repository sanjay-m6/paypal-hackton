use serde::{Deserialize, Serialize};
use uuid::Uuid;

use payment_core::{CorrelationId, TenantId, Timestamp, UserId};

/// A recorded audit event for compliance and forensic analysis.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditEvent {
    /// Unique identifier for this audit record.
    pub id: Uuid,
    /// Classification of the event.
    pub event_type: AuditEventType,
    /// Tenant that owns the resource involved.
    pub tenant_id: TenantId,
    /// User who triggered the event, if applicable.
    pub user_id: Option<UserId>,
    /// Correlation id linking related events across the platform.
    pub correlation_id: CorrelationId,
    /// Arbitrary structured details about the event.
    pub details: serde_json::Value,
    /// When the event occurred.
    pub timestamp: Timestamp,
}

/// Classification of audit events.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum AuditEventType {
    /// User login, logout, token refresh.
    Authentication,
    /// Permission check or role evaluation.
    Authorization,
    /// A deterministic policy decision was made.
    PolicyDecision,
    /// An AI agent executed and produced findings.
    AgentExecution,
    /// A payment moved through its lifecycle.
    PaymentLifecycle,
    /// A system configuration was changed.
    ConfigChange,
    /// A security-relevant event (e.g., rate-limit breach, suspicious access).
    SecurityEvent,
}
