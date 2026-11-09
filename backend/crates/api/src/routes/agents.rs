use axum::Json;
use payment_core::dto::AgentDescriptorDto;

use crate::middleware::auth::AuthenticatedUser;

/// GET /api/v1/agents
///
/// Returns metadata descriptors for all 13 specialist agents in the platform.
pub async fn list_agents(_user: AuthenticatedUser) -> Json<Vec<AgentDescriptorDto>> {
    let descriptors = vec![
        AgentDescriptorDto {
            id: 0,
            name: "Master Orchestrator".to_string(),
            version: "1.0.0".to_string(),
            owner: "Kaviya".to_string(),
            scope: "Task planning, agent selection, timeout enforcement, evidence collection"
                .to_string(),
            capabilities: vec!["Orchestration".to_string(), "ResultAggregation".to_string()],
            timeout_ms: 1000,
        },
        AgentDescriptorDto {
            id: 1,
            name: "Transaction Pattern Analyst".to_string(),
            version: "1.0.0".to_string(),
            owner: "Kaviya".to_string(),
            scope: "Transaction volume, velocity, amount anomalies, geographic drift".to_string(),
            capabilities: vec!["TransactionAnalysis".to_string()],
            timeout_ms: 500,
        },
        AgentDescriptorDto {
            id: 2,
            name: "Identity Verification Intelligence".to_string(),
            version: "1.0.0".to_string(),
            owner: "Kaviya".to_string(),
            scope: "Identity consistency, KYC signals, email/phone domain risk".to_string(),
            capabilities: vec!["IdentityVerification".to_string()],
            timeout_ms: 500,
        },
        AgentDescriptorDto {
            id: 3,
            name: "Merchant Risk Intelligence".to_string(),
            version: "1.0.0".to_string(),
            owner: "Kaviya".to_string(),
            scope: "Merchant chargeback ratios, volume spikes, reputation indicators".to_string(),
            capabilities: vec!["MerchantRisk".to_string()],
            timeout_ms: 500,
        },
        AgentDescriptorDto {
            id: 4,
            name: "Device & Session Intelligence".to_string(),
            version: "1.0.0".to_string(),
            owner: "Kaviya".to_string(),
            scope: "Browser fingerprints, proxy/VPN detection, session hijacking signals"
                .to_string(),
            capabilities: vec!["DeviceSession".to_string()],
            timeout_ms: 500,
        },
        AgentDescriptorDto {
            id: 5,
            name: "Behavioral Intelligence".to_string(),
            version: "1.0.0".to_string(),
            owner: "Kaviya".to_string(),
            scope: "Event sequences, checkout cadence, behavioral drift anomalies".to_string(),
            capabilities: vec!["BehavioralAnalysis".to_string()],
            timeout_ms: 500,
        },
        AgentDescriptorDto {
            id: 6,
            name: "Payment Intelligence".to_string(),
            version: "1.0.0".to_string(),
            owner: "Sanjay".to_string(),
            scope: "Declines, retries, payment method performance, gateway error codes".to_string(),
            capabilities: vec!["PaymentAnalysis".to_string()],
            timeout_ms: 500,
        },
        AgentDescriptorDto {
            id: 7,
            name: "Checkout Optimization".to_string(),
            version: "1.0.0".to_string(),
            owner: "Kaviya".to_string(),
            scope: "Evidence-based checkout flow friction reduction and recommendations"
                .to_string(),
            capabilities: vec!["CheckoutOptimization".to_string()],
            timeout_ms: 500,
        },
        AgentDescriptorDto {
            id: 8,
            name: "Customer Intelligence".to_string(),
            version: "1.0.0".to_string(),
            owner: "Kaviya".to_string(),
            scope: "Permitted customer lifecycle and service telemetry".to_string(),
            capabilities: vec!["CustomerIntelligence".to_string()],
            timeout_ms: 500,
        },
        AgentDescriptorDto {
            id: 9,
            name: "Recommendation Engine".to_string(),
            version: "1.0.0".to_string(),
            owner: "Kaviya".to_string(),
            scope: "Explainable product and payment method recommendations".to_string(),
            capabilities: vec!["Recommendations".to_string()],
            timeout_ms: 500,
        },
        AgentDescriptorDto {
            id: 10,
            name: "Refund & Return Intelligence".to_string(),
            version: "1.0.0".to_string(),
            owner: "Sanjay".to_string(),
            scope: "Refund and return patterns, abuse detection, review recommendations"
                .to_string(),
            capabilities: vec!["RefundAnalysis".to_string()],
            timeout_ms: 500,
        },
        AgentDescriptorDto {
            id: 11,
            name: "Dispute Investigation".to_string(),
            version: "1.0.0".to_string(),
            owner: "Sanjay".to_string(),
            scope: "Evidence compilation timelines and chargeback defense packets".to_string(),
            capabilities: vec!["DisputeInvestigation".to_string()],
            timeout_ms: 500,
        },
        AgentDescriptorDto {
            id: 12,
            name: "Business Operations Intelligence".to_string(),
            version: "1.0.0".to_string(),
            owner: "Sanjay".to_string(),
            scope: "Operational bottlenecks, SLA breaches, business trend monitoring".to_string(),
            capabilities: vec!["BusinessOps".to_string()],
            timeout_ms: 500,
        },
    ];

    Json(descriptors)
}
