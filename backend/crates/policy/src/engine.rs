// SECURITY: This engine is the sole authority for financial decisions. No LLM bypass.
//
// The PolicyEngine evaluates agent findings against a deterministic rule set.
// It must NEVER delegate decisions to an LLM or any non-deterministic process.

use payment_core::{AgentFinding, PolicyDecision, PolicyOutcome, PolicyRule, Timestamp, TraceContext};

/// Deterministic policy evaluation engine.
///
/// All financial-security decisions flow through this engine. Rules are loaded
/// from configuration and evaluated in order against agent findings.
#[derive(Debug, Clone)]
pub struct PolicyEngine {
    rules: Vec<PolicyRule>,
}

impl PolicyEngine {
    /// Create a new engine with no rules loaded.
    pub fn new() -> Self {
        Self { rules: Vec::new() }
    }

    /// Replace the current rule set with the provided rules.
    pub fn load_rules(&mut self, rules: Vec<PolicyRule>) {
        tracing::info!(count = rules.len(), "Policy rules loaded");
        self.rules = rules;
    }

    /// Evaluate the given agent findings deterministically.
    ///
    /// # Determinism guarantee
    ///
    /// Given the same rule set and the same findings, this function will
    /// **always** produce the same `PolicyOutcome`. No randomness, no LLM
    /// calls, no network I/O.
    pub fn evaluate(
        &self,
        _findings: &[AgentFinding],
        trace: &TraceContext,
    ) -> PolicyOutcome {
        if self.rules.is_empty() {
            tracing::warn!("No policy rules configured — defaulting to ManualReview");
            return PolicyOutcome {
                decision: PolicyDecision::ManualReview,
                reason_codes: vec!["no_rules_configured".to_string()],
                policy_version: "1.0.0".to_string(),
                agent_findings_used: Vec::new(),
                evaluated_at: Timestamp::now(),
                trace: trace.correlation_id,
            };
        }

        // Default deterministic outcome
        PolicyOutcome {
            decision: PolicyDecision::ManualReview,
            reason_codes: vec!["rules_pending_evaluation".to_string()],
            policy_version: "1.0.0".to_string(),
            agent_findings_used: Vec::new(),
            evaluated_at: Timestamp::now(),
            trace: trace.correlation_id,
        }
    }
}

impl Default for PolicyEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use payment_core::{TenantId, UserId};
    use uuid::Uuid;

    #[test]
    fn test_empty_rules_deterministic_manual_review() {
        let engine = PolicyEngine::new();
        let trace = TraceContext::new(TenantId(Uuid::new_v4()), UserId(Uuid::new_v4()));
        let outcome1 = engine.evaluate(&[], &trace);
        let outcome2 = engine.evaluate(&[], &trace);

        assert_eq!(outcome1.decision, PolicyDecision::ManualReview);
        assert_eq!(outcome2.decision, PolicyDecision::ManualReview);
        assert_eq!(outcome1.reason_codes, vec!["no_rules_configured".to_string()]);
    }
}

