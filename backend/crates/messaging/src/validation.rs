use std::time::Duration;

use chrono::{DateTime, Utc};
use payment_core::{
    AgentId, AgentMessageEnvelope, AgentMessagePayload, AgentMessageType, AgentPayloadType,
    MessagePrincipal,
};
use thiserror::Error;

use crate::auth::{MessageAuthError, MessageKeyring};

pub const MESSAGE_SCHEMA_VERSION: &str = "1.0";

#[derive(Debug, Clone)]
pub struct MessageValidationConfig {
    pub max_payload_bytes: usize,
    pub max_ttl: Duration,
    pub allowed_clock_skew: Duration,
}

impl Default for MessageValidationConfig {
    fn default() -> Self {
        Self {
            max_payload_bytes: 64 * 1024,
            max_ttl: Duration::from_secs(300),
            allowed_clock_skew: Duration::from_secs(30),
        }
    }
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum MessageValidationError {
    #[error("Message authentication failed: {0}")]
    Authentication(#[from] MessageAuthError),
    #[error("Unsupported schema version")]
    UnsupportedVersion,
    #[error("Message has expired or has invalid timestamps")]
    InvalidLifetime,
    #[error("Message payload exceeds configured size")]
    PayloadTooLarge,
    #[error("Message payload type does not match the event type")]
    PayloadTypeMismatch,
    #[error("Sender is not authorized for this recipient and event")]
    UnauthorizedRoute,
    #[error("Message task, tenant, or trace context does not match its payload")]
    ContextMismatch,
    #[error("Message fields are invalid")]
    InvalidFields,
}

#[derive(Debug)]
pub struct MessageValidator {
    keyring: MessageKeyring,
    config: MessageValidationConfig,
}

impl MessageValidator {
    pub fn new(keyring: MessageKeyring, config: MessageValidationConfig) -> Self {
        Self { keyring, config }
    }

    pub fn validate(
        &self,
        envelope: &AgentMessageEnvelope,
        now: DateTime<Utc>,
    ) -> Result<MessagePrincipal, MessageValidationError> {
        let authenticated_sender = self.keyring.verify(envelope)?;
        if envelope.schema_version != MESSAGE_SCHEMA_VERSION {
            return Err(MessageValidationError::UnsupportedVersion);
        }
        if envelope.key_id.is_empty() || envelope.key_id.len() > 64 {
            return Err(MessageValidationError::InvalidFields);
        }
        if !envelope.payload_matches_type() {
            return Err(MessageValidationError::PayloadTypeMismatch);
        }
        let issued_at = envelope.issued_at.0;
        let expires_at = envelope.expires_at.0;
        let skew = chrono::Duration::from_std(self.config.allowed_clock_skew)
            .map_err(|_| MessageValidationError::InvalidLifetime)?;
        let max_ttl = chrono::Duration::from_std(self.config.max_ttl)
            .map_err(|_| MessageValidationError::InvalidLifetime)?;
        let lifetime = expires_at - issued_at;
        if issued_at > now + skew
            || expires_at <= now
            || expires_at <= issued_at
            || lifetime > max_ttl
        {
            return Err(MessageValidationError::InvalidLifetime);
        }
        let payload = serde_json::to_vec(&envelope.payload)
            .map_err(|_| MessageValidationError::InvalidFields)?;
        if payload.len() > self.config.max_payload_bytes {
            return Err(MessageValidationError::PayloadTooLarge);
        }
        self.validate_route(envelope, authenticated_sender)?;
        self.validate_context(envelope)?;
        Ok(authenticated_sender)
    }

    fn validate_route(
        &self,
        envelope: &AgentMessageEnvelope,
        sender: MessagePrincipal,
    ) -> Result<(), MessageValidationError> {
        let authorized = matches!(
            (sender, envelope.recipient_id, envelope.message_type),
            (
                MessagePrincipal::Orchestrator,
                MessagePrincipal::Agent(_),
                AgentMessageType::AgentTaskRequest
            ) | (
                MessagePrincipal::Agent(_),
                MessagePrincipal::Orchestrator,
                AgentMessageType::AgentTaskCompleted
            ) | (
                MessagePrincipal::Agent(_),
                MessagePrincipal::Orchestrator,
                AgentMessageType::AgentTaskFailed
            )
        );
        if !authorized {
            return Err(MessageValidationError::UnauthorizedRoute);
        }
        Ok(())
    }

    fn validate_context(
        &self,
        envelope: &AgentMessageEnvelope,
    ) -> Result<(), MessageValidationError> {
        match &envelope.payload {
            AgentMessagePayload::AgentTask { request } => {
                let MessagePrincipal::Agent(recipient) = envelope.recipient_id else {
                    return Err(MessageValidationError::UnauthorizedRoute);
                };
                if request.task_id != envelope.task_id
                    || request.agent_id != recipient
                    || request.trace.tenant_id != envelope.tenant_id
                    || request.trace.correlation_id.0 != envelope.correlation_id
                    || request.trace.span_id != envelope.trace_id
                {
                    return Err(MessageValidationError::ContextMismatch);
                }
            }
            AgentMessagePayload::AgentResult { response } => {
                let MessagePrincipal::Agent(sender) = envelope.sender_id else {
                    return Err(MessageValidationError::UnauthorizedRoute);
                };
                if response.task_id != envelope.task_id || response.agent_id != sender {
                    return Err(MessageValidationError::ContextMismatch);
                }
            }
            AgentMessagePayload::AgentFailure {
                task_id,
                agent_id,
                failure_category,
                safe_diagnostic,
                ..
            } => {
                let MessagePrincipal::Agent(sender) = envelope.sender_id else {
                    return Err(MessageValidationError::UnauthorizedRoute);
                };
                if *task_id != envelope.task_id
                    || *agent_id != sender
                    || failure_category.trim().is_empty()
                    || failure_category.len() > 64
                    || safe_diagnostic.len() > 512
                {
                    return Err(MessageValidationError::ContextMismatch);
                }
            }
        }
        if matches!(envelope.payload_type, AgentPayloadType::AgentTask)
            && envelope.message_type != AgentMessageType::AgentTaskRequest
        {
            return Err(MessageValidationError::PayloadTypeMismatch);
        }
        Ok(())
    }
}

pub fn agent_subject(agent_id: AgentId) -> String {
    format!("agent.task.{}", agent_id_subject_name(agent_id))
}

pub fn result_subject() -> &'static str {
    "agent.result.orchestrator"
}

pub fn dead_letter_subject() -> &'static str {
    "agent.dead-letter"
}

