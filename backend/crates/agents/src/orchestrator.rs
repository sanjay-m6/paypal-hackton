use std::sync::Arc;
use std::time::Duration;

use payment_core::{AgentId, AgentRequest, AgentResponse, PlatformError};
use serde_json::json;

use crate::event_bus::{AgentEvent, EventBus, EventType};
use crate::registry::AgentRegistry;

/// Orchestrator dispatches work to registered agents and enforces timeouts.
#[derive(Debug, Clone)]
pub struct Orchestrator {
    registry: Arc<AgentRegistry>,
    event_bus: Arc<EventBus>,
    /// Maximum time an agent is allowed to run before being cancelled.
    timeout: Duration,
}

impl Orchestrator {
    /// Create a new orchestrator backed by the given registry with a default event bus.
    pub fn new(registry: Arc<AgentRegistry>, timeout: Duration) -> Self {
        Self {
            registry,
            event_bus: Arc::new(EventBus::new()),
            timeout,
        }
    }

    /// Create an orchestrator with explicit event bus and timeout.
    pub fn with_event_bus(
        registry: Arc<AgentRegistry>,
        event_bus: Arc<EventBus>,
        timeout: Duration,
    ) -> Self {
        Self {
            registry,
            event_bus,
            timeout,
        }
    }

    /// Return a reference to the active event bus.
    pub fn event_bus(&self) -> &Arc<EventBus> {
        &self.event_bus
    }

    /// Look up the agent identified by `agent_id` and execute the request,
    /// enforcing the configured timeout and publishing lifecycle events.
    pub async fn execute_task(
        &self,
        agent_id: &AgentId,
        request: AgentRequest,
    ) -> Result<AgentResponse, PlatformError> {
        let agent = self
            .registry
            .get(agent_id)
            .ok_or_else(|| PlatformError::NotFound {
                entity: "Agent".to_string(),
                id: format!("{agent_id:?}"),
            })?;

        // 1. Publish SpecialistDispatched event
        let dispatch_event = AgentEvent::create(
            EventType::SpecialistDispatched,
            request.trace.tenant_id.0,
            request.task_id,
            request.trace.correlation_id.0,
            AgentId::MasterOrchestrator,
            Some(*agent_id),
            json!({
                "agent_id": format!("{agent_id:?}"),
                "timeout_ms": self.timeout.as_millis()
            }),
            60,
        );
        let _ = self.event_bus.publish(dispatch_event).await;

        // 2. Execute with strict Tokio timeout
        let result = tokio::time::timeout(self.timeout, agent.execute(request.clone()))
            .await
            .map_err(|_| {
                PlatformError::AgentError {
                    agent_id: format!("{agent_id:?}"),
                    message: format!("Agent exceeded timeout of {:?}", self.timeout),
                }
            })?;

        // 3. Publish outcome event
        match &result {
            Ok(response) => {
                let finding_event = AgentEvent::create(
                    EventType::FindingPublished,
                    request.trace.tenant_id.0,
                    request.task_id,
                    request.trace.correlation_id.0,
                    *agent_id,
                    Some(AgentId::MasterOrchestrator),
                    json!({
                        "findings_count": response.findings.len(),
                        "status": format!("{:?}", response.status),
                        "execution_ms": response.execution_ms,
                    }),
                    60,
                );
                let _ = self.event_bus.publish(finding_event).await;
            }
            Err(err) => {
                let alert_event = AgentEvent::create(
                    EventType::SecurityAlert,
                    request.trace.tenant_id.0,
                    request.task_id,
                    request.trace.correlation_id.0,
                    *agent_id,
                    Some(AgentId::MasterOrchestrator),
                    json!({
                        "error": format!("{err:?}")
                    }),
                    60,
                );
                let _ = self.event_bus.publish(alert_event).await;
            }
        }

        result
    }
}
