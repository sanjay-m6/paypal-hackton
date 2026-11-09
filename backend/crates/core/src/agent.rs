use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::types::{AgentId, Timestamp, TraceContext};

// ---------------------------------------------------------------------------
// Agent metadata & versioning
// ---------------------------------------------------------------------------

/// Semantic version of an agent implementation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct AgentVersion {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
}

impl std::fmt::Display for AgentVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

/// What an agent is capable of doing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AgentCapability {
    TransactionAnalysis,
    IdentityVerification,
    MerchantRisk,
    DeviceSession,
    BehavioralAnalysis,
    PaymentAnalysis,
    CheckoutOptimization,
    CustomerIntelligence,
    Recommendations,
    RefundAnalysis,
    DisputeInvestigation,
    BusinessOps,
    Orchestration,
}

/// Runtime status of an agent execution.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AgentStatus {
    Ready,
    Running,
    Completed,
    Failed,
    TimedOut,
    Skipped,
}

/// Static metadata describing an agent.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentMetadata {
    pub id: AgentId,
    pub version: AgentVersion,
    pub capabilities: Vec<AgentCapability>,
    pub max_execution_ms: u64,
}

// ---------------------------------------------------------------------------
// Request / Response
// ---------------------------------------------------------------------------

/// Payload sent to an agent for processing.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentRequest {
    pub task_id: Uuid,
    pub trace: TraceContext,
    pub agent_id: AgentId,
    pub payload: serde_json::Value,
    pub timeout_ms: u64,
}

/// Result returned by an agent after processing.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentResponse {
    pub task_id: Uuid,
    pub agent_id: AgentId,
    pub agent_version: AgentVersion,
    pub status: AgentStatus,
    pub findings: Vec<AgentFinding>,
    pub evidence: Vec<crate::evidence::EvidenceRef>,
    pub confidence: Option<ConfidenceLevel>,
    pub limitations: Vec<String>,
    pub execution_ms: u64,
    pub timestamp: Timestamp,
}

// ---------------------------------------------------------------------------
// Findings
// ---------------------------------------------------------------------------

/// Category of an agent finding.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FindingType {
    Observation,
    DerivedFeature,
    Hypothesis,
    Recommendation,
    Alert,
}

/// Severity of a finding.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Severity {
    Low,
    Medium,
    High,
    Critical,
}

/// A single insight produced by an agent.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentFinding {
    pub finding_type: FindingType,
    pub description: String,
    pub severity: Severity,
    pub evidence_refs: Vec<Uuid>,
    pub metadata: serde_json::Value,
}

// ---------------------------------------------------------------------------
// Confidence
// ---------------------------------------------------------------------------

/// Bounded confidence value **with mandatory calibration note**.
///
/// > *"Do not present arbitrary LLM confidence as calibrated probability."*
///
/// The `value` is clamped to `[0.0, 1.0]`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfidenceLevel {
    value: f64,
    calibration_note: String,
}

impl ConfidenceLevel {
    /// Create a new confidence level.
    ///
    /// `value` is clamped to the `[0.0, 1.0]` range.
    /// `calibration_note` is **required** — it must explain what the number
    /// means (e.g. "based on historical chargeback correlation").
    pub fn new(value: f64, calibration_note: impl Into<String>) -> Self {
        Self {
            value: value.clamp(0.0, 1.0),
            calibration_note: calibration_note.into(),
        }
    }

    /// Return the clamped confidence value.
    pub fn value(&self) -> f64 {
        self.value
    }

    /// Return the calibration note explaining this confidence.
    pub fn calibration_note(&self) -> &str {
        &self.calibration_note
    }
}

impl PartialEq for ConfidenceLevel {
    fn eq(&self, other: &Self) -> bool {
        (self.value - other.value).abs() < f64::EPSILON
            && self.calibration_note == other.calibration_note
    }
}

