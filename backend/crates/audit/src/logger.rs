use crate::types::AuditEvent;

/// Structured audit logger.
///
/// Emits audit events as structured tracing spans/events for now.
// TODO: Persist to database in Phase 10
#[derive(Debug, Clone)]
pub struct AuditLogger;

impl AuditLogger {
    /// Create a new audit logger.
    pub fn new() -> Self {
        Self
    }

    /// Record an audit event.
    ///
    /// Currently emits the event via `tracing::info!`. In Phase 10 this will
    /// persist to the audit table in the database.
    // TODO: Persist to database in Phase 10
    pub async fn log(&self, event: AuditEvent) {
        tracing::info!(
            audit_event_id = %event.id,
            event_type = ?event.event_type,
            tenant_id = ?event.tenant_id,
            correlation_id = ?event.correlation_id,
            timestamp = ?event.timestamp,
            "Audit event recorded"
        );
    }
}

impl Default for AuditLogger {
    fn default() -> Self {
        Self::new()
    }
}
