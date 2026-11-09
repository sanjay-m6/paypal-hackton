use std::sync::Arc;
use std::time::Duration;

use payment_core::{AgentId, AgentRequest, AgentResponse, PlatformError};

use crate::registry::AgentRegistry;

/// Orchestrator dispatches work to registered agents and enforces timeouts.
#[derive(Debug, Clone)]
pub struct Orchestrator {
    registry: Arc<AgentRegistry>,
    /// Maximum time an agent is allowed to run before being cancelled.
    timeout: Duration,
}

impl Orchestrator {
    /// Create a new orchestrator backed by the given registry.
    pub fn new(registry: Arc<AgentRegistry>, timeout: Duration) -> Self {
        Self { registry, timeout }
    }

    /// Look up the agent identified by `agent_id` and execute the request,
    /// enforcing the configured timeout.
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

        let result = tokio::time::timeout(self.timeout, agent.execute(request))
            .await
            .map_err(|_| {
                PlatformError::AgentError {
                    agent_id: format!("{agent_id:?}"),
                    message: format!("Agent exceeded timeout of {:?}", self.timeout),
                }
            })?;

        result
    }
}
