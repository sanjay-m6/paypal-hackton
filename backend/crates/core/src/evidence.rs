use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::types::{AgentId, Timestamp};

/// Classification of evidence collected by agents.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceType {
    TransactionLog,
    IdentitySignal,
    DeviceFingerprint,
    BehaviorSequence,
    MerchantRecord,
    PaymentEvent,
    ExternalVerification,
    AuditTrail,
}

/// A reference to a piece of evidence collected during agent analysis.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvidenceRef {
    pub id: Uuid,
    pub source_agent: AgentId,
    pub evidence_type: EvidenceType,
    pub reference: String,
    pub collected_at: Timestamp,
}

