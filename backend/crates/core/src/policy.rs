use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::types::{CorrelationId, Timestamp};

// ---------------------------------------------------------------------------
// Policy decision
// ---------------------------------------------------------------------------

/// The deterministic outcome of the policy engine.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PolicyDecision {
    Allow,
    Deny,
    StepUp,
    ManualReview,
}

/// Full result of a policy evaluation, including audit trail references.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyOutcome {
    pub decision: PolicyDecision,
    pub reason_codes: Vec<String>,
    pub policy_version: String,
    pub agent_findings_used: Vec<Uuid>,
    pub evaluated_at: Timestamp,
    pub trace: CorrelationId,
}

// ---------------------------------------------------------------------------
// Policy rule definition
// ---------------------------------------------------------------------------

/// A single configurable policy rule.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyRule {
    pub id: String,
    pub version: String,
    pub description: String,
    pub priority: u32,
    pub enabled: bool,
}
