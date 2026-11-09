use std::fmt::Debug;
use payment_core::{AgentId, AgentMetadata, AgentRequest, AgentResponse, PlatformError};

/// Trait that all AI agents must implement.
///
/// Agents are the intelligence components that analyze transactions, detect
/// fraud, and produce findings. They are NOT permitted to make financial
/// decisions — that authority belongs exclusively to the PolicyEngine.
#[async_trait::async_trait]
pub trait Agent: Send + Sync + Debug {
    /// Unique identifier for this agent.
    fn id(&self) -> AgentId;

    /// Metadata describing the agent's capabilities and version.
    fn metadata(&self) -> &AgentMetadata;

    /// Execute the agent's analysis on the given request.
    async fn execute(
        &self,
        request: AgentRequest,
    ) -> Result<AgentResponse, PlatformError>;
}
