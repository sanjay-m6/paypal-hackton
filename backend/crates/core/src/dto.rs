use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::payment::{PaymentMethod, PaymentStatus};
use crate::policy::PolicyDecision;
use crate::types::{AgentId, Currency, Timestamp};

// ---------------------------------------------------------------------------
// 1. Payment Evaluation DTOs (Inbound Risk Assessment)
// ---------------------------------------------------------------------------

/// Inbound request payload to evaluate payment risk across the 13 specialist agents.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentEvaluationRequestDto {
    pub transaction_id: Uuid,
    pub tenant_id: Uuid,
    pub user_id: Option<Uuid>,
    pub amount: Decimal,
    pub currency: Currency,
    pub payment_method: PaymentMethod,
    pub customer: CustomerContextDto,
    pub session: Option<SessionContextDto>,
    pub idempotency_key: Uuid,
}

/// Scoped customer signals permitted for intelligence analysis.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomerContextDto {
    pub customer_id: String,
    pub email: String,
    pub phone: Option<String>,
    pub ip_address: String,
    pub country: Option<String>,
    pub account_age_days: Option<u32>,
}

/// Device and session integrity indicators.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionContextDto {
    pub session_id: String,
    pub device_fingerprint: String,
    pub user_agent: String,
    pub is_proxy_or_vpn: bool,
    pub client_timestamp: Timestamp,
}

/// Outbound response containing the deterministic policy outcome and supporting agent findings.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentEvaluationResponseDto {
    pub transaction_id: Uuid,
    pub policy_decision: PolicyDecision,
    pub reason_codes: Vec<String>,
    pub policy_version: String,
    pub agent_findings: Vec<AgentFindingDto>,
    pub evaluated_at: Timestamp,
    pub correlation_id: Uuid,
}

/// Serializable representation of an agent's structured evidence finding.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentFindingDto {
    pub agent_id: AgentId,
    pub agent_version: String,
    pub finding_type: String,
    pub severity: String,
    pub description: String,
    pub confidence_value: Option<f64>,
    pub confidence_note: Option<String>,
    pub evidence_refs: Vec<Uuid>,
}

// ---------------------------------------------------------------------------
// 2. Payment Execution DTOs (Money-Moving PayPal Service)
// ---------------------------------------------------------------------------

/// Request to execute an approved payment against the PayPal Sandbox.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentExecutionRequestDto {
    pub transaction_id: Uuid,
    pub idempotency_key: Uuid,
}

/// Response returned after authoritative PayPal order creation/capture.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentExecutionResponseDto {
    pub transaction_id: Uuid,
    pub status: PaymentStatus,
    pub paypal_order_id: String,
    pub captured_amount: Decimal,
    pub currency: Currency,
    pub executed_at: Timestamp,
}

// ---------------------------------------------------------------------------
// 3. Agent Communication Envelope (Signed Wire Protocol)
// ---------------------------------------------------------------------------

/// Wire message envelope crossing trust boundaries between agents and the orchestrator.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentMessageEnvelope {
    pub header: AgentMessageHeader,
    pub payload: serde_json::Value,
}

/// Cryptographically verifiable message header for cross-agent communication.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentMessageHeader {
    pub message_id: Uuid,
    pub task_id: Uuid,
    pub correlation_id: Uuid,
    pub sender: AgentId,
    pub recipient: Option<AgentId>,
    pub schema_version: String,
    pub timestamp: Timestamp,
    pub expires_at: Timestamp,
    pub payload_hash_sha256: String,
}

// ---------------------------------------------------------------------------
// 4. Agent Descriptor DTO
// ---------------------------------------------------------------------------

/// Public descriptor exposing an agent's version, scope, and capabilities.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentDescriptorDto {
    pub id: u8,
    pub name: String,
    pub version: String,
    pub owner: String,
    pub scope: String,
    pub capabilities: Vec<String>,
    pub timeout_ms: u64,
}
