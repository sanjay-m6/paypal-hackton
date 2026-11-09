# Mandatory Project Rules

These rules apply to both team members (Sanjay and Kaviya), AI coding
agents, generated code, reviews, phase prompts, tests, and releases.

> **Team:** Sanjay (Team Lead) · Kaviya (Full-Stack & AI Engineer)
>
> Every task has one owner and one reviewer. With two members, the
> reviewer is always the other person.
>
> 🛑 **MANDATORY USER APPROVAL GATE (STRICT RULE):**
> Upon completing any phase:
> 1. Launch/open the application so the user can visually and interactively inspect it.
> 2. Present a comprehensive list of all implemented features, modules, and components.
> 3. Ask the user to check the UI and verify that all components are working properly.
> 4. **NEVER start building the next phase without explicit user approval.** Always wait for the user's review and go-ahead.


---

## 1. Architecture and authority

1. Rust owns core backend control, authorization, policy enforcement,
   payment orchestration, webhook validation, reconciliation, and
   audit-critical paths unless an approved architecture decision says
   otherwise. **Owner: Sanjay.**
2. React + TypeScript owns the operator UI. The browser is never an
   authority for access control or financial decisions. **Owner: Kaviya.**
3. AI is advisory. No LLM or probabilistic model may be the final
   authority for payment approval, capture, refund, or other financial
   execution.
4. The deterministic Rust policy engine returns explicit outcomes:
   `ALLOW`, `DENY`, `STEP_UP`, or `MANUAL_REVIEW`, with reason codes
   and policy version.
5. Only a dedicated, authorized payment service may call money-moving
   PayPal endpoints. Agents and the orchestrator never receive payment
   credentials.
6. Keep external providers behind adapters. Do not leak
   PayPal/provider-specific types into core domain logic.
7. Modules interact through documented APIs, typed messages, or
   explicit interfaces—not private-table access.

## 2. Security

1. Never describe the platform as unhackable. Use defense-in-depth and
   assume components can fail or be compromised.
2. Enforce least privilege for users, tenants, services, agents,
   credentials, database roles, and deployment identities.
3. Enforce tenant isolation server-side and at data-access boundaries.
   Never trust a browser-supplied tenant ID.
4. Never commit secrets, API keys, tokens, private keys, webhook
   secrets, real payment data, or customer data.
5. Use TLS and authenticated, integrity-protected service
   communication.
6. Authenticate/sign messages crossing trust boundaries. Include
   message ID, task ID, sender, schema version, timestamp, expiry, and
   trace ID.
7. Reject expired, replayed, malformed, oversized, or unauthorized
   messages.
8. Validate all external inputs at boundaries using strict schemas and
   bounded values.
9. Treat prompts, retrieved documents, merchant/customer text, tool
   outputs, and agent results as untrusted. Defend against prompt
   injection and tool misuse.
10. Tool access is capability-allowlisted. No unrestricted shell,
    filesystem, browser, network, or database access for agents.
11. Isolate untrusted parsing/model workloads and enforce CPU, memory,
    time, and output limits.
12. Mitigate brute force, credential stuffing, enumeration, CSRF, XSS,
    SSRF, injection, unsafe deserialization, and denial-of-service.
13. Use secure session handling, short-lived credentials,
    rotation/revocation where applicable.
14. Audit authentication, authorization failures, policy decisions,
    agent actions, configuration changes, and payment lifecycle
    transitions.
15. Never log secrets, tokens, full card data, or unnecessary personal
    data.
16. Apply data minimization, purpose limitation, retention limits,
    deletion workflows, and access logging.
17. Record security findings with severity, reproduction, affected
    component, mitigation, and verification evidence.

## 3. PayPal and financial safety

1. Use PayPal Sandbox for development and demonstrations.
2. Verify current PayPal APIs, SDKs, fields, and webhook procedures
   against official documentation before implementation.
3. Validate amount, currency, merchant/account context, order state,
   authorization, and permitted operation before execution.
4. Use idempotency keys/request identifiers where supported and persist
   idempotency state.
5. Never infer success from a redirect, browser event, model response,
   or local status.
6. Confirm final state through authoritative PayPal responses and
   verified webhooks/reconciliation.
7. Verify webhook signatures using PayPal's documented process before
   processing.
8. Deduplicate events; handle repeated, delayed, and out-of-order
   delivery.
9. Enforce a transaction state machine; reject invalid transitions and
   reconcile discrepancies.
10. Separate permissions for authorization, capture, refund, void,
    dispute, and reconciliation.
11. Require human approval for high-impact or irreversible actions
    unless an explicitly approved bounded policy allows automation.
12. Never use live credentials or real funds in local development or
    public demos.
13. Label all synthetic and Sandbox data clearly.

## 4. Agent behavior and contracts

1. Every agent has a stable ID, version, capability declaration, narrow
   permissions, typed schemas, and resource budget.
2. No unrestricted "god agent."
3. Validate agent output schema, provenance, authorization, and policy
   before use.
