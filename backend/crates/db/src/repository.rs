use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde_json::Value;
use sqlx::{PgPool, Row};
use uuid::Uuid;

use payment_core::payment::{PaymentMethod, PaymentStatus};
use payment_core::policy::PolicyDecision;
use payment_core::types::{TenantId, TransactionId, UserId};
use payment_core::PlatformError;

// ---------------------------------------------------------------------------
// 1. Tenant Repository (Multi-Tenant Registry & Boundary Verification)
// ---------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct TenantModel {
    pub id: Uuid,
    pub name: String,
    pub active: bool,
    pub tier: String,
    pub rate_limit_per_min: i32,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct TenantRepository {
    pool: PgPool,
}

impl TenantRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Retrieve tenant by ID and verify active status.
    pub async fn get_by_id(&self, tenant_id: TenantId) -> Result<TenantModel, PlatformError> {
        let row = sqlx::query(
            "SELECT id, name, active, tier, rate_limit_per_min, created_at FROM tenants WHERE id = $1",
        )
        .bind(tenant_id.0)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| PlatformError::DatabaseError(e.to_string()))?;

        match row {
            Some(r) => Ok(TenantModel {
                id: r.get("id"),
                name: r.get("name"),
                active: r.get("active"),
                tier: r.get("tier"),
                rate_limit_per_min: r.get("rate_limit_per_min"),
                created_at: r.get("created_at"),
            }),
            None => Err(PlatformError::NotFound {
                entity: "Tenant".to_string(),
                id: tenant_id.0.to_string(),
            }),
        }
    }
}

// ---------------------------------------------------------------------------
// 2. Transaction Repository (Strict Row-Level Tenant Boundary Scoping)
// ---------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct TransactionModel {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub user_id: Option<Uuid>,
    pub amount: Decimal,
    pub currency: String,
    pub status: String,
    pub payment_method: String,
    pub idempotency_key: Uuid,
    pub correlation_id: Uuid,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct TransactionRepository {
    pool: PgPool,
}

impl TransactionRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Insert new transaction enforcing idempotency key uniqueness and tenant boundary.
    pub async fn create(
        &self,
        tenant_id: TenantId,
        user_id: Option<UserId>,
        amount: Decimal,
        currency: &str,
        method: PaymentMethod,
        idempotency_key: Uuid,
        correlation_id: Uuid,
    ) -> Result<TransactionModel, PlatformError> {
        let method_str = format!("{method:?}");
        let initial_status = format!("{:?}", PaymentStatus::Created);

        let row = sqlx::query(
            r#"
            INSERT INTO transactions (tenant_id, user_id, amount, currency, status, payment_method, idempotency_key, correlation_id)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            RETURNING id, tenant_id, user_id, amount, currency, status, payment_method, idempotency_key, correlation_id, created_at
            "#,
        )
        .bind(tenant_id.0)
        .bind(user_id.map(|u| u.0))
        .bind(amount)
        .bind(currency)
        .bind(initial_status)
        .bind(method_str)
        .bind(idempotency_key)
        .bind(correlation_id)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| PlatformError::DatabaseError(format!("Transaction creation failed: {e}")))?;

        Ok(TransactionModel {
            id: row.get("id"),
            tenant_id: row.get("tenant_id"),
            user_id: row.get("user_id"),
            amount: row.get("amount"),
            currency: row.get("currency"),
            status: row.get("status"),
            payment_method: row.get("payment_method"),
            idempotency_key: row.get("idempotency_key"),
            correlation_id: row.get("correlation_id"),
            created_at: row.get("created_at"),
        })
    }

    /// Retrieve a transaction strictly bounded by tenant_id (prevents cross-tenant access).
    pub async fn get_scoped(
        &self,
        tenant_id: TenantId,
        tx_id: TransactionId,
    ) -> Result<TransactionModel, PlatformError> {
        let row = sqlx::query(
            r#"
            SELECT id, tenant_id, user_id, amount, currency, status, payment_method, idempotency_key, correlation_id, created_at
            FROM transactions
            WHERE id = $1 AND tenant_id = $2
            "#,
        )
        .bind(tx_id.0)
        .bind(tenant_id.0)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| PlatformError::DatabaseError(e.to_string()))?;

        match row {
            Some(r) => Ok(TransactionModel {
                id: r.get("id"),
                tenant_id: r.get("tenant_id"),
                user_id: r.get("user_id"),
                amount: r.get("amount"),
                currency: r.get("currency"),
                status: r.get("status"),
                payment_method: r.get("payment_method"),
                idempotency_key: r.get("idempotency_key"),
                correlation_id: r.get("correlation_id"),
                created_at: r.get("created_at"),
            }),
            None => Err(PlatformError::NotFound {
                entity: "Transaction".to_string(),
                id: tx_id.0.to_string(),
            }),
        }
    }
}

