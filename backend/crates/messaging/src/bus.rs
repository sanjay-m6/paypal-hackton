use std::{future::Future, sync::Arc, time::Duration};

use async_nats::{
    ConnectOptions, HeaderMap,
    jetstream::{
        self,
        consumer::{AckPolicy, DeliverPolicy, pull},
        message::AckKind,
        stream::{Config as StreamConfig, RetentionPolicy, StorageType},
    },
};
use chrono::Utc;
use futures_util::StreamExt;
use payment_core::{
    AgentId, AgentMessageEnvelope, AgentMessageType, MessagePrincipal, types::TenantId,
};
use payment_db::messages::{InboxClaim, InboxOutboxCompletion, MessageRepository};
use serde::Serialize;
use thiserror::Error;
use tokio::time::timeout;

use crate::{
    retry::retry_delay,
    validation::{
        MessageValidationError, MessageValidator, agent_subject, dead_letter_subject,
        result_subject,
    },
};

const TASK_SUBJECTS: [&str; 13] = [
    "agent.task.master_orchestrator",
    "agent.task.transaction_analysis",
    "agent.task.identity_verification",
    "agent.task.merchant_risk",
    "agent.task.device_session",
    "agent.task.behavioral_analysis",
    "agent.task.payment_analysis",
    "agent.task.checkout_optimization",
    "agent.task.customer_intelligence",
    "agent.task.recommendations",
    "agent.task.refund_analysis",
    "agent.task.dispute_investigation",
    "agent.task.business_ops",
];

#[derive(Clone)]
pub struct MessagingConfig {
    pub nats_url: String,
    pub username: Option<String>,
    pub password: Option<String>,
    pub require_tls: bool,
    pub stream_name: String,
    pub dead_letter_stream_name: String,
    pub max_payload_bytes: usize,
    pub message_ttl: Duration,
    pub max_deliveries: u32,
    pub retry_base_delay: Duration,
    pub retry_max_delay: Duration,
    pub handler_timeout: Duration,
    pub connect_timeout: Duration,
}

impl std::fmt::Debug for MessagingConfig {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("MessagingConfig")
            .field("nats_url", &"[REDACTED]")
            .field("username", &self.username.as_ref().map(|_| "[REDACTED]"))
            .field("password", &self.password.as_ref().map(|_| "[REDACTED]"))
            .field("require_tls", &self.require_tls)
            .field("stream_name", &self.stream_name)
            .field("dead_letter_stream_name", &self.dead_letter_stream_name)
            .field("max_payload_bytes", &self.max_payload_bytes)
            .field("message_ttl", &self.message_ttl)
            .field("max_deliveries", &self.max_deliveries)
            .field("retry_base_delay", &self.retry_base_delay)
            .field("retry_max_delay", &self.retry_max_delay)
            .field("handler_timeout", &self.handler_timeout)
            .field("connect_timeout", &self.connect_timeout)
            .finish()
    }
}