4. Distinguish observed evidence, derived features, hypotheses, and
   recommendations.
5. Findings include evidence references, timestamp, agent/model
   version, uncertainty, limitations, and trace ID.
6. Do not present arbitrary LLM confidence as a calibrated probability.
7. The orchestrator handles timeouts, cancellation, bounded retries,
   duplicate results, partial completion, conflicting findings, and
   unavailable agents.
8. Never retry a financial side effect without idempotency protection.
9. Agents do not write to other agents' state or shared tables outside
   approved interfaces.
10. Version agent messages and define compatibility/migration behavior.
11. Preserve conflicting findings; resolve them through documented
    aggregation/policy logic.
12. UI must show which agents ran, failed, timed out, or were skipped.

## 5. Data and model governance

1. Process only data authorized for the documented purpose.
2. Record provenance, schema, ownership, retention, and access policy
   for every data source.
3. Do not infer or use protected/sensitive attributes for risk scoring.
4. Separate raw events, normalized records, derived features, labels,
   model outputs, and policy decisions.
5. Model predictions and agent-generated labels are never verified
   ground truth.
6. Train/evaluate only against verified outcomes with documented label
   definitions.
7. Use time-aware train/validation/test splits to reduce leakage.
8. Version datasets, features, models, training code, evaluation
   metrics, and approvals.
9. Monitor drift, false positives/negatives, calibration, and data
   quality.
10. Evaluate new models offline or in shadow mode before production use.
11. Require approval, rollback, and version pinning for model promotion.
12. No autonomous self-modification, self-deployment, or production
    model update by agents.
13. Recommendations include concise rationale, evidence, expected
    benefit, uncertainty, and guardrails.

## 6. API, database, and events

1. Document schemas, versions, authentication, authorization,
   validation, and error contracts for every API/event.
2. Use typed domain models; avoid untyped JSON for financial or policy
   state.
3. Use reviewed database migrations with a recovery/rollback plan.
4. Protect invariants with transactions and database constraints, not
   only application checks.
5. Store timestamps in UTC; document display timezone conversion.
6. Use pagination, bounded limits, and timeouts.
7. Make event handlers idempotent and persist processed event IDs where
   needed.
8. Do not put secrets or unnecessary personal data in event payloads.
9. Define retry, dead-letter, replay, and poison-message handling.
10. Propagate correlation IDs across API, orchestration, agents, PayPal,
    and audit records.

## 7. Frontend and UX

1. Use the established design system and shared components.
2. Build responsive, accessible interfaces with keyboard support,
   semantic labels, visible focus, and adequate contrast.
3. Provide loading, empty, error, stale, partial-result, and
   permission-denied states.
4. Visually distinguish AI recommendations, policy outcomes, human
   decisions, and confirmed PayPal states.
5. Show evidence, reason codes, confidence caveats, timestamps, agent
   status, and model/policy versions where relevant.
6. Consequential operator actions require confirmation showing exact
   target, amount, currency, and effect.
7. Never store secrets or sensitive payment data in browser storage.
8. Mock data must be isolated from production paths and clearly
   labelled.
9. Animation must not obscure status, delay workflows, or reduce
   accessibility.
10. Mandatory UI Theme & Stack: Always use shadcn/ui components with a
    clean, light/white neumorphic theme (`--neu-bg: #e0e5ec` or soft-white
    dual-shadow aesthetic with GSAP animations via `useGSAP`). Never use
    raw HTML when a shadcn component can be wrapped.


## 8. Code quality

1. Follow existing conventions and pinned dependency versions. Preserve
   lockfiles.
2. Make small, reviewable changes. Avoid unrelated rewrites.
3. Avoid Rust `unwrap`, `expect`, panic-based control flow, and
   unchecked conversions in production paths; document justified
   exceptions.
4. Never suppress warnings to hide defects.
5. Handle errors explicitly and return safe, non-sensitive client
   messages.
6. Do not duplicate business rules across frontend, agents, and
   backend.
7. Comment non-obvious security/domain invariants.
8. Remove dead code, debug output, temporary credentials, and unused
   dependencies before merge.
9. Do not introduce dependencies without a clear need and review.

## 9. Testing and quality gates

Add relevant:
- Unit tests for domain logic and policy outcomes.
- Integration tests for database, APIs, and service boundaries.
- Contract tests for APIs, agent messages, and events.
- Negative tests for malformed, unauthorized, expired, replayed, and
  oversized inputs.
- Sandbox/mock payment tests and idempotency tests.
- Webhook signature, duplicate, delayed, and out-of-order tests.
- Tenant-isolation and authorization tests.
- Prompt-injection and tool-permission tests.
- UI loading, failure, stale, and partial-state tests.
- Timeout, load, recovery, and failure-injection tests for critical
  paths.

Before merge, run applicable formatters, linters, type checks, tests,
and builds. Record exact commands and results. Identify skipped tests
and their reason.

## 10. Git and collaboration

1. Work in focused feature/phase branches; never commit directly to
   protected main.
