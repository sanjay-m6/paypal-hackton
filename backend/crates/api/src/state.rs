use std::sync::Arc;

use payment_agents::{event_bus::EventBus, registry::AgentRegistry};
use payment_audit::logger::AuditLogger;
use payment_db::pool::DbPool;
use payment_policy::engine::PolicyEngine;

use crate::middleware::auth::AuthService;

/// Shared application state available to all request handlers.
#[derive(Debug, Clone)]
pub struct AppState {
    /// Database connection pool shared by API repositories.
    pub db: DbPool,
    /// Issues and validates short-lived API access tokens.
    pub auth: AuthService,
    /// The deterministic policy engine — sole authority for financial decisions.
    pub policy_engine: Arc<PolicyEngine>,
    /// Registry of all available AI agents.
    pub agent_registry: Arc<AgentRegistry>,
    /// High-throughput asynchronous agent communication and event bus.
    pub event_bus: Arc<EventBus>,
    /// Structured audit logger for compliance events.
    pub audit_logger: Arc<AuditLogger>,
}
