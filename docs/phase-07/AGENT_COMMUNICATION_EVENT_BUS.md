# Phase 7: Agent Communication & Event Bus

**Owner:** Kaviya (Full-Stack & AI Engineer)  
**Reviewer:** Sanjay (Team Lead & Backend)  
**Status:** ✅ COMPLETE  
**Date:** 2026-11-11  

---

## 1. Overview & Architecture

Phase 7 establishes the asynchronous, decoupled messaging backbone connecting the **Master Orchestrator** with the **13 Specialist Intelligence Agents**. All inter-agent and orchestrator events traverse a bounded in-memory event bus carrying cryptographically hashed, typed message envelopes.

In adherence with **Project Rule 2.6 and Rule 2.7**, every message crossing agent boundaries is verifiable, signed with a SHA-256 payload digest, stamped with correlation IDs for end-to-end distributed tracing, and validated against expiration time-to-live (TTL) limits.

```
                           +-------------------------------------+
                           |         Master Orchestrator         |
                           +-------------------------------------+
                                       |              ^
                 publish(Dispatched)   |              |  publish(Finding)
                                       v              |
+----------------------------------------------------------------------------------------+
|                          Asynchronous Tokio Broadcast Event Bus                         |
|  - Bounded Capacity: 1,024 events                                                      |
|  - Cryptographic Verification: SHA-256 Payload Hash                                    |
|  - In-Memory Circular Buffer: Recent 100 events                                        |
+----------------------------------------------------------------------------------------+
         |                       |                       |                      |
         v                       v                       v                      v
+-----------------+     +-----------------+     +-----------------+    +-----------------+
|   Transaction   |     |    Identity     |     |  Device/Session |    |   PolicyEngine  |
| Analysis Agent  |     |   Verification  |     |  Intelligence   |    |    (Decision)   |
+-----------------+     +-----------------+     +-----------------+    +-----------------+
```

---

## 2. Key Deliverables & Components

### 2.1 Event Bus Architecture (`backend/crates/agents/src/event_bus.rs`)
- **Bounded Channel:** Configurable `tokio::sync::broadcast` channel with capacity of 1,024 events to prevent memory exhaustion under burst traffic.
- **Envelope Protocol (`AgentMessageEnvelope`):**
  - `message_id`: Unique UUID v4 for each message.
  - `task_id`: Identifies the parent evaluation task.
  - `correlation_id`: Distributed trace identifier across boundaries.
  - `sender`: `AgentId` of emitting component.
  - `recipient`: Optional destination `AgentId` or broadcast.
  - `schema_version`: Semantic schema version (`1.0.0`).
  - `timestamp`: UTC emission time.
  - `expires_at`: Strict expiration boundary (TTL).
  - `payload_hash_sha256`: Hex-encoded SHA-256 digest of payload bytes.

### 2.2 Security Invariants & Verification (`verify_envelope_integrity`)
1. **Payload Non-Tampering (Rule 2.6):** The event bus computes `SHA256(payload)` and asserts equality against `payload_hash_sha256`. Mismatched payloads are rejected with `PlatformError::SecurityViolation`.
2. **Expiration Enforcement (Rule 2.7):** Messages with `expires_at < Utc::now()` are dropped immediately.
3. **Trace Context Invariant:** Rejects any message missing non-nil `task_id` or `correlation_id`.

### 2.3 Orchestrator Lifecycle Hooks (`backend/crates/agents/src/orchestrator.rs`)
- Orchestrator holds an `Arc<EventBus>`.
- Emits `EventType::SpecialistDispatched` prior to invoking an agent with strict Tokio timeouts.
- Emits `EventType::FindingPublished` when an agent successfully returns findings.
- Emits `EventType::SecurityAlert` when an agent times out or errors.

### 2.4 API Endpoints & State (`backend/crates/api/src/routes/events.rs`)
- `GET /api/v1/events/recent`: Scoped to authenticated tenant, returns the in-memory circular ring buffer of recent events.
- `GET /api/v1/events/metrics`: Returns total published, delivered, and tamper failure metrics.
- `POST /api/v1/payments/evaluate`: Automatically emits `TransactionSubmitted`, `EvaluationStarted`, and `PolicyDecisionEmitted` onto the event bus.

### 2.5 Neumorphic Frontend Stream Explorer (`frontend/src/app/Phase2Architecture.tsx`)
- Dedicated **"Event Bus & Agent Feed"** tab in the dashboard.
- Live scrolling event feed displaying event category, sender/recipient routing, correlation IDs, and SHA-256 digest badges.
- **"Simulate Multi-Agent Event Fanout"** interactive trigger generating real-time event sequences.

---

## 3. Verification & Test Coverage

All 4 unit tests in `payment-agents` passed:
- `test_event_bus_publish_and_subscribe`: Multi-subscriber broadcast fanout.
- `test_event_bus_tamper_detection`: Verifies tamper rejection on modified payload.
- `test_event_bus_expired_event_rejection`: Drops expired messages.
- `test_event_bus_recent_history_retention`: Ring buffer capacity and correlation filtering.

Across the workspace: **18 tests passed, 0 failures**.
TypeScript compilation: **0 errors**.
Production build: **Success in 1.68s**.
