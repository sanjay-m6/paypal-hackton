use axum::{
    Json,
    extract::{State, rejection::JsonRejection},
};
use payment_core::{
    PlatformError,
    dto::{PaymentEvaluationRequestDto, PaymentEvaluationResponseDto},
    types::TraceContext,
};
use rust_decimal::Decimal;

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
    let outcome = state.policy_engine.evaluate(&[], &trace);
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