2. Use descriptive branches such as `phase-09-policy-engine` or
   `feature/agent-evidence-schema`.
3. PRs state purpose, scope, decisions, API/data changes, tests,
   security impact, limitations, and screenshots when relevant.
4. Shared contracts require review by both Sanjay and Kaviya.
5. Coordinate merge conflicts; never overwrite the other member's work
   without agreement.
6. Keep secrets, local databases, build output, and user data out of
   Git.
7. Every task has one accountable owner (Sanjay or Kaviya) and the
   other reviews.
8. Sanjay coordinates integration; both members remain accountable for
   correctness in their owned modules.

## 11. Team ownership

| Domain | Owner | Reviewer |
|--------|-------|----------|
| Architecture, scope, integration schedule, release gates, demo | **Sanjay** | Kaviya |
| Rust API, auth, database, policy engine, PayPal adapter, webhooks, reconciliation, audit, backend hardening | **Sanjay** | Kaviya |
| Agent contracts, orchestration, specialist agents, evidence aggregation, model evaluation, learning | **Kaviya** | Sanjay |
| React/TypeScript UI, dashboards, agent workspace, investigation flows, API integration, accessibility | **Kaviya** | Sanjay |

Cross-cutting work (testing, documentation, CI/CD, demo prep) is
shared. Role owner means accountable delivery, not exclusive
contribution.

## 12. Forty-phase roadmap control

| Stage | Phases | Scope | Primary | Reviewer |
|-------|--------|-------|---------|----------|
| A | 1–5 | Requirements, architecture, repo, data model, auth/tenancy | Sanjay | Kaviya |
| B | 6–10 | Rust core, messaging, agent registry, policy engine, audit | Sanjay | Kaviya |
| C | 11–17 | Orchestrator, specialist risk agents, risk aggregation | Kaviya | Sanjay |
| D | 18–24 | Payment, checkout, customer, recs, refunds, disputes, ops | Split* | Cross-review |
| E | 25–30 | Historical features, feedback, model eval, adversarial testing | Kaviya | Sanjay |
| F | 31–34 | PayPal Sandbox, gateway, webhooks/reconciliation, financial safety | Sanjay | Kaviya |
| G | 35–37 | Command center, agent workspace, investigation/analytics UI | Kaviya | Sanjay |
| H | 38–40 | E2E integration, security/perf validation, deployment/demo | Both | Both |

*Stage D split: Sanjay owns payment/refund/dispute backend (phases 18,
23, 24); Kaviya owns checkout/customer/recommendation agents and UI
(phases 19–22).

Do not silently skip prerequisites. A mock is not a completed external
integration. Every phase needs objective, prerequisites, owner,
reviewer, contracts, implementation tasks, tests, security checks,
acceptance criteria, evidence, and handoff.

## 13. AI coding-agent response rules

1. Classify the request: feature, bug, refactor, security, or
   documentation.
2. Identify affected phase, modules, and owner (Sanjay or Kaviya).
3. Inspect source files before proposing repository edits.
4. Ask one concise clarification only when a missing decision blocks
   safe work.
5. Do not rewrite the application when a focused change is requested.
6. State assumptions and distinguish plans from completed work.
7. Never claim repo access, file edits, tests, builds, or external
   verification that did not occur.
8. Explain unsafe requests and propose a safer implementation.
9. If a request conflicts with these rules, document the conflict and
   seek an approved exception.

## 14. Release and demo

1. Use reproducible builds and documented environment configuration.
2. Keep Sandbox and production credentials/configuration separate.
3. Never demo with real customer data, live credentials, or real money
   movement.
4. Prepare synthetic scenarios for normal payment, suspicious
   transaction, step-up, manual review, payment failure, refund
   analysis, dispute investigation, and agent outage.
5. Demonstrate claims with traces, agent evidence, policy outcomes,
   audit records, and authoritative Sandbox state.
6. Document simulated behavior and limitations.
7. Verify clean install, startup, health checks, migrations, and
   recovery procedures.
8. Release only after quality gates pass and critical/high security
   findings are resolved or explicitly accepted by Sanjay.

## 15. Exception process

Any exception must record the rule, reason, affected components,
impact/threat assessment, compensating controls, approver (Sanjay for
architecture/security, module owner for implementation), review/expiry
date, tests, and evidence. Temporary exceptions must not silently become
permanent.

## 16. Mandatory Phase-by-Phase Interactive Review & User Approval Gate

1. **No Silent Phase Progression:** The AI agent and engineering team must **never** advance to or begin building the next phase without explicit, written user approval.
2. **Interactive Application Launch:** At the completion of each phase, the application must be launched (e.g. frontend dev server and backend API running) so the user can interact directly with the running software.
3. **Component & Module Verification Checklist:** The agent must present a detailed breakdown of:
   - All completed features in the phase.
   - All created/updated components, modules, and crates.
   - Verification commands run and test evidence.
4. **User Verification Prompt:** The agent must explicitly ask the user to inspect the UI, verify components and modules, and test workflows before requesting approval to proceed.

