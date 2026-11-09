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
│   │   └── audit/        # Audit trail + observability
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
4. Start PostgreSQL with `docker compose up -d postgres`.

```bash
# Backend
cd backend
cargo run -p payment-api

# Frontend
cd frontend
npm install
npm run dev
```

The API applies database migrations on startup. `/api/v1/auth/login` issues a 15-minute signed bearer token; `/api/v1/agents` and `/api/v1/payments/evaluate` require it. Payment evaluation currently returns `MANUAL_REVIEW` because no policy rules or live agents are configured yet. No payment is executed.

## Phase Progress

See [docs/PHASE_TRACKER.md](docs/PHASE_TRACKER.md) for current status.

## License

Private — Hackathon Project
