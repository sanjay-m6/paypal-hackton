# Phase 4: Database Architecture & Data Governance

**Document ID:** `DATA-GOV-004`  
**Data Governance Lead:** Sanjay (Team Lead & Backend)  
**AI Intelligence Reviewer:** Kaviya (Full-Stack & AI)  
**Status:** 🚀 IN PROGRESS  
**Date:** 2026-11-09  

---

## 1. Governance Principles & Compliance

The platform adheres to strict data minimization, purpose limitation, and tenant isolation requirements (aligned with **Project Rule 2.16**, **Rule 5.1**, and PCI-DSS / GDPR standards):

1. **Purpose Limitation:** Every data element ingested (IP, device headers, payment amounts) must have a documented security or fraud prevention purpose. No extraneous telemetry is stored.
2. **Zero-Storage Invariant for Cardholder Data (CHD):** Primary account numbers (PAN), CVVs, and magnetic stripe data are **never** accepted, stored, or logged.
3. **Strict Row-Level Tenant Isolation:** Every query executing against transactional tables (`transactions`, `policy_decisions`, `agent_findings`, `audit_events`, `idempotency_keys`) MUST include `WHERE tenant_id = $1` at the repository level. Browser-supplied tenant parameters are untrusted.
4. **Immutable Audit Ledger:** The `audit_events` table is append-only. Updates and deletes are prohibited at both database permission and repository interface levels.

---

## 2. Retention Lifecycles & Deletion Workflows

| Table / Data Category | Retention Period | Storage Location | Purge / Archive Mechanism |
|---|---|---|---|
| **`idempotency_keys`** | 24 Hours | PostgreSQL | Automated partition purge / `DELETE WHERE expires_at < NOW()` |
| **`transactions`** | 7 Years | PostgreSQL / Cold Store | Retained for financial regulatory compliance |
| **`policy_decisions`** | 7 Years | PostgreSQL | Retained alongside transaction records for dispute defense |
| **`agent_findings`** | 180 Days | PostgreSQL | Truncated to summary features after 180 days |
| **`audit_events`** | 7 Years | PostgreSQL Append-Only | Cryptographically signed, exportable to WORM storage |
| **Session & Device Signals** | 90 Days | PostgreSQL | PII pseudonymized after 90 days |

---

## 3. PII Pseudonymization & Right-to-be-Forgotten Workflow

When a tenant or customer exercises an authorized erasure request:
1. Customer identifiers in `transactions` (`customer_id`, `email`, `ip_address`) are overwritten with an irreversible cryptographic HMAC hash (`HMAC-SHA256(customer_id, tenant_salt)`).
2. Financial amounts, currencies, timestamps, policy reason codes, and dispute records are preserved for legal and financial audit obligations without linking back to natural persons.
3. The erasure action itself is logged in `audit_events` with the operator ID and cryptographic hash.

