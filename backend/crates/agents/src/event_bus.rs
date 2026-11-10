use std::collections::VecDeque;
use std::sync::Arc;

use chrono::Utc;
use hex;
use payment_core::dto::{AgentMessageEnvelope, AgentMessageHeader};
use payment_core::types::{AgentId, Timestamp};
use payment_core::PlatformError;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use tokio::sync::{broadcast, RwLock};
use uuid::Uuid;

/// Distinct event categories flowing through the agent event bus.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum EventType {
    /// Inbound payment transaction received for analysis.
    TransactionSubmitted,
    /// Master orchestrator initiated multi-agent evaluation pipeline.
    EvaluationStarted,
    /// Work dispatched to a specific specialist agent.
    SpecialistDispatched,
    /// Specialist agent generated a typed risk finding or recommendation.
    FindingPublished,
    /// All specialist agents finished; findings aggregated.
    EvaluationCompleted,
    /// Deterministic Rust policy engine emitted the authoritative decision.
    PolicyDecisionEmitted,
    /// Security anomaly, injection attempt, or timeout detected.
    SecurityAlert,
}

/// Structured event envelope transported over the broadcast event bus.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentEvent {
    pub envelope: AgentMessageEnvelope,
    pub event_type: EventType,
    pub tenant_id: Uuid,
}

impl AgentEvent {
    /// Helper to construct a fully validated and hashed event.
    pub fn create(
        event_type: EventType,
        tenant_id: Uuid,
        task_id: Uuid,
        correlation_id: Uuid,
        sender: AgentId,
        recipient: Option<AgentId>,
        payload: serde_json::Value,
        ttl_secs: i64,
    ) -> Self {
        let now = Utc::now();
        let expires_at = now + chrono::Duration::seconds(ttl_secs);
        let payload_hash = compute_payload_hash(&payload);

        let header = AgentMessageHeader {
            message_id: Uuid::new_v4(),
            task_id,
            correlation_id,
            sender,
            recipient,
            schema_version: "1.0.0".to_string(),
            timestamp: Timestamp(now),
            expires_at: Timestamp(expires_at),
            payload_hash_sha256: payload_hash,
        };

        Self {
            envelope: AgentMessageEnvelope { header, payload },
            event_type,
            tenant_id,
        }
    }
}

/// Computes deterministic SHA256 hex digest of JSON payload (Rule 2.6).
pub fn compute_payload_hash(payload: &serde_json::Value) -> String {
    let bytes = serde_json::to_vec(payload).unwrap_or_default();
    let mut hasher = Sha256::new();
    hasher.update(&bytes);
    hex::encode(hasher.finalize())
}

/// Verifies cryptographic envelope integrity and lifetime bounds (Rules 2.6 & 2.7).
pub fn verify_envelope_integrity(envelope: &AgentMessageEnvelope) -> Result<(), PlatformError> {
    let now = Utc::now();

    // 1. Expiration check
    if envelope.header.expires_at.0 < now {
        return Err(PlatformError::SecurityViolation(format!(
            "Message {} expired at {}",
            envelope.header.message_id, envelope.header.expires_at.0
        )));
    }

    // 2. Correlation & Task ID invariants
    if envelope.header.correlation_id.is_nil() || envelope.header.task_id.is_nil() {
        return Err(PlatformError::SecurityViolation(
            "Message missing required non-nil task_id or correlation_id".to_string(),
        ));
    }

    // 3. Payload SHA256 hash match
    let computed_hash = compute_payload_hash(&envelope.payload);
    if computed_hash != envelope.header.payload_hash_sha256 {
        return Err(PlatformError::SecurityViolation(format!(
            "Payload tamper detected! Header hash: {}, Calculated: {}",
            envelope.header.payload_hash_sha256, computed_hash
        )));
    }

    Ok(())
}

/// Event bus runtime telemetry metrics.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EventBusMetrics {
    pub total_published: u64,
    pub total_delivered: u64,
    pub integrity_failures: u64,
    pub expired_dropped: u64,
}

/// Configuration options for the asynchronous event bus.
#[derive(Debug, Clone)]
pub struct EventBusConfig {
    pub buffer_capacity: usize,
    pub history_limit: usize,
}

impl Default for EventBusConfig {
    fn default() -> Self {
        Self {
            buffer_capacity: 1024,
            history_limit: 100,
        }
    }
}

/// High-throughput, bounded asynchronous event bus for agent communication.
#[derive(Debug, Clone)]
pub struct EventBus {
    sender: broadcast::Sender<AgentEvent>,
    recent_history: Arc<RwLock<VecDeque<AgentEvent>>>,
    metrics: Arc<RwLock<EventBusMetrics>>,
    history_limit: usize,
}

impl EventBus {
    /// Create a new event bus with default buffer capacities.
    pub fn new() -> Self {
        Self::with_config(EventBusConfig::default())
    }

    /// Create an event bus with explicit configuration.
    pub fn with_config(config: EventBusConfig) -> Self {
        let (sender, _) = broadcast::channel(config.buffer_capacity);
        Self {
            sender,
            recent_history: Arc::new(RwLock::new(VecDeque::with_capacity(config.history_limit))),
            metrics: Arc::new(RwLock::new(EventBusMetrics::default())),
            history_limit: config.history_limit,
        }
    }

