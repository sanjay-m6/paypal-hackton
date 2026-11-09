# Project Skill: Multi-Agent AI Payment & Commerce Intelligence

**Skill ID:** `multi-agent-payment-intelligence`\
**Purpose:** Give AI coding agents and the two-person engineering team a
consistent method to build, secure, test, document, and integrate this
project.

## Mission

Build a secure, auditable multi-agent platform for payment security and
commerce operations. Specialist agents analyze narrowly scoped domains
and return structured evidence. A deterministic Rust policy engine—not
an LLM—makes final `ALLOW`, `DENY`, `STEP_UP`, or `MANUAL_REVIEW`
decisions.

The platform covers transaction/payment risk, identity, merchant,
device/session and behavioral intelligence, payment failures, checkout
optimization, customer intelligence, recommendations, refunds/returns,
disputes, business operations, and controlled learning from verified
outcomes.

## Team

| Role | Person | Primary Ownership |
|------|--------|-------------------|
| **Team Lead** | **Sanjay** | Architecture, Rust backend, policy engine, PayPal adapter, payment security, webhooks, reconciliation, audit, integration schedule, review gates, release readiness, demo |
| **Full-Stack & AI Engineer** | **Kaviya** | React/TypeScript UI, agent contracts & orchestration, specialist agents, evidence aggregation, model evaluation, dashboards, investigation flows, API integration, accessibility |

> Both members contribute to testing, integration, documentation, code
> review, and demo readiness. Every task has one accountable owner; the
> other reviews.

## Technology direction

- **Backend and security-critical paths:** Rust, Tokio, Axum, typed
  domain models.
- **Frontend:** React + TypeScript using the repository's approved UI
  framework and design system.
- **Primary database:** PostgreSQL. Redis, a message broker, and Qdrant
  only where justified by an approved architecture decision.
- **AI:** Provider-neutral adapters. Optional isolated Python service
  for model training/evaluation; never use it as the authority for
  financial execution.
- **Payments:** PayPal Sandbox first; production only after explicit
  approval and current official-documentation verification.
- Preserve the repository's pinned versions and lockfiles. Do not
  silently upgrade or replace frameworks.

## Agent catalogue

| ID | Agent | Scope | Owner |
|----|-------|-------|-------|
| 00 | Master Orchestrator | Plan tasks, select agents, enforce limits, validate and collect results | Kaviya |
| 01 | Transaction Pattern Analyst | Transaction anomaly and pattern analysis | Kaviya |
| 02 | Identity Verification Intelligence | Identity consistency and verification evidence | Kaviya |
| 03 | Merchant Risk Intelligence | Merchant risk and operational signals | Kaviya |
| 04 | Device & Session Intelligence | Permitted device/session security signals | Kaviya |
| 05 | Behavioral Intelligence | Event-sequence and behavior-change analysis | Kaviya |
| 06 | Payment Intelligence | Declines, retries, payment outcomes and method performance | Sanjay |
| 07 | Checkout Optimization | Evidence-based checkout recommendations | Kaviya |
| 08 | Customer Intelligence | Permitted customer lifecycle/service signals | Kaviya |
| 09 | Recommendation Engine | Explainable product/payment recommendations | Kaviya |
| 10 | Refund & Return Intelligence | Refund/return patterns and review recommendations | Sanjay |
| 11 | Dispute Investigation | Evidence timelines and investigation summaries | Sanjay |
| 12 | Business Operations Intelligence | Operational bottlenecks and business trends | Sanjay |

Every agent needs a stable ID, version, capability declaration, narrow
permissions, typed input/output schemas, resource/time limits, evidence
provenance, confidence caveats, limitations, structured errors, and
tests. Agents do not receive unrestricted database, filesystem, network,
or payment credentials.

## Core architecture boundaries

1. The orchestrator coordinates agents; it cannot bypass policy
   enforcement.
2. Agent output is untrusted until schema, provenance, authorization,
   and policy validation.