pub fn agent_id_subject_name(agent_id: AgentId) -> &'static str {
    match agent_id {
        AgentId::MasterOrchestrator => "master_orchestrator",
        AgentId::TransactionAnalysis => "transaction_analysis",
        AgentId::IdentityVerification => "identity_verification",
        AgentId::MerchantRisk => "merchant_risk",
        AgentId::DeviceSession => "device_session",
        AgentId::BehavioralAnalysis => "behavioral_analysis",
        AgentId::PaymentAnalysis => "payment_analysis",
        AgentId::CheckoutOptimization => "checkout_optimization",
        AgentId::CustomerIntelligence => "customer_intelligence",
        AgentId::Recommendations => "recommendations",
        AgentId::RefundAnalysis => "refund_analysis",
        AgentId::DisputeInvestigation => "dispute_investigation",
        AgentId::BusinessOps => "business_ops",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::auth::MessageKey;
    use chrono::Duration as ChronoDuration;
    use payment_core::{
        AgentMessagePayload, AgentMessageType, AgentPayloadType, AgentRequest, CorrelationId,
        TenantId, Timestamp, TraceContext, UserId,
    };
    use uuid::Uuid;

    fn valid_request(now: DateTime<Utc>) -> AgentMessageEnvelope {
        let tenant_id = TenantId(Uuid::new_v4());
        let correlation_id = Uuid::new_v4();
        let task_id = Uuid::new_v4();
        let trace_id = Uuid::new_v4();
        let trace = TraceContext {
            correlation_id: CorrelationId(correlation_id),
            span_id: trace_id,
            parent_span_id: None,
            tenant_id,
            user_id: UserId(Uuid::new_v4()),
            timestamp: Timestamp(now),
        };
        AgentMessageEnvelope {
            message_id: Uuid::new_v4(),
            task_id,
            tenant_id,
            sender_id: MessagePrincipal::Orchestrator,
            recipient_id: MessagePrincipal::Agent(AgentId::TransactionAnalysis),
            message_type: AgentMessageType::AgentTaskRequest,
            payload_type: AgentPayloadType::AgentTask,
            schema_version: MESSAGE_SCHEMA_VERSION.into(),
            issued_at: Timestamp(now),
            expires_at: Timestamp(now + ChronoDuration::seconds(60)),
            correlation_id,
            trace_id,
            payload: AgentMessagePayload::AgentTask {
                request: AgentRequest {
                    task_id,
                    trace,
                    agent_id: AgentId::TransactionAnalysis,
                    payload: serde_json::json!({"amount_minor": 1250}),
                    timeout_ms: 500,
                },
            },
            key_id: String::new(),
            signature: String::new(),
        }
    }

    fn validator(principal: MessagePrincipal) -> MessageValidator {
        let key = MessageKey::new("test-key", principal, vec![9; 32]).expect("key");
        MessageValidator::new(
            MessageKeyring::new([key]).expect("keyring"),
            MessageValidationConfig::default(),
        )
    }

    #[test]
    fn accepts_valid_typed_message_and_rejects_expired_message() {
        let now = Utc::now();
        let validator = validator(MessagePrincipal::Orchestrator);
        let keyring = MessageKeyring::new([MessageKey::new(
            "test-key",
            MessagePrincipal::Orchestrator,
            vec![9; 32],
        )
        .expect("key")])
        .expect("keyring");
        let mut message = valid_request(now);
        keyring.sign(&mut message, "test-key").expect("sign");
        assert_eq!(
            validator.validate(&message, now),
            Ok(MessagePrincipal::Orchestrator)
        );

        message.expires_at = Timestamp(now - ChronoDuration::seconds(1));
        keyring.sign(&mut message, "test-key").expect("resign");
        assert_eq!(
            validator.validate(&message, now),
            Err(MessageValidationError::InvalidLifetime)
        );
    }

    #[test]
    fn rejects_unsupported_schema_and_mismatched_payload_context() {
        let now = Utc::now();
        let keyring = MessageKeyring::new([MessageKey::new(
            "test-key",
            MessagePrincipal::Orchestrator,
            vec![9; 32],
        )
        .expect("key")])
        .expect("keyring");
        let validator = validator(MessagePrincipal::Orchestrator);

        let mut message = valid_request(now);
        message.schema_version = "2.0".into();
        keyring.sign(&mut message, "test-key").expect("sign");
        assert_eq!(
            validator.validate(&message, now),
            Err(MessageValidationError::UnsupportedVersion)
        );

        let mut message = valid_request(now);
        message.task_id = Uuid::new_v4();
        keyring.sign(&mut message, "test-key").expect("sign");
        assert_eq!(
            validator.validate(&message, now),
            Err(MessageValidationError::ContextMismatch)
        );
    }

    #[test]
    fn serializes_strict_envelope_and_rejects_unknown_fields() {
        let message = valid_request(Utc::now());
        let mut value = serde_json::to_value(message).expect("serialize");
        value["unexpected"] = serde_json::json!(true);
        assert!(serde_json::from_value::<AgentMessageEnvelope>(value).is_err());
    }

    #[test]
    fn task_subject_is_scoped_to_agent() {
        assert_eq!(
            agent_subject(AgentId::TransactionAnalysis),
            "agent.task.transaction_analysis"
        );
    }
}
