# Phase 3: Repository, Workspace & Developer Experience

**Document ID:** `DEV-EXP-003`  
**Lead Owner:** Sanjay (Team Lead & Backend)  
**Reviewer:** Kaviya (Full-Stack & AI)  
**Status:** 🚀 IN PROGRESS  
**Date:** 2026-11-09  

---

## 1. Developer Environment & Prerequisites

To build, test, and contribute to the platform, engineers need:

| Dependency | Minimum Version | Verified Local Version | Purpose |
|---|---|---|---|
| **Rust Toolchain** | `1.85+` (2024 edition) | `rustc 1.99.1` | Backend API, Policy Engine, Crates |
| **Node.js** | `20+` LTS | `v24.13.1` | React 19 Frontend & Tooling |
| **npm** | `10+` | `11.8.0` | Package manager |
| **PostgreSQL** | `16+` | Docker container / local | ACID Relational Store |
| **Docker & Compose** | `24+` | Optional / Local | Database & integration containers |

---

## 2. Workspace Scripts & Automation

All developer tasks are unified via standard PowerShell scripts under `scripts/`:

| Script | Purpose | Command |
|---|---|---|
| `scripts/setup.ps1` | Full one-command environment setup (dependencies, `.env`, tools) | `powershell -File scripts/setup.ps1` |
| `scripts/validate.ps1` | Comprehensive quality gate: fmt, clippy, cargo test, tsc, vite build | `powershell -File scripts/validate.ps1` |
| `scripts/seed.ps1` | Injects synthetic test transactions and tenant profiles | `powershell -File scripts/seed.ps1` |

---

## 3. CI/CD Workflows (`.github/workflows/`)

1. **`ci.yml` (Continuous Integration):**
   - Matrix testing on Ubuntu and Windows.
   - Runs `cargo check`, `cargo fmt --check`, `cargo clippy -- -D warnings`.
   - Runs `cargo test --workspace` with 100% pass requirement.
   - Runs frontend `npx tsc --noEmit` and `npm run build`.
2. **`security-audit.yml` (Security & Dependency Audit):**
   - Runs `cargo audit` to detect known vulnerabilities in Rust crate dependencies.
   - Automated secret-scanning regex check preventing accidental token commits.

---

## 4. Local Containerization (`docker-compose.yml`)

A lightweight PostgreSQL 16 container pre-configured with:
- Database: `payment_security`
- User: `postgres`
- Port: `5432`
- Health check polling `pg_isready`
- Persistent volume `pgdata` for local state

