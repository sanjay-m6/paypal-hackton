use axum::{Json, extract::State};
use payment_messaging::MessagingStatus;
use serde::Serialize;

use crate::{middleware::auth::AuthenticatedUser, state::AppState};

#[derive(Debug, Serialize)]
pub struct MessagingStatusResponse {
    pub enabled: bool,
    pub connected: bool,
    pub stream_ready: bool,
    pub stream_name: Option<String>,
    pub dead_letter_stream_ready: bool,
    pub dead_letter_stream_name: Option<String>,
    pub delivery_semantics: &'static str,
    pub workers_active: bool,
}

pub async fn messaging_status(
    _user: AuthenticatedUser,
    State(state): State<AppState>,
) -> Json<MessagingStatusResponse> {
    let response = match state.messaging {
        Some(bus) => {
            let MessagingStatus {
                connected,
                stream_ready,
                stream_name,
                dead_letter_stream_ready,
                dead_letter_stream_name,
            } = bus.status().await;
            MessagingStatusResponse {
                enabled: true,
                connected,
                stream_ready,
                stream_name: Some(stream_name),
                dead_letter_stream_ready,
                dead_letter_stream_name: Some(dead_letter_stream_name),
                delivery_semantics: "at_least_once",
                workers_active: false,
            }
        }
        None => MessagingStatusResponse {
            enabled: false,
            connected: false,
            stream_ready: false,
            stream_name: None,
            dead_letter_stream_ready: false,
            dead_letter_stream_name: None,
            delivery_semantics: "at_least_once",
            workers_active: false,
        },
    };
    Json(response)
}