3. The Rust policy engine owns deterministic financial decisions.
4. A dedicated payment service alone may call money-moving PayPal
   endpoints.
5. AI agents may analyze and recommend, but may not independently
   approve payments, move money, issue refunds, change permissions, or
   perform consequential actions.
6. Use provider adapters and documented APIs/events. Avoid hidden
   cross-module database coupling.
7. Webhook verification, deduplication, replay protection, idempotency,
   state-machine validation, and authoritative PayPal readback are
   mandatory.

## Required workflow for every task

1. Read repository instructions and inspect relevant files before
   editing.
2. Identify phase, owner (Sanjay or Kaviya), reviewer (the other
   member), prerequisites, interfaces, and acceptance criteria.
3. For multi-file or architectural work, produce a short implementation
   plan.
4. Preserve existing conventions and approved dependency versions.
5. Implement a small, complete vertical slice.
6. Add relevant unit, integration, contract, negative, security, and UI
   tests.
7. Run applicable formatters, linters, type checks, tests, and builds.
8. Review the diff for secrets, unsafe defaults, unnecessary
   dependencies, and unrelated changes.
9. Update API/event contracts and documentation when behavior changes.
10. Report changed files, decisions, exact commands run, observed
    results, limitations, and handoff to the other team member.

Never claim a test, build, deployment, integration, or file edit
succeeded unless it actually happened and the result was observed.

## Required response structure for implementation tasks

- Understanding and boundaries
- Repository findings
- Ordered implementation plan
- API/event/data contracts
- Implementation summary
- Verification commands and observed results
- Security review and residual risks
- Changed files, open issues, and next owner (Sanjay or Kaviya)

## Phase discipline

The roadmap has 40 phases across foundation, Rust security
infrastructure, agent intelligence, commerce intelligence,
learning/evaluation, PayPal integration, frontend, and release. Work
only on the requested/approved phase. Do not mark downstream phases
complete based on mocks.

### Phase ownership (2-person split)

| Stage | Phases | Scope | Primary | Reviewer |
|-------|--------|-------|---------|----------|
| A | 1–5 | Requirements, architecture, repo, data model, auth/tenancy | Sanjay | Kaviya |
| B | 6–10 | Rust core, messaging, agent registry, policy engine, audit | Sanjay | Kaviya |
| C | 11–17 | Orchestrator, specialist risk agents, risk aggregation | Kaviya | Sanjay |
| D | 18–24 | Payment, checkout, customer, recs, refunds, disputes, ops | Split* | Cross-review |
| E | 25–30 | Historical features, feedback, model eval, prediction, adversarial | Kaviya | Sanjay |
| F | 31–34 | PayPal Sandbox, gateway, webhooks/reconciliation, financial safety | Sanjay | Kaviya |
| G | 35–37 | Command center, agent workspace, investigation/analytics UI | Kaviya | Sanjay |
| H | 38–40 | E2E integration, security/perf validation, deployment/demo | Both | Both |

*Stage D split: Sanjay owns payment/refund/dispute backend; Kaviya owns
checkout/customer/recommendation agents and UI.

Every phase prompt must define objective, prerequisites, owner,
reviewer, files/modules, contracts, implementation tasks, tests,
security checks, acceptance criteria, demo evidence, and handoff.

## Completion gate

A phase is complete only when acceptance criteria pass, relevant tests
are run, security-sensitive changes are reviewed, documentation is
updated, and the other team member accepts the handoff. Otherwise label
it `BLOCKED`, `IN PROGRESS`, or `PARTIALLY COMPLETE`.

## Explicit non-goals

- Never promise an unhackable system.
- Never let an LLM make final financial authorization decisions.
- Never fabricate PayPal transactions, webhook events, or payment
  statuses.
- Never train on unverified outcomes or use model-generated labels as
  ground truth.
- Never enable autonomous self-modification or production model
  deployment.
- Never expose hidden chain-of-thought; show evidence, concise
  rationale, uncertainty, versions, and policy outcome.
- Never collect or retain sensitive data without a lawful, authorized,
  documented purpose.
