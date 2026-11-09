use std::sync::Arc;

use payment_agents::registry::AgentRegistry;
use payment_audit::logger::AuditLogger;
use payment_policy::engine::PolicyEngine;

/// Shared application state available to all request handlers.
#[derive(Debug, Clone)]
pub struct AppState {
    /// The deterministic policy engine — sole authority for financial decisions.
    pub policy_engine: Arc<PolicyEngine>,
    /// Registry of all available AI agents.
    pub agent_registry: Arc<AgentRegistry>,
    /// Structured audit logger for compliance events.
    pub audit_logger: Arc<AuditLogger>,
}