    /// Publish an event across all active subscriber channels after integrity verification.
    pub async fn publish(&self, event: AgentEvent) -> Result<usize, PlatformError> {
        // Enforce message integrity & non-tampering (Rule 2.6 & 2.7)
        if let Err(err) = verify_envelope_integrity(&event.envelope) {
            let mut m = self.metrics.write().await;
            m.integrity_failures += 1;
            tracing::warn!(?err, "Rejected invalid agent event envelope");
            return Err(err);
        }

        // Retain in recent history ring buffer
        {
            let mut history = self.recent_history.write().await;
            if history.len() >= self.history_limit {
                history.pop_front();
            }
            history.push_back(event.clone());
        }

        // Increment published counter
        {
            let mut m = self.metrics.write().await;
            m.total_published += 1;
        }

        // Non-blocking broadcast dispatch
        match self.sender.send(event) {
            Ok(receiver_count) => {
                let mut m = self.metrics.write().await;
                m.total_delivered += receiver_count as u64;
                Ok(receiver_count)
            }
            Err(_) => {
                // No active receivers is normal if no components are currently listening
                Ok(0)
            }
        }
    }

    /// Subscribe to all broadcasted events across the system.
    pub fn subscribe(&self) -> broadcast::Receiver<AgentEvent> {
        self.sender.subscribe()
    }

    /// Retrieve the most recent $N$ events from the in-memory ring buffer.
    pub async fn get_recent(&self, limit: usize) -> Vec<AgentEvent> {
        let history = self.recent_history.read().await;
        history.iter().rev().take(limit).cloned().collect()
    }

    /// Retrieve recent events matching a specific correlation ID.
    pub async fn get_recent_by_correlation(&self, correlation_id: Uuid) -> Vec<AgentEvent> {
        let history = self.recent_history.read().await;
        history
            .iter()
            .filter(|e| e.envelope.header.correlation_id == correlation_id)
            .cloned()
            .collect()
    }

    /// Retrieve recent events bounded by tenant ID (enforcing tenant boundary).
    pub async fn get_recent_by_tenant(&self, tenant_id: Uuid) -> Vec<AgentEvent> {
        let history = self.recent_history.read().await;
        history
            .iter()
            .filter(|e| e.tenant_id == tenant_id)
            .cloned()
            .collect()
    }

    /// Get current runtime telemetry metrics.
    pub async fn get_metrics(&self) -> EventBusMetrics {
        self.metrics.read().await.clone()
    }
}

impl Default for EventBus {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------------
// Unit Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[tokio::test]
    async fn test_event_bus_publish_and_subscribe() {
        let bus = EventBus::new();
        let mut rx1 = bus.subscribe();
        let mut rx2 = bus.subscribe();

        let event = AgentEvent::create(
            EventType::TransactionSubmitted,
            Uuid::new_v4(),
            Uuid::new_v4(),
            Uuid::new_v4(),
            AgentId::MasterOrchestrator,
            Some(AgentId::TransactionAnalysis),
            json!({"amount": 149.99, "currency": "USD"}),
            60,
        );

        let count = bus.publish(event.clone()).await.unwrap();
        assert_eq!(count, 2);

        let received1 = rx1.recv().await.unwrap();
        let received2 = rx2.recv().await.unwrap();

        assert_eq!(received1.event_type, EventType::TransactionSubmitted);
        assert_eq!(received2.event_type, EventType::TransactionSubmitted);
        assert_eq!(
            received1.envelope.header.message_id,
            received2.envelope.header.message_id
        );
    }

    #[tokio::test]
    async fn test_event_bus_tamper_detection() {
        let bus = EventBus::new();

        let mut event = AgentEvent::create(
            EventType::SpecialistDispatched,
            Uuid::new_v4(),
            Uuid::new_v4(),
            Uuid::new_v4(),
            AgentId::MasterOrchestrator,
            Some(AgentId::IdentityVerification),
            json!({"action": "evaluate_velocity"}),
            60,
        );

        // Tamper with payload without updating header hash
        event.envelope.payload = json!({"action": "evaluate_velocity_TAMPERED"});

        let result = bus.publish(event).await;
        assert!(result.is_err());
        match result.unwrap_err() {
            PlatformError::SecurityViolation(msg) => {
                assert!(msg.contains("Payload tamper detected"));
            }
            other => panic!("Expected SecurityViolation, got {:?}", other),
        }

        let metrics = bus.get_metrics().await;
        assert_eq!(metrics.integrity_failures, 1);
    }

    #[tokio::test]
    async fn test_event_bus_expired_event_rejection() {
        let bus = EventBus::new();

        let event = AgentEvent::create(
            EventType::FindingPublished,
            Uuid::new_v4(),
            Uuid::new_v4(),
            Uuid::new_v4(),
            AgentId::MerchantRisk,
            None,
            json!({"finding": "low_risk"}),
            -10, // Expired 10 seconds ago
        );

        let result = bus.publish(event).await;
        assert!(result.is_err());
        match result.unwrap_err() {
            PlatformError::SecurityViolation(msg) => {
                assert!(msg.contains("expired"));
            }
            other => panic!("Expected SecurityViolation, got {:?}", other),
        }
    }

    #[tokio::test]
    async fn test_event_bus_recent_history_retention() {
        let bus = EventBus::with_config(EventBusConfig {
            buffer_capacity: 16,
            history_limit: 3,
        });

        let tenant_id = Uuid::new_v4();
        let correlation_id = Uuid::new_v4();

        for i in 0..5 {
            let event = AgentEvent::create(
                EventType::SpecialistDispatched,
                tenant_id,
                Uuid::new_v4(),
                correlation_id,
                AgentId::MasterOrchestrator,
                Some(AgentId::TransactionAnalysis),
                json!({"step": i}),
                60,
            );
            bus.publish(event).await.unwrap();
        }

        let recent = bus.get_recent(10).await;
        // History limit capped at 3
        assert_eq!(recent.len(), 3);

        let tenant_events = bus.get_recent_by_tenant(tenant_id).await;
        assert_eq!(tenant_events.len(), 3);

        let correlation_events = bus.get_recent_by_correlation(correlation_id).await;
        assert_eq!(correlation_events.len(), 3);
    }
}
