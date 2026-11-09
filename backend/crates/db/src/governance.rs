use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};

/// Canonical retention policy constants enforced across all data layers.
pub struct RetentionPolicy;

impl RetentionPolicy {
    /// 24 Hours for idempotency locks and cached payloads.
    pub const IDEMPOTENCY_HOURS: i64 = 24;

    /// 90 Days for high-frequency session & device telemetry.
    pub const SESSION_SIGNALS_DAYS: i64 = 90;

    /// 180 Days for advisory agent findings and confidence records.
    pub const AGENT_FINDINGS_DAYS: i64 = 180;

    /// 7 Years for immutable financial ledger and audit trails (regulatory compliance).
    pub const FINANCIAL_LEDGER_YEARS: i64 = 7;

    /// Check if an idempotency key created at `timestamp` has passed its 24h retention window.
    pub fn is_idempotency_expired(created_at: DateTime<Utc>, now: DateTime<Utc>) -> bool {
        now - created_at > Duration::hours(Self::IDEMPOTENCY_HOURS)
    }

    /// Check if session telemetry signals created at `timestamp` have passed the 90d window.
    pub fn is_session_signal_expired(created_at: DateTime<Utc>, now: DateTime<Utc>) -> bool {
        now - created_at > Duration::days(Self::SESSION_SIGNALS_DAYS)
    }

    /// Check if agent findings have passed the 180d window.
    pub fn is_agent_finding_expired(created_at: DateTime<Utc>, now: DateTime<Utc>) -> bool {
        now - created_at > Duration::days(Self::AGENT_FINDINGS_DAYS)
    }
}

/// Verification structure for Tenant Isolation Boundaries.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TenantBoundaryScope {
    pub tenant_id: String,
    pub row_level_security_enforced: bool,
    pub cross_tenant_access_blocked: bool,
}

impl TenantBoundaryScope {
    pub fn verify(tenant_id: &str, query_tenant_id: &str) -> bool {
        tenant_id == query_tenant_id
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_idempotency_retention_window() {
        let now = Utc::now();
        let fresh_key = now - Duration::hours(12);
        let expired_key = now - Duration::hours(25);

        assert!(!RetentionPolicy::is_idempotency_expired(fresh_key, now));
        assert!(RetentionPolicy::is_idempotency_expired(expired_key, now));
    }

    #[test]
    fn test_session_signals_retention_window() {
        let now = Utc::now();
        let fresh_signals = now - Duration::days(89);
        let expired_signals = now - Duration::days(91);

        assert!(!RetentionPolicy::is_session_signal_expired(
            fresh_signals,
            now
        ));
        assert!(RetentionPolicy::is_session_signal_expired(
            expired_signals,
            now
        ));
    }

    #[test]
    fn test_agent_findings_retention_window() {
        let now = Utc::now();
        let fresh_finding = now - Duration::days(170);
        let expired_finding = now - Duration::days(181);

        assert!(!RetentionPolicy::is_agent_finding_expired(
            fresh_finding,
            now
        ));
        assert!(RetentionPolicy::is_agent_finding_expired(
            expired_finding,
            now
        ));
    }

    #[test]
    fn test_strict_tenant_isolation_boundary() {
        let tenant_alpha = "tenant_11111111-1111-1111-1111-111111111111";
        let tenant_beta = "tenant_22222222-2222-2222-2222-222222222222";

        // Same tenant query allowed
        assert!(TenantBoundaryScope::verify(tenant_alpha, tenant_alpha));
        // Cross-tenant access strictly rejected
        assert!(!TenantBoundaryScope::verify(tenant_alpha, tenant_beta));
    }
}
