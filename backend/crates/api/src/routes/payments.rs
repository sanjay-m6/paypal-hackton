use axum::{
    Json,
    extract::{State, rejection::JsonRejection},
};
use payment_agents::event_bus::{AgentEvent, EventType};
use payment_core::{
    PlatformError,
    dto::{PaymentEvaluationRequestDto, PaymentEvaluationResponseDto},
    types::{AgentId, TraceContext},
};
use rust_decimal::Decimal;
use serde_json::json;
use uuid::Uuid;

use crate::{
    middleware::{auth::AuthenticatedUser, errors::ApiError},
    state::AppState,
};

pub async fn evaluate_payment(
    user: AuthenticatedUser,
    State(state): State<AppState>,
    payload: Result<Json<PaymentEvaluationRequestDto>, JsonRejection>,
) -> Result<Json<PaymentEvaluationResponseDto>, ApiError> {
    let Json(payload) = payload.map_err(ApiError::from)?;
    if !user.can_evaluate_payments() {
        return Err(ApiError(PlatformError::Forbidden(
            "This role cannot evaluate payments".into(),
        )));
    }
    if payload.tenant_id != user.tenant_id.0 {
        return Err(ApiError(PlatformError::TenantIsolation(
            "Request tenant does not match the authenticated tenant".into(),
        )));
    }
    if payload.amount <= Decimal::ZERO {
        return Err(ApiError(PlatformError::Validation(
            "Payment amount must be greater than zero".into(),
        )));
    }
    if payload.customer.email.len() > 254
        || payload.customer.customer_id.len() > 128
        || payload.customer.ip_address.len() > 64
    {
        return Err(ApiError(PlatformError::Validation(
            "Customer fields exceed their maximum length".into(),
        )));
    }

    let trace = TraceContext::new(user.tenant_id, user.user_id);
    let correlation_id = trace.correlation_id.0;
    let task_id = Uuid::new_v4();

    // 1. Emit TransactionSubmitted event
    let tx_submitted_event = AgentEvent::create(
        EventType::TransactionSubmitted,
        user.tenant_id.0,
        task_id,
        correlation_id,
        AgentId::MasterOrchestrator,
        None,
        json!({
            "transaction_id": payload.transaction_id,
            "amount": payload.amount.to_string(),
            "currency": format!("{:?}", payload.currency),
            "payment_method": format!("{:?}", payload.payment_method),
        }),
        60,
    );
    let _ = state.event_bus.publish(tx_submitted_event).await;

    // 2. Emit EvaluationStarted event
    let eval_started_event = AgentEvent::create(
        EventType::EvaluationStarted,
        user.tenant_id.0,
        task_id,
        correlation_id,
        AgentId::MasterOrchestrator,
        None,
        json!({
            "agent_count": state.agent_registry.list().len(),
            "customer_id": payload.customer.customer_id,
        }),
        60,
    );
    let _ = state.event_bus.publish(eval_started_event).await;

    // 3. Evaluate deterministic policy
    let outcome = state.policy_engine.evaluate(&[], &trace);

    // 4. Emit PolicyDecisionEmitted event
    let decision_event = AgentEvent::create(
        EventType::PolicyDecisionEmitted,
        user.tenant_id.0,
        task_id,
        correlation_id,
        AgentId::MasterOrchestrator,
        None,
        json!({
            "decision": format!("{:?}", outcome.decision),
            "reason_codes": outcome.reason_codes,
            "policy_version": outcome.policy_version,
        }),
        60,
    );
    let _ = state.event_bus.publish(decision_event).await;

    let response = PaymentEvaluationResponseDto {
        transaction_id: payload.transaction_id,
        policy_decision: outcome.decision,
        reason_codes: outcome.reason_codes,
        policy_version: outcome.policy_version,
        agent_findings: Vec::new(),
        evaluated_at: outcome.evaluated_at,
        correlation_id: outcome.trace.0,
    };

    Ok(Json(response))
}
