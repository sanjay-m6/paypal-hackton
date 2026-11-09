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

```bash
# Backend
cd backend && cargo build

# Frontend
cd frontend && npm install && npm run dev
```

## Phase Progress

See [docs/PHASE_TRACKER.md](docs/PHASE_TRACKER.md) for current status.

## License

Private — Hackathon Project
