# Phase 5: Authentication, Authorization & Tenant Isolation

**Owner:** Sanjay
**Reviewer:** Kaviya
**Implementation status:** Implemented; awaiting Kaviya's application review

## Delivered

- `POST /api/v1/auth/login` looks up only active users in active tenants and verifies Argon2id password hashes.
- Successful login returns a signed HS256 bearer token with user, tenant, role, issuer, and 15-minute expiry claims.
- Protected API handlers verify the signature, issuer, expiry, UUID claims, and role allow-list.
- Payment evaluation is limited to `admin`, `operator`, and `risk_analyst`; request tenant IDs must match the verified token claim.
- Error responses distinguish unauthenticated (`401`), unauthorized (`403`), invalid input (`400`), missing resources (`404`), and internal failures without exposing internal error details.
- Login attempts are limited to 10 per identity in a rolling 60-second window, with a bounded in-memory tracker.
- Unknown users still incur a password-hash verification to reduce account-enumeration timing differences.
- CORS allows the local frontend origin only. `JWT_SECRET` is required at startup and must be at least 32 bytes.
- Optional local first-admin provisioning is controlled by `BOOTSTRAP_ADMIN_EMAIL` and `BOOTSTRAP_ADMIN_PASSWORD`; the password is hashed before it reaches PostgreSQL.

## Local operator provisioning

Set `BOOTSTRAP_ADMIN_EMAIL` and a unique `BOOTSTRAP_ADMIN_PASSWORD` of at least 12 characters in the local `.env`, start the API once, then remove the bootstrap values. The user is inserted only if the email is not already present. The default local tenant ID is `22222222-2222-4222-8222-222222222222`; override it with `BOOTSTRAP_TENANT_ID` if needed.

## Review checks

1. Login succeeds with the provisioned operator and fails with an incorrect password.
2. Requests to `/api/v1/agents` and `/api/v1/payments/evaluate` without a bearer token return `401`.
3. A malformed, tampered, wrong-issuer, or expired token is rejected.
4. A payment evaluation with another tenant's ID returns `403`.
5. A `read_only` account cannot evaluate payments.

Tokens are short-lived but are not currently revocable before expiry. Login throttling is process-local; multi-instance deployments need a shared rate-limit store. Refresh-token support remains future hardening work.
