# Phase 07 — Agent Communication & Event Bus

## Status and scope

Phase 07 establishes the message contract and transport foundation. It does not
implement specialist agents, agent identity discovery, dynamic capability
registration, policy decisions, or payment execution. The API's messaging status
reports `workers_active: false`; no production agent workers are currently
registered.

## Message contract and trust

Agent messages use a strict, versioned JSON envelope with message/task/tenant
identifiers, sender and recipient principals, event and payload types, issue and
expiry timestamps, correlation and trace identifiers, key ID, and signature.
Unknown envelope fields are rejected. The payload must match its declared event
type and context.

HMAC-SHA256 signs the serialized envelope with its signature field cleared.
Keys are selected by configured key ID, must contain at least 32 bytes, and are
never serialized or exposed in `Debug` output. `MESSAGE_SIGNING_KEYS_JSON`
contains a JSON array with `key_id`, `principal`, and `secret_base64`; optional
`revoked` keys are rejected at verification. Principals are `API`,
`ORCHESTRATOR`, or `AGENT:<snake_case_agent_id>`. Secret material is URL-safe
Base64 without padding. Never place production keys in `.env.example`, source
control, or logs.

Authorization is intentionally a static Phase 07 allowlist:

- Orchestrator → agent: task request.
- Agent → orchestrator: task completed or failed.

Although `API` keys can be loaded for future use, API-originated message routes
are not authorized by this Phase 07 allowlist.

The configured key, not an untrusted message-body identity, is the source of
sender identity. Validators enforce schema version, 5-minute maximum lifetime,
30-second clock skew, payload size, route, and task/tenant context.

## Transport and delivery

NATS JetStream persists messages in the `PAYMENT_AGENT_EVENTS` file-backed
stream. Subjects are `agent.task.<agent_id>` and `agent.result.orchestrator`.
Dead-letter metadata is published to the separate, file-backed
`PAYMENT_AGENT_DEAD_LETTERS` stream on `agent.dead-letter`, with 90-day
retention. Publication uses the stable message ID as the JetStream
deduplication ID. The stream retains messages for the configured TTL (default
300 seconds) and uses a 120-second duplicate window.

Delivery is **at least once**, not exactly once. There is no global ordering
guarantee. Consumers use durable pull subscriptions and explicit acknowledgments.
Consumers claim work through the tenant-scoped PostgreSQL inbox before calling
the handler. A processed duplicate is acknowledged without invoking the handler;
an active claim is NAKed for retry. The handler's signed result is committed to
the outbox in the same transaction that marks the inbox row processed, and the
input is acknowledged only after that transaction succeeds. A handler timeout
or retryable error releases its claim before NAK. Leases expire to recover work
after a worker crash. External side effects inside handlers still require
idempotency because this transport provides at-least-once delivery.

Retryable failures use exponential NAK delay (default 250 ms base, capped at
30 seconds) and a maximum of five deliveries. Handler execution is capped at
60 seconds by default (maximum configurable value: 90 seconds); permanent
failures and exhausted retries go to the dead-letter subject and database using
redacted metadata.

The PostgreSQL inbox and outbox use `(tenant_id, message_id)` uniqueness. The
outbox publisher validates signed envelopes before publication, polls pending
rows, and marks them published only after JetStream acknowledges the publish.
Outbox publish failures back off exponentially, capped at five minutes, and are
recorded as redacted dead-letter metadata after ten attempts. A crash between
broker acknowledgment and the database update can result in a duplicate; inbox
deduplication is required.

Inbox/outbox envelopes are stored as JSONB and may contain sensitive transaction
context. Protect database access and backups, apply the platform's retention
and data-governance requirements, and avoid logging message payloads. Expired
inbox/outbox payloads are purged after seven days; database and JetStream
dead-letter metadata is retained for 90 days. Dead-letter records contain only
IDs, subject, attempt count, and a safe reason code—not the message body.

## Local operation

1. Copy `.env.example` to `.env` and set the existing database and JWT settings.
2. Start services with `docker compose up -d postgres nats`.
3. Generate a random secret with at least 32 bytes, encode it as URL-safe Base64
   without padding, and configure `MESSAGE_SIGNING_KEYS_JSON`. Do not reuse the
   illustrative shape as a real key.
4. Set `MESSAGING_ENABLED=true`; local NATS uses `NATS_URL=nats://127.0.0.1:4222`
   and `NATS_REQUIRE_TLS=false`. Plaintext is accepted only for loopback.
5. Start the API. Startup fails explicitly if messaging is enabled but the key
   configuration or broker is unavailable.
6. Sign in and query `GET /api/v1/messaging/status` for broker/stream readiness.

For remote deployments, require TLS and configure NATS credentials separately
from the URL. The status route is authenticated. The default local setup leaves
messaging disabled.

## Verification and limitations

Unit tests cover signing, tampering, spoofing, revoked/weak keys, strict schema,
expiry, route/context validation, subject naming, and retry delay. The database
repository has validation tests; live PostgreSQL/NATS integration and end-to-end
worker processing still require service-backed testing and reviewer approval.
No fake production agents are included.