impl MessagingConfig {
    pub fn from_env() -> Result<Self, MessagingConfigError> {
        let nats_url = env_value("NATS_URL").unwrap_or_else(|| "nats://127.0.0.1:4222".into());
        let username = env_value("NATS_USERNAME");
        let password = env_value("NATS_PASSWORD");
        let require_tls = env_bool("NATS_REQUIRE_TLS", false)?;
        if nats_url.contains('@') || nats_url.contains('?') {
            return Err(MessagingConfigError::Invalid(
                "credentials and query parameters are not allowed in NATS_URL",
            ));
        }
        if username.is_some() != password.is_some() {
            return Err(MessagingConfigError::Invalid(
                "NATS_USERNAME and NATS_PASSWORD must be set together",
            ));
        }
        if !require_tls && !is_loopback_nats_url(&nats_url) {
            return Err(MessagingConfigError::Invalid(
                "plaintext NATS is allowed only for a loopback NATS_URL",
            ));
        }
        if !is_loopback_nats_url(&nats_url) && username.is_none() {
            return Err(MessagingConfigError::Invalid(
                "NATS_USERNAME and NATS_PASSWORD are required for non-loopback NATS",
            ));
        }
        let stream_name =
            env_value("NATS_STREAM_NAME").unwrap_or_else(|| "PAYMENT_AGENT_EVENTS".into());
        let dead_letter_stream_name = env_value("NATS_DEAD_LETTER_STREAM_NAME")
            .unwrap_or_else(|| "PAYMENT_AGENT_DEAD_LETTERS".into());
        if !valid_stream_name(&stream_name)
            || !valid_stream_name(&dead_letter_stream_name)
            || stream_name == dead_letter_stream_name
        {
            return Err(MessagingConfigError::Invalid(
                "NATS stream names must be distinct and contain only letters, digits, underscores, or hyphens",
            ));
        }
        let max_payload_bytes = env_parse("NATS_MAX_PAYLOAD_BYTES", 64 * 1024)?;
        let message_ttl = Duration::from_secs(env_parse("NATS_MESSAGE_TTL_SECONDS", 300)?);
        let max_deliveries = env_parse("NATS_MAX_DELIVERIES", 5)?;
        let retry_base_delay = Duration::from_millis(env_parse("NATS_RETRY_BASE_DELAY_MS", 250)?);
        let retry_max_delay = Duration::from_millis(env_parse("NATS_RETRY_MAX_DELAY_MS", 30_000)?);
        let handler_timeout = Duration::from_secs(env_parse("NATS_HANDLER_TIMEOUT_SECONDS", 60)?);
        let connect_timeout = Duration::from_secs(env_parse("NATS_CONNECT_TIMEOUT_SECONDS", 5)?);
        if max_payload_bytes == 0
            || max_payload_bytes > 64 * 1024
            || message_ttl.is_zero()
            || message_ttl > Duration::from_secs(300)
            || max_deliveries == 0
            || retry_base_delay.is_zero()
            || retry_max_delay < retry_base_delay
            || handler_timeout.is_zero()
            || handler_timeout > Duration::from_secs(90)
            || connect_timeout.is_zero()
        {
            return Err(MessagingConfigError::Invalid(
                "message size, TTL, delivery limit, and retry delays must be positive and ordered",
            ));
        }

        Ok(Self {
            nats_url,
            username,
            password,
            require_tls,
            stream_name,
            dead_letter_stream_name,
            max_payload_bytes,
            message_ttl,
            max_deliveries,
            retry_base_delay,
            retry_max_delay,
            handler_timeout,
            connect_timeout,
        })
    }
}

