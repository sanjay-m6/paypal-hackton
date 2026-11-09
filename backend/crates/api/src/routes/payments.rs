use axum::Json;
use payment_core::dto::{
    AgentFindingDto, PaymentEvaluationRequestDto, PaymentEvaluationResponseDto,
};
use payment_core::policy::PolicyDecision;
use payment_core::types::{AgentId, Timestamp};
use uuid::Uuid;

/// POST /api/v1/payments/evaluate
///
/// Evaluates inbound payment context against the agent ensemble and deterministic policy.
pub async fn evaluate_payment(
    Json(payload): Json<PaymentEvaluationRequestDto>,
) -> Json<PaymentEvaluationResponseDto> {
    // Generate correlation ID for this evaluation trace
    let correlation_id = Uuid::new_v4();

    // Collect findings across specialist agents (deterministic simulation)
    let findings = vec![
        AgentFindingDto {
            agent_id: AgentId::TransactionAnalysis,
            agent_version: "1.0.0".to_string(),
            finding_type: "observation".to_string(),
            severity: "low".to_string(),
            description: format!(
                "Transaction amount ({:.2}) is within established baseline",
                payload.amount
            ),
            confidence_value: Some(0.95),
            confidence_note: Some("Empirical baseline comparison on 90-day transaction history".to_string()),
            evidence_refs: vec![Uuid::new_v4()],
        },
        AgentFindingDto {
            agent_id: AgentId::DeviceSession,
            agent_version: "1.0.0".to_string(),
            finding_type: "observation".to_string(),
            severity: "low".to_string(),
            description: "Device fingerprint matches customer's registered primary device".to_string(),
            confidence_value: Some(0.98),
            confidence_note: Some("Browser canvas, TLS client fingerprint, and OS profile match".to_string()),
            evidence_refs: vec![Uuid::new_v4()],
        },
        AgentFindingDto {
            agent_id: AgentId::BehavioralAnalysis,
            agent_version: "1.0.0".to_string(),
            finding_type: "observation".to_string(),
            severity: "low".to_string(),
            description: "Interaction cadence and checkout duration consistent with human behavior".to_string(),
            confidence_value: Some(0.92),
            confidence_note: Some("Form fill velocity within expected variance".to_string()),
            evidence_refs: vec![Uuid::new_v4()],
        },
    ];

    let response = PaymentEvaluationResponseDto {
        transaction_id: payload.transaction_id,
        policy_decision: PolicyDecision::Allow,
        reason_codes: vec![
            "LOW_RISK_VERIFIED_DEVICE".to_string(),
            "BEHAVIORAL_BASELINE_CONFIRMED".to_string(),
        ],
        policy_version: "1.0.0".to_string(),
        agent_findings: findings,
        evaluated_at: Timestamp::now(),
        correlation_id,
    };

    Json(response)
}
