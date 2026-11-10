use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    agent::{AgentRequest, AgentResponse},
    types::{AgentId, TenantId, Timestamp},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[serde(deny_unknown_fields)]
pub enum MessagePrincipal {
    Api,
    Orchestrator,
    Agent(AgentId),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[serde(deny_unknown_fields)]
pub enum AgentMessageType {
    AgentTaskRequest,
    AgentTaskCompleted,
    AgentTaskFailed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[serde(deny_unknown_fields)]
pub enum AgentPayloadType {
    AgentTask,
    AgentResult,
    AgentFailure,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(
    tag = "payload_type",
    content = "data",
    rename_all = "SCREAMING_SNAKE_CASE"
)]
#[serde(deny_unknown_fields)]
pub enum AgentMessagePayload {
    AgentTask {
        request: AgentRequest,
    },
    AgentResult {
        response: AgentResponse,
    },
    AgentFailure {
        task_id: Uuid,
        agent_id: AgentId,
        failure_category: String,
        retryable: bool,
        safe_diagnostic: String,
    },
}

impl AgentMessagePayload {
    pub fn payload_type(&self) -> AgentPayloadType {
        match self {
            Self::AgentTask { .. } => AgentPayloadType::AgentTask,
            Self::AgentResult { .. } => AgentPayloadType::AgentResult,
            Self::AgentFailure { .. } => AgentPayloadType::AgentFailure,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AgentMessageEnvelope {
    pub message_id: Uuid,
    pub task_id: Uuid,
    pub tenant_id: TenantId,
    pub sender_id: MessagePrincipal,
    pub recipient_id: MessagePrincipal,
    pub message_type: AgentMessageType,
    pub payload_type: AgentPayloadType,
    pub schema_version: String,
    pub issued_at: Timestamp,
    pub expires_at: Timestamp,
    pub correlation_id: Uuid,
    pub trace_id: Uuid,
    pub payload: AgentMessagePayload,
    pub key_id: String,
    pub signature: String,
}

impl AgentMessageEnvelope {
    pub fn payload_matches_type(&self) -> bool {
        self.payload.payload_type() == self.payload_type
            && matches!(
                (self.message_type, self.payload_type),
                (
                    AgentMessageType::AgentTaskRequest,
                    AgentPayloadType::AgentTask
                ) | (
                    AgentMessageType::AgentTaskCompleted,
                    AgentPayloadType::AgentResult
                ) | (
                    AgentMessageType::AgentTaskFailed,
                    AgentPayloadType::AgentFailure
                )
            )
    }
}
