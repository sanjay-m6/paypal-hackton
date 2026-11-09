# Phase 2 Threat Model & Security Architecture (STRIDE)

**Document ID:** `SEC-THREAT-002`  
**Security Lead:** Sanjay (Team Lead & Backend)  
**AI & Intelligence Lead:** Kaviya (Full-Stack & AI)  
**Status:** ✅ APPROVED  
**Date:** 2026-11-09  

---

## 1. Scope & Trust Boundaries

The system processes financial risk assessments and coordinates PayPal transactions. Six concentric trust boundaries are enforced:

```
[Untrusted: Browser / Public Internet]
        │ TLS 1.3 + Signed JWT
        ▼
[Boundary 1: Axum Gateway (Auth & Rate Limiting)]
        │ Typed DTOs + Strict Schema Validation
        ▼
[Boundary 2: Orchestration Engine]
        │ Capability Allowlist + Resource Limits (500ms max)
        ▼
[Boundary 3: Specialist Agent Intelligence (00 - 12)]
        │ Pure Rust In-Memory (Zero LLM Bypass)
        ▼
[Boundary 4: Deterministic Policy Engine (ALLOW / DENY / STEP_UP / MANUAL_REVIEW)]
        │ Idempotency Key + Authoritative Webhooks
        ▼
[Boundary 5: Dedicated Payment Service (PayPal Sandbox)]
        │ Row-Level Tenant Scoping + Immutable Audit
        ▼
[Boundary 6: PostgreSQL 16 & Audit Ledger]
```

---

## 2. STRIDE Threat Analysis Matrix

| Threat Category | Specific Attack Vector | Target Boundary | Severity | Engineered Mitigation |
|---|---|---|---|---|
| **Spoofing** | Forged Tenant ID in request body | Boundary 1 (Gateway) | **Critical** | Tenant ID is extracted strictly from the cryptographically verified JWT claims, never from client-provided headers or JSON bodies. |
| **Spoofing** | Spoofed Agent Finding | Boundary 2 (Orchestrator) | **High** | All agent findings require an `AgentMessageEnvelope` containing sender `AgentId`, SHA-256 payload hash, task ID, and timestamp before acceptance. |
| **Tampering** | Modifying Policy Outcome in Transit | Boundary 4 (Policy Engine) | **Critical** | Policy engine is pure Rust in-memory code. LLMs have zero write access to policy rules or decisions. Decisions are committed to the DB in an atomic transaction. |
| **Tampering** | Replayed Payment Webhook | Boundary 5 (PayPal Adapter) | **High** | Webhooks verify PayPal signature headers (`PAYPAL-AUTH-ALGO`, `PAYPAL-CERT-URL`, `PAYPAL-TRANSMISSION-SIG`) and deduplicate using an atomic `idempotency_keys` table. |
| **Repudiation** | Operator denies reviewing flagged transaction | Boundary 6 (Audit Trail) | **Medium** | Immutable append-only `audit_events` table logs user ID, tenant ID, correlation ID, exact timestamp, and decision diff. |
| **Information Disclosure** | Card PAN / CVV leakage in logs | Boundary 1 & 6 | **Critical** | Platform never collects, accepts, or logs raw PAN or CVV. Adapter interacts only with PayPal order tokens and masked tokens (`*** 9098`). |
| **Information Disclosure** | LLM Chain-of-Thought leakage | Boundary 3 (Agents) | **Medium** | Public contracts expose only structured `AgentFindingDto` (evidence refs, observation, severity). Raw prompts and internal reasoning remain unexposed. |
| **Denial of Service** | Slow LLM Agent hanging financial evaluation | Boundary 2 & 3 | **High** | Hard `tokio::time::timeout` (500ms default) enforced per agent. If an agent times out, the orchestrator records `AgentStatus::TimedOut` and degrades gracefully. |
| **Elevation of Privilege** | Agent executing money-moving refund or capture | Boundary 4 & 5 | **Critical** | Only `payment-paypal` service has credentials for PayPal capture. Agents possess zero network or payment credentials. Financial actions require prior `ALLOW` policy decision. |

---

## 3. Data Classification & Handling Invariants

1. **Cardholder Data (CHD):** **NEVER STORED.** No primary account numbers (PAN), magnetic stripe data, or CVV/CVC codes enter the platform.
2. **Authentication Secrets:** PayPal client secrets and API keys are loaded via environment variables (`dotenvy`) server-side only. Never exposed to browser or agents.
3. **Personally Identifiable Information (PII):** Customer emails and IP addresses are masked in client-facing telemetry and deleted according to tenant data retention limits.
4. **Policy Rules:** Stored with explicit version tags in PostgreSQL and evaluated deterministically with zero probabilistic variance.

