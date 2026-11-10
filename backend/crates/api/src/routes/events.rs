use axum::{
    extract::{Query, State},
    Json,
};
use payment_agents::event_bus::{AgentEvent, EventBusMetrics};
use payment_core::PlatformError;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    middleware::{auth::AuthenticatedUser, errors::ApiError},
    state::AppState,
};

#[derive(Debug, Deserialize)]
pub struct EventQueryParams {
    pub limit: Option<usize>,
    pub correlation_id: Option<Uuid>,
}

#[derive(Debug, Serialize)]
pub struct EventsResponse {
    pub events: Vec<AgentEvent>,
    pub count: usize,
    pub metrics: EventBusMetrics,
}

/// Retrieve recent agent events from the asynchronous event bus.
/// Scoped to the authenticated operator's tenant boundary.
pub async fn get_recent_events(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Query(params): Query<EventQueryParams>,
) -> Result<Json<EventsResponse>, ApiError> {
    if !matches!(
        user.role.as_str(),
        "admin" | "operator" | "risk_analyst" | "read_only"
    ) {
        return Err(ApiError(PlatformError::Forbidden(
            "Access to agent event bus denied for this role".into(),
        )));
    }

    let limit = params.limit.unwrap_or(50).min(100);

    let events = if let Some(corr_id) = params.correlation_id {
        state.event_bus.get_recent_by_correlation(corr_id).await
    } else {
        // Enforce tenant boundary: filter by tenant_id Uuid
        state.event_bus.get_recent_by_tenant(user.tenant_id.0).await
    };

    let events: Vec<AgentEvent> = events.into_iter().take(limit).collect();
    let count = events.len();
    let metrics = state.event_bus.get_metrics().await;

    Ok(Json(EventsResponse {
        events,
        count,
        metrics,
    }))
}

/// Retrieve event bus telemetry metrics.
pub async fn get_event_metrics(
    State(state): State<AppState>,
    user: AuthenticatedUser,
) -> Result<Json<EventBusMetrics>, ApiError> {
    if !matches!(
        user.role.as_str(),
        "admin" | "operator" | "risk_analyst" | "read_only"
    ) {
        return Err(ApiError(PlatformError::Forbidden(
            "Access to agent event metrics denied for this role".into(),
        )));
    }
    let metrics = state.event_bus.get_metrics().await;
    Ok(Json(metrics))
}