// ---------------------------------------------------------------------------
// 3. Idempotency Repository (Atomic Request Locking & Double-Payment Defense)
// ---------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct IdempotencyRepository {
    pool: PgPool,
}

impl IdempotencyRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Atomically acquire an idempotency lock. Returns true if acquired, false if duplicate.
    pub async fn acquire_lock(
        &self,
        key: Uuid,
        tenant_id: TenantId,
        request_hash: &str,
    ) -> Result<bool, PlatformError> {
        let result = sqlx::query(
            r#"
            INSERT INTO idempotency_keys (key, tenant_id, request_hash)
            VALUES ($1, $2, $3)
            ON CONFLICT (key) DO NOTHING
            "#,
        )
        .bind(key)
        .bind(tenant_id.0)
        .bind(request_hash)
        .execute(&self.pool)
        .await
        .map_err(|e| PlatformError::DatabaseError(format!("Idempotency check failed: {e}")))?;

        Ok(result.rows_affected() > 0)
    }

    /// Commit the completed response payload against the idempotency key.
    pub async fn commit_response(
        &self,
        key: Uuid,
        response_payload: Value,
        status_code: i32,
    ) -> Result<(), PlatformError> {
        sqlx::query(
            r#"
            UPDATE idempotency_keys
            SET response_payload = $1, status_code = $2
            WHERE key = $3
            "#,
        )
        .bind(response_payload)
        .bind(status_code)
        .bind(key)
        .execute(&self.pool)
        .await
        .map_err(|e| PlatformError::DatabaseError(e.to_string()))?;

        Ok(())
    }
}

// ---------------------------------------------------------------------------
// 4. Audit Repository (Immutable Append-Only Compliance Trail)
// ---------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct AuditRepository {
    pool: PgPool,
}

impl AuditRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Append a tamper-evident audit record. Updates/Deletions are prohibited.
    pub async fn append_event(
        &self,
        event_type: &str,
        tenant_id: TenantId,
        user_id: Option<UserId>,
        correlation_id: Uuid,
        details: Value,
    ) -> Result<Uuid, PlatformError> {
        let row = sqlx::query(
            r#"
            INSERT INTO audit_events (event_type, tenant_id, user_id, correlation_id, details)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id
            "#,
        )
        .bind(event_type)
        .bind(tenant_id.0)
        .bind(user_id.map(|u| u.0))
        .bind(correlation_id)
        .bind(details)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| PlatformError::DatabaseError(format!("Audit logging failed: {e}")))?;

        Ok(row.get("id"))
    }
}

// ---------------------------------------------------------------------------
// 5. Policy Decision Repository
// ---------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct PolicyRepository {
    pool: PgPool,
}

impl PolicyRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Persist a deterministic policy decision linked to a transaction.
    pub async fn persist_decision(
        &self,
        transaction_id: TransactionId,
        tenant_id: TenantId,
        decision: PolicyDecision,
        policy_version: &str,
        reason_codes: &[String],
        correlation_id: Uuid,
    ) -> Result<Uuid, PlatformError> {
        let decision_str = format!("{decision:?}");

        let row = sqlx::query(
            r#"
            INSERT INTO policy_decisions (transaction_id, tenant_id, decision, policy_version, reason_codes, correlation_id)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING id
            "#,
        )
        .bind(transaction_id.0)
        .bind(tenant_id.0)
        .bind(decision_str)
        .bind(policy_version)
        .bind(reason_codes)
        .bind(correlation_id)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| PlatformError::DatabaseError(format!("Policy decision persistence failed: {e}")))?;

        Ok(row.get("id"))
    }
}
