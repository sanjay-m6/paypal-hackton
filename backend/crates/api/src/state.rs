use std::sync::Arc;

use payment_agents::registry::AgentRegistry;
use payment_audit::logger::AuditLogger;
use payment_db::pool::DbPool;
use payment_messaging::{JetStreamBus, MessageValidator};
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
    /// Structured audit logger for compliance events.
    pub audit_logger: Arc<AuditLogger>,
    /// Optional durable message bus; disabled unless explicitly configured.
    pub messaging: Option<Arc<JetStreamBus>>,
    /// Validates signed agent envelopes before they are dispatched or published.
    pub messaging_validator: Option<Arc<MessageValidator>>,
}
