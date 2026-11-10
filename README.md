# Multi-Agent AI Payment Security Platform

> A secure, auditable multi-agent platform for payment security and commerce intelligence.

## Team

| Role | Person |
|------|--------|
| **Team Lead** | Sanjay |
| **Full-Stack & AI Engineer** | Kaviya |

## Architecture

- **Backend:** Rust (Axum + Tokio) — policy engine, payment orchestration, audit
- **Frontend:** React + TypeScript — operator dashboard, agent workspace
- **Database:** PostgreSQL
- **Payments:** PayPal Sandbox
- **AI:** Provider-neutral agent adapters (13 specialist agents)

## Key Principle

> AI agents analyze and recommend. A **deterministic Rust policy engine** makes final
> `ALLOW` / `DENY` / `STEP_UP` / `MANUAL_REVIEW` decisions. No LLM ever approves a payment.

## Project Structure

```
hackton/
├── backend/              # Rust workspace (Axum API, policy engine, agents)
│   ├── crates/
│   │   ├── api/          # HTTP API server
│   │   ├── core/         # Domain models, shared types
│   │   ├── policy/       # Deterministic policy engine
│   │   ├── agents/       # Agent framework + specialist agents
│   │   ├── paypal/       # PayPal adapter
│   │   ├── db/           # Database layer (SQLx + migrations)
│   │   ├── audit/        # Audit trail + observability
│   │   └── messaging/    # Signed agent contracts + JetStream bus
│   └── Cargo.toml        # Workspace root
├── frontend/             # React + TypeScript app
├── docs/                 # Architecture, API specs, phase docs
├── migrations/           # PostgreSQL migrations
├── scripts/              # Dev scripts, seed data
├── SKILL.md
├── PROJECT_RULES.md
└── README.md
```

## Getting Started

1. Copy `.env.example` to `.env`.
2. Set `JWT_SECRET` to a randomly generated secret with at least 32 bytes. Never reuse the example or commit `.env`.
3. Optionally set `BOOTSTRAP_ADMIN_EMAIL` and `BOOTSTRAP_ADMIN_PASSWORD` (at least 12 characters) to create the first local development operator. This only inserts the account if it does not already exist. Remove these values after provisioning.
4. Start PostgreSQL with `docker compose up -d postgres`. Start NATS too when testing Phase 7 messaging.

```

For Phase 7, run `docker compose up -d postgres nats`, set `MESSAGING_ENABLED=true`, and configure `MESSAGE_SIGNING_KEYS_JSON` with at least one active URL-safe Base64 key containing 32 or more random bytes. The local Compose broker is bound to loopback and uses plaintext only for development. Remote NATS deployments require TLS and credentials.bash
# Backend
cd backend
cargo run -p payment-api

# Frontend
cd frontend
npm install
npm run dev
```

The API applies database migrations on startup. `/api/v1/auth/login` issues a 15-minute signed bearer token; `/api/v1/agents`, `/api/v1/payments/evaluate`, and `/api/v1/messaging/status` require it. Messaging remains disabled unless explicitly enabled. Payment evaluation currently returns `MANUAL_REVIEW` because no policy rules or live agents are configured yet. No payment is executed.

## Phase Progress

See [docs/PHASE_TRACKER.md](docs/PHASE_TRACKER.md) for current status.

## License

Private — Hackathon Project