#[derive(Debug, Error)]
pub enum MessagingConfigError {
    #[error("Invalid messaging configuration: {0}")]
    Invalid(&'static str),
    #[error("Invalid value for {0}")]
    InvalidValue(&'static str),
}

#[derive(Debug, Error)]
pub enum MessagingError {
    #[error("NATS operation failed: {0}")]
    Nats(String),
    #[error("Message serialization failed")]
    Serialization,
    #[error("Message payload exceeds configured size")]
    MessageTooLarge,
    #[error("Message validation failed: {0}")]
    Validation(#[from] MessageValidationError),
    #[error("Message has no supported destination")]
    InvalidRoute,
    #[error("Message is unsigned")]
    UnsignedMessage,
    #[error("Message consumer configuration is invalid")]
    InvalidConsumer,
    #[error("Message delivery metadata is invalid")]
    InvalidMessageMetadata,
    #[error("Messaging configuration is invalid")]
    InvalidConfiguration,
    #[error("NATS connection timed out")]
    ConnectTimeout,
    #[error("Message persistence failed: {0}")]
    Persistence(String),
}

#[derive(Debug, Clone, Serialize)]
pub struct MessagingStatus {
    pub connected: bool,
    pub stream_ready: bool,
    pub stream_name: String,
    pub dead_letter_stream_ready: bool,
    pub dead_letter_stream_name: String,
}

#[derive(Clone)]
pub struct JetStreamBus {
    jetstream: jetstream::Context,
    config: Arc<MessagingConfig>,
}

impl std::fmt::Debug for JetStreamBus {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("JetStreamBus")
            .field("stream_name", &self.config.stream_name)
            .finish_non_exhaustive()
    }
}

impl JetStreamBus {
    pub async fn connect(config: MessagingConfig) -> Result<Self, MessagingError> {
        validate_messaging_config(&config)?;
        let mut options = ConnectOptions::new().require_tls(config.require_tls);
        if let (Some(username), Some(password)) = (&config.username, &config.password) {
            options = options.user_and_password(username.clone(), password.clone());
        }
        let client = timeout(config.connect_timeout, options.connect(&config.nats_url))
            .await
            .map_err(|_| MessagingError::ConnectTimeout)?
            .map_err(nats_error)?;
        let jetstream = jetstream::new(client);
        let bus = Self {
            jetstream,
            config: Arc::new(config),
        };
        bus.ensure_stream().await?;
        Ok(bus)
    }

    pub async fn status(&self) -> MessagingStatus {
        let stream_ready = self
            .jetstream
            .get_stream(&self.config.stream_name)
            .await
            .is_ok();
        let dead_letter_stream_ready = self
            .jetstream
            .get_stream(&self.config.dead_letter_stream_name)
            .await
            .is_ok();
        MessagingStatus {
            connected: stream_ready && dead_letter_stream_ready,
            stream_ready,
            stream_name: self.config.stream_name.clone(),
            dead_letter_stream_ready,
            dead_letter_stream_name: self.config.dead_letter_stream_name.clone(),
        }
    }

    pub async fn publish(
        &self,
        envelope: &AgentMessageEnvelope,
        validator: &MessageValidator,
    ) -> Result<(), MessagingError> {
        validator.validate(envelope, Utc::now())?;
        let subject = subject_for_envelope(envelope)?;
        if envelope.signature.is_empty() || envelope.key_id.is_empty() {
            return Err(MessagingError::UnsignedMessage);
        }
        let payload = serde_json::to_vec(envelope).map_err(|_| MessagingError::Serialization)?;
        if payload.len() > self.config.max_payload_bytes {
            return Err(MessagingError::MessageTooLarge);
        }
        if !envelope.payload_matches_type() {
            return Err(MessagingError::Serialization);
        }
        let ack = self
            .jetstream
            .send_publish(
                subject,
                jetstream::message::PublishMessage::build()
                    .payload(payload.into())
                    .message_id(envelope.message_id.to_string()),
            )
            .await
            .map_err(nats_error)?;
        ack.await.map_err(nats_error)?;
        Ok(())
    }

    pub async fn consume_agent_tasks<F, Fut>(
        &self,
        agent_id: AgentId,
        durable_name: &str,
        validator: Arc<MessageValidator>,
        repository: MessageRepository,
        mut handler: F,
    ) -> Result<(), MessagingError>
    where
        F: FnMut(AgentMessageEnvelope) -> Fut,
        Fut: Future<Output = Result<AgentMessageEnvelope, ProcessingFailure>>,
    {
        if !valid_consumer_name(durable_name) {
            return Err(MessagingError::InvalidConsumer);
        }
        let stream = self
            .jetstream
            .get_stream(&self.config.stream_name)
            .await
            .map_err(nats_error)?;
        let consumer = stream
            .get_or_create_consumer(
                durable_name,
                pull::Config {
                    durable_name: Some(durable_name.to_owned()),
                    description: Some(format!("Phase 7 agent worker for {agent_id:?}")),
                    deliver_policy: DeliverPolicy::All,
                    ack_policy: AckPolicy::Explicit,
                    ack_wait: self.config.handler_timeout + Duration::from_secs(60),
                    max_deliver: i64::from(self.config.max_deliveries),
                    filter_subject: agent_subject(agent_id),
                    ..Default::default()
                },
            )
            .await
            .map_err(nats_error)?;
        let mut messages = consumer.messages().await.map_err(nats_error)?;
        while let Some(delivery) = messages.next().await {
            let message = delivery.map_err(nats_error)?;
            let delivery_attempt = message.info().map_err(nats_error)?.delivered;
            if delivery_attempt <= 0 {
                return Err(MessagingError::InvalidMessageMetadata);
            }
            let envelope = match serde_json::from_slice::<AgentMessageEnvelope>(&message.payload) {
                Ok(envelope) => envelope,
                Err(_) => {
                    self.dead_letter(&message.subject, None, delivery_attempt, "invalid_envelope")
                        .await?;
                    message.ack().await.map_err(nats_error)?;
                    continue;
                }
            };

            if envelope.recipient_id != MessagePrincipal::Agent(agent_id)
                || message.subject.as_str() != agent_subject(agent_id)
            {
                self.dead_letter(
                    &message.subject,
                    Some(envelope.message_id),
                    delivery_attempt,
                    "route_mismatch",
                )
                .await?;
                message.ack().await.map_err(nats_error)?;
                continue;
            }

            if validator.validate(&envelope, Utc::now()).is_err() {
                self.dead_letter(
                    &message.subject,
                    Some(envelope.message_id),
                    delivery_attempt,
                    "validation_failed",
                )
                .await?;
                message.ack().await.map_err(nats_error)?;
                continue;
            }

            let envelope_value =
                serde_json::to_value(&envelope).map_err(|_| MessagingError::Serialization)?;
            let lease_seconds = i64::try_from(self.config.handler_timeout.as_secs())
                .map_err(|_| MessagingError::InvalidConfiguration)?
                + 90;
            match repository
                .claim_inbox(
                    envelope.tenant_id,
                    envelope.message_id,
                    envelope.task_id,
                    &envelope_value,
                    chrono::Duration::seconds(lease_seconds),
                )
                .await
                .map_err(persistence_error)?
            {
                InboxClaim::AlreadyProcessed => {
                    message.ack().await.map_err(nats_error)?;
                    continue;
                }
                InboxClaim::AlreadyFailed => {
                    repository
                        .record_dead_letter(
                            envelope.tenant_id,
                            Some(envelope.message_id),
                            Some(envelope.task_id),
                            &message.subject,
                            i32::try_from(delivery_attempt)
                                .map_err(|_| MessagingError::InvalidMessageMetadata)?,
                            "previously_failed",
                        )
                        .await
                        .map_err(persistence_error)?;
                    self.dead_letter(
                        &message.subject,
                        Some(envelope.message_id),
                        delivery_attempt,
                        "previously_failed",
                    )
                    .await?;
                    message.ack().await.map_err(nats_error)?;
                    continue;
                }
                InboxClaim::Conflict => {
                    repository
                        .record_dead_letter(
                            envelope.tenant_id,
                            Some(envelope.message_id),
                            Some(envelope.task_id),
                            &message.subject,
                            i32::try_from(delivery_attempt)
                                .map_err(|_| MessagingError::InvalidMessageMetadata)?,
                            "inbox_id_conflict",
                        )
                        .await
                        .map_err(persistence_error)?;
                    self.dead_letter(
                        &message.subject,
                        Some(envelope.message_id),
                        delivery_attempt,
                        "inbox_id_conflict",
                    )
                    .await?;
                    message.ack().await.map_err(nats_error)?;
                    continue;
                }
                InboxClaim::InProgress => {
                    message
                        .ack_with(AckKind::Nak(Some(self.config.retry_base_delay)))
                        .await
                        .map_err(nats_error)?;
                    continue;
                }
                InboxClaim::Acquired => {}
            }

            let result = match timeout(self.config.handler_timeout, handler(envelope.clone())).await
            {
                Ok(result) => result,
                Err(_) => Err(ProcessingFailure {
                    category: "handler_timeout",
                    retryable: true,
                }),
            };
            match result {
                Ok(response)
                    if valid_task_response(&envelope, &response, agent_id)
                        && validator.validate(&response, Utc::now()).is_ok() =>
                {
                    let response_subject = subject_for_envelope(&response)?;
                    let response_value = serde_json::to_value(&response)
                        .map_err(|_| MessagingError::Serialization)?;
                    let completed = repository
                        .complete_inbox_with_outbox(InboxOutboxCompletion {
                            tenant_id: TenantId(envelope.tenant_id.0),
                            inbound_message_id: envelope.message_id,
                            task_id: envelope.task_id,
                            inbound_envelope: &envelope_value,
                            outbound_message_id: response.message_id,
                            outbound_subject: &response_subject,
                            outbound_envelope: &response_value,
                        })
                        .await
                        .map_err(persistence_error)?;
                    if completed {
                        message.ack().await.map_err(nats_error)?;
                    } else {
                        message
                            .ack_with(AckKind::Nak(Some(self.config.retry_base_delay)))
                            .await
                            .map_err(nats_error)?;
                    }
                }
                Ok(_) => {
                    if !repository
                        .fail_inbox(envelope.tenant_id, envelope.message_id, "invalid_response")
                        .await
                        .map_err(persistence_error)?
                    {
                        return Err(MessagingError::Persistence(
                            "Invalid task response could not be recorded".into(),
                        ));
                    }
                    repository
                        .record_dead_letter(
                            envelope.tenant_id,
                            Some(envelope.message_id),
                            Some(envelope.task_id),
                            &message.subject,
                            i32::try_from(delivery_attempt)
                                .map_err(|_| MessagingError::InvalidMessageMetadata)?,
                            "invalid_response",
                        )
                        .await
                        .map_err(persistence_error)?;
                    self.dead_letter(
                        &message.subject,
                        Some(envelope.message_id),
                        delivery_attempt,
                        "invalid_response",
                    )
                    .await?;
                    message.ack().await.map_err(nats_error)?;
                }
                Err(failure)
                    if failure.retryable
                        && delivery_attempt < i64::from(self.config.max_deliveries) =>
                {
                    if !repository
                        .release_inbox(envelope.tenant_id, envelope.message_id, failure.category)
                        .await
                        .map_err(persistence_error)?
                    {
                        return Err(MessagingError::Persistence(
                            "Message retry lease could not be released".into(),
                        ));
                    }
                    let delay = retry_delay(
                        u32::try_from(delivery_attempt)
                            .map_err(|_| MessagingError::InvalidMessageMetadata)?,
                        self.config.retry_base_delay,
                        self.config.retry_max_delay,
                    );
                    message
                        .ack_with(AckKind::Nak(Some(delay)))
                        .await
                        .map_err(nats_error)?;
                }
                Err(failure) => {
                    if !repository
                        .fail_inbox(envelope.tenant_id, envelope.message_id, failure.category)
                        .await
                        .map_err(persistence_error)?
                    {
                        return Err(MessagingError::Persistence(
                            "Message failure state could not be recorded".into(),
                        ));
                    }
                    repository
                        .record_dead_letter(
                            envelope.tenant_id,
                            Some(envelope.message_id),
                            Some(envelope.task_id),
                            &message.subject,
                            i32::try_from(delivery_attempt)
                                .map_err(|_| MessagingError::InvalidMessageMetadata)?,
                            failure.category,
                        )
                        .await
                        .map_err(persistence_error)?;
                    self.dead_letter(
                        &message.subject,
                        Some(envelope.message_id),
                        delivery_attempt,
                        failure.category,
                    )
                    .await?;
                    message.ack().await.map_err(nats_error)?;
                }
            }
        }
        Ok(())
    }

    async fn ensure_stream(&self) -> Result<(), MessagingError> {
        self.jetstream
            .get_or_create_stream(StreamConfig {
                name: self.config.stream_name.clone(),
                subjects: TASK_SUBJECTS
                    .into_iter()
                    .chain([result_subject()])
                    .map(str::to_owned)
                    .collect(),
                retention: RetentionPolicy::Limits,
                storage: StorageType::File,
                max_age: self.config.message_ttl,
                max_message_size: i32::try_from(self.config.max_payload_bytes).unwrap_or(i32::MAX),
                duplicate_window: Duration::from_secs(120),
                num_replicas: 1,
                ..Default::default()
            })
            .await
            .map_err(nats_error)?;
        self.jetstream
            .get_or_create_stream(StreamConfig {
                name: self.config.dead_letter_stream_name.clone(),
                subjects: vec![dead_letter_subject().to_owned()],
                retention: RetentionPolicy::Limits,
                storage: StorageType::File,
                max_age: Duration::from_secs(90 * 24 * 60 * 60),
                max_message_size: 16 * 1024,
                num_replicas: 1,
                ..Default::default()
            })
            .await
            .map_err(nats_error)?;
        Ok(())
    }

    async fn dead_letter(
        &self,
        original_subject: &str,
        message_id: Option<uuid::Uuid>,
        delivery_attempt: i64,
        reason: &'static str,
    ) -> Result<(), MessagingError> {
        #[derive(Serialize)]
        struct DeadLetter<'a> {
            message_id: Option<uuid::Uuid>,
            original_subject: &'a str,
            delivery_attempt: i64,
            reason: &'static str,
            recorded_at: chrono::DateTime<Utc>,
        }
        let payload = serde_json::to_vec(&DeadLetter {
            message_id,
            original_subject,
            delivery_attempt,
            reason,
            recorded_at: Utc::now(),
        })
        .map_err(|_| MessagingError::Serialization)?;
        let mut headers = HeaderMap::new();
        headers.append("Nats-Msg-Id", uuid::Uuid::new_v4().to_string());
        let ack = self
            .jetstream
            .publish_with_headers(dead_letter_subject(), headers, payload.into())
            .await
            .map_err(nats_error)?;
        ack.await.map_err(nats_error)?;
        Ok(())
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ProcessingFailure {
    pub category: &'static str,
    pub retryable: bool,
}

pub fn subject_for_envelope(envelope: &AgentMessageEnvelope) -> Result<String, MessagingError> {
    match (envelope.message_type, envelope.recipient_id) {
        (AgentMessageType::AgentTaskRequest, MessagePrincipal::Agent(agent_id)) => {
            Ok(agent_subject(agent_id))
        }
        (
            AgentMessageType::AgentTaskCompleted | AgentMessageType::AgentTaskFailed,
            MessagePrincipal::Orchestrator,
        ) => Ok(result_subject().to_owned()),
        _ => Err(MessagingError::InvalidRoute),
    }
}

fn valid_consumer_name(name: &str) -> bool {
    !name.is_empty()
        && name.len() <= 64
        && name
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_'))
}

fn valid_stream_name(name: &str) -> bool {
    !name.is_empty()
        && name.len() <= 64
        && name
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'_' | b'-'))
}

fn env_value(name: &'static str) -> Option<String> {
    std::env::var(name)
        .ok()
        .map(|value| value.trim().to_owned())
        .filter(|value| !value.is_empty())
}

fn env_parse<T>(name: &'static str, default: T) -> Result<T, MessagingConfigError>
where
    T: std::str::FromStr,
{
    match env_value(name) {
        Some(value) => value
            .parse()
            .map_err(|_| MessagingConfigError::InvalidValue(name)),
        None => Ok(default),
    }
}

fn env_bool(name: &'static str, default: bool) -> Result<bool, MessagingConfigError> {
    match env_value(name).as_deref() {
        Some("true") => Ok(true),
        Some("false") => Ok(false),
        Some(_) => Err(MessagingConfigError::InvalidValue(name)),
        None => Ok(default),
    }
}

fn is_loopback_nats_url(url: &str) -> bool {
    ["nats://127.0.0.1:", "nats://localhost:", "nats://[::1]:"]
        .iter()
        .any(|prefix| url.starts_with(prefix))
}

fn nats_error(error: impl std::fmt::Display) -> MessagingError {
    MessagingError::Nats(error.to_string())
}

fn persistence_error(error: impl std::fmt::Display) -> MessagingError {
    MessagingError::Persistence(error.to_string())
}

fn valid_task_response(
    request: &AgentMessageEnvelope,
    response: &AgentMessageEnvelope,
    agent_id: AgentId,
) -> bool {
    response.tenant_id == request.tenant_id
        && response.task_id == request.task_id
        && response.correlation_id == request.correlation_id
        && response.trace_id == request.trace_id
        && response.sender_id == MessagePrincipal::Agent(agent_id)
        && response.recipient_id == MessagePrincipal::Orchestrator
        && matches!(
            response.message_type,
            AgentMessageType::AgentTaskCompleted | AgentMessageType::AgentTaskFailed
        )
        && response.payload_matches_type()
}

fn validate_messaging_config(config: &MessagingConfig) -> Result<(), MessagingError> {
    let remote = !is_loopback_nats_url(&config.nats_url);
    if config.nats_url.contains('@')
        || config.nats_url.contains('?')
        || (!config.require_tls && remote)
        || (remote && config.username.is_none())
        || (config.username.is_some() != config.password.is_some())
        || !valid_stream_name(&config.stream_name)
        || !valid_stream_name(&config.dead_letter_stream_name)
        || config.stream_name == config.dead_letter_stream_name
        || config.max_payload_bytes == 0
        || config.max_payload_bytes > 64 * 1024
        || config.message_ttl.is_zero()
        || config.message_ttl > Duration::from_secs(300)
        || config.max_deliveries == 0
        || config.retry_base_delay.is_zero()
        || config.retry_max_delay < config.retry_base_delay
        || config.handler_timeout.is_zero()
        || config.handler_timeout > Duration::from_secs(90)
        || config.connect_timeout.is_zero()
    {
        return Err(MessagingError::InvalidConfiguration);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use payment_core::{
        AgentMessagePayload, AgentPayloadType, AgentRequest,
        types::{Timestamp, TraceContext, UserId},
    };

    #[test]
    fn consumer_name_is_restricted_to_safe_characters() {
        assert!(valid_consumer_name("transaction-analysis"));
        assert!(!valid_consumer_name("transaction.analysis"));
        assert!(!valid_consumer_name(""));
    }

    #[test]
    fn task_response_must_match_the_claimed_tenant_and_trace() {
        let tenant_id = TenantId(uuid::Uuid::new_v4());
        let task_id = uuid::Uuid::new_v4();
        let trace = TraceContext::new(tenant_id, UserId(uuid::Uuid::new_v4()));
        let now = Utc::now();
        let request = AgentMessageEnvelope {
            message_id: uuid::Uuid::new_v4(),
            task_id,
            tenant_id,
            sender_id: MessagePrincipal::Orchestrator,
            recipient_id: MessagePrincipal::Agent(AgentId::TransactionAnalysis),
            message_type: AgentMessageType::AgentTaskRequest,
            payload_type: AgentPayloadType::AgentTask,
            schema_version: "1.0".into(),
            issued_at: Timestamp(now),
            expires_at: Timestamp(now + chrono::Duration::minutes(1)),
            correlation_id: trace.correlation_id.0,
            trace_id: uuid::Uuid::new_v4(),
            payload: AgentMessagePayload::AgentTask {
                request: AgentRequest {
                    task_id,
                    trace,
                    agent_id: AgentId::TransactionAnalysis,
                    payload: serde_json::json!({}),
                    timeout_ms: 500,
                },
            },
            key_id: "orchestrator-1".into(),
            signature: "signature".into(),
        };
        let response = AgentMessageEnvelope {
            message_id: uuid::Uuid::new_v4(),
            task_id,
            tenant_id,
            sender_id: MessagePrincipal::Agent(AgentId::TransactionAnalysis),
            recipient_id: MessagePrincipal::Orchestrator,
            message_type: AgentMessageType::AgentTaskFailed,
            payload_type: AgentPayloadType::AgentFailure,
            schema_version: "1.0".into(),
            issued_at: Timestamp(now),
            expires_at: Timestamp(now + chrono::Duration::minutes(1)),
            correlation_id: request.correlation_id,
            trace_id: request.trace_id,
            payload: AgentMessagePayload::AgentFailure {
                task_id,
                agent_id: AgentId::TransactionAnalysis,
                failure_category: "analysis_failed".into(),
                retryable: false,
                safe_diagnostic: "Analysis could not be completed".into(),
            },
            key_id: "transaction-analysis-1".into(),
            signature: "signature".into(),
        };
        assert!(valid_task_response(
            &request,
            &response,
            AgentId::TransactionAnalysis
        ));

        let mut mismatched = response;
        mismatched.trace_id = uuid::Uuid::new_v4();
        assert!(!valid_task_response(
            &request,
            &mismatched,
            AgentId::TransactionAnalysis
        ));
    }
}
