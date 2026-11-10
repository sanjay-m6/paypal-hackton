use chrono::{DateTime, Utc};
use serde_json::Value;
use sqlx::{PgPool, Row};
use uuid::Uuid;

use payment_core::{AgentMessageEnvelope, PlatformError, types::TenantId};

#[derive(Debug, Clone)]
pub struct OutboxMessage {
    pub tenant_id: Uuid,
    pub message_id: Uuid,
    pub task_id: Uuid,
    pub subject: String,
    pub envelope: Value,
    pub attempts: i32,
}

pub struct InboxOutboxCompletion<'a> {
    pub tenant_id: TenantId,
    pub inbound_message_id: Uuid,
    pub task_id: Uuid,
    pub inbound_envelope: &'a Value,
    pub outbound_message_id: Uuid,
    pub outbound_subject: &'a str,
    pub outbound_envelope: &'a Value,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InboxClaim {
    Acquired,
    AlreadyProcessed,
    AlreadyFailed,
    InProgress,
    Conflict,
}

#[derive(Debug, Clone)]
pub struct MessageRepository {
    pool: PgPool,
}

impl MessageRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn record_inbox(
        &self,
        tenant_id: TenantId,
        message_id: Uuid,
        task_id: Uuid,
        envelope: &Value,
    ) -> Result<bool, PlatformError> {
        let expires_at = validate_envelope_metadata(tenant_id, message_id, task_id, envelope)?;
        let inserted = sqlx::query(
            r#"
            INSERT INTO agent_message_inbox
                (tenant_id, message_id, task_id, envelope, expires_at)
            VALUES ($1, $2, $3, $4, $5)
            ON CONFLICT (tenant_id, message_id) DO NOTHING
            RETURNING message_id
            "#,
        )
        .bind(tenant_id.0)
        .bind(message_id)
        .bind(task_id)
        .bind(envelope)
        .bind(expires_at)
        .fetch_optional(&self.pool)
        .await
        .map_err(database_error)?;
        Ok(inserted.is_some())
    }

    pub async fn claim_inbox(
        &self,
        tenant_id: TenantId,
        message_id: Uuid,
        task_id: Uuid,
        envelope: &Value,
        lease_duration: chrono::Duration,
    ) -> Result<InboxClaim, PlatformError> {
        let expires_at = validate_envelope_metadata(tenant_id, message_id, task_id, envelope)?;
        if lease_duration <= chrono::Duration::zero()
            || lease_duration > chrono::Duration::minutes(5)
        {
            return Err(PlatformError::Validation(
                "Message processing lease must be between 0 and 5 minutes".into(),
            ));
        }
        let now = Utc::now();
        let lease_until = now + lease_duration;
        let claimed = sqlx::query_scalar::<_, Uuid>(
            r#"
            INSERT INTO agent_message_inbox
                (tenant_id, message_id, task_id, envelope, expires_at, status, lease_until, attempts)
            VALUES ($1, $2, $3, $4, $5, 'processing', $6, 1)
            ON CONFLICT (tenant_id, message_id) DO UPDATE
            SET status = 'processing',
                lease_until = EXCLUDED.lease_until,
                attempts = agent_message_inbox.attempts + 1,
                failure_code = NULL,
                updated_at = NOW()
            WHERE agent_message_inbox.status NOT IN ('processed', 'failed')
                AND agent_message_inbox.task_id = EXCLUDED.task_id
                AND agent_message_inbox.envelope = EXCLUDED.envelope
                AND (
                    agent_message_inbox.status <> 'processing'
                    OR agent_message_inbox.lease_until <= NOW()
                )
            RETURNING message_id
            "#,
        )
        .bind(tenant_id.0)
        .bind(message_id)
        .bind(task_id)
        .bind(envelope)
        .bind(expires_at)
        .bind(lease_until)
        .fetch_optional(&self.pool)
        .await
        .map_err(database_error)?;

        if claimed.is_some() {
            return Ok(InboxClaim::Acquired);
        }
        let existing = sqlx::query_as::<_, (String, Uuid, Value)>(
            "SELECT status, task_id, envelope FROM agent_message_inbox WHERE tenant_id = $1 AND message_id = $2",
        )
        .bind(tenant_id.0)
        .bind(message_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(database_error)?;
        match existing {
            Some((_, existing_task_id, existing_envelope))
                if existing_task_id != task_id || existing_envelope != *envelope =>
            {
                Ok(InboxClaim::Conflict)
            }
            Some((status, _, _)) if status == "processed" => Ok(InboxClaim::AlreadyProcessed),
            Some((status, _, _)) if status == "failed" => Ok(InboxClaim::AlreadyFailed),
            Some((status, _, _)) if status == "processing" => Ok(InboxClaim::InProgress),
            Some((status, _, _)) if status == "received" => Err(PlatformError::DatabaseError(
                "Message inbox claim could not be acquired".into(),
            )),
            None => Err(PlatformError::DatabaseError(
                "Message inbox row disappeared during claim".into(),
            )),
            _ => Err(PlatformError::DatabaseError(
                "Message inbox row has an unknown state".into(),
            )),
        }
    }

    pub async fn release_inbox(
        &self,
        tenant_id: TenantId,
        message_id: Uuid,
        safe_failure_code: &str,
    ) -> Result<bool, PlatformError> {
        validate_code(safe_failure_code)?;
        let result = sqlx::query(
            r#"
            UPDATE agent_message_inbox
            SET status = 'received',
                lease_until = NULL,
                failure_code = $3,
                updated_at = NOW()
            WHERE tenant_id = $1 AND message_id = $2 AND status = 'processing'
            "#,
        )
        .bind(tenant_id.0)
        .bind(message_id)
        .bind(safe_failure_code)
        .execute(&self.pool)
        .await
        .map_err(database_error)?;
        Ok(result.rows_affected() == 1)
    }

    pub async fn fail_inbox(
        &self,
        tenant_id: TenantId,
        message_id: Uuid,
        safe_failure_code: &str,
    ) -> Result<bool, PlatformError> {
        validate_code(safe_failure_code)?;
        let result = sqlx::query(
            r#"
            UPDATE agent_message_inbox
            SET status = 'failed',
                processed_at = NOW(),
                lease_until = NULL,
                failure_code = $3,
                updated_at = NOW()
            WHERE tenant_id = $1 AND message_id = $2 AND status = 'processing'
            "#,
        )
        .bind(tenant_id.0)
        .bind(message_id)
        .bind(safe_failure_code)
        .execute(&self.pool)
        .await
        .map_err(database_error)?;
        Ok(result.rows_affected() == 1)
    }

    pub async fn complete_inbox(
        &self,
        tenant_id: TenantId,
        message_id: Uuid,
    ) -> Result<bool, PlatformError> {
        let result = sqlx::query(
            r#"
            UPDATE agent_message_inbox
            SET status = 'processed',
                processed_at = NOW(),
                lease_until = NULL,
                failure_code = NULL,
                updated_at = NOW()
            WHERE tenant_id = $1 AND message_id = $2 AND status = 'processing'
            "#,
        )
        .bind(tenant_id.0)
        .bind(message_id)
        .execute(&self.pool)
        .await
        .map_err(database_error)?;
        Ok(result.rows_affected() == 1)
    }

    pub async fn enqueue_outbox(
        &self,
        tenant_id: TenantId,
        message_id: Uuid,
        task_id: Uuid,
        subject: &str,
        envelope: &Value,
    ) -> Result<bool, PlatformError> {
        validate_subject(subject)?;
        let expires_at = validate_envelope_metadata(tenant_id, message_id, task_id, envelope)?;
        let inserted = sqlx::query(
            r#"
            INSERT INTO agent_message_outbox
                (tenant_id, message_id, task_id, subject, envelope, expires_at)
            VALUES ($1, $2, $3, $4, $5, $6)
            ON CONFLICT (tenant_id, message_id) DO NOTHING
            RETURNING message_id
            "#,
        )
        .bind(tenant_id.0)
        .bind(message_id)
        .bind(task_id)
        .bind(subject)
        .bind(envelope)
        .bind(expires_at)
        .fetch_optional(&self.pool)
        .await
        .map_err(database_error)?;
        Ok(inserted.is_some())
    }

    pub async fn complete_inbox_with_outbox(
        &self,
        completion: InboxOutboxCompletion<'_>,
    ) -> Result<bool, PlatformError> {
        validate_subject(completion.outbound_subject)?;
        validate_envelope_metadata(
            completion.tenant_id,
            completion.inbound_message_id,
            completion.task_id,
            completion.inbound_envelope,
        )?;
        let outbound_expires_at = validate_envelope_metadata(
            completion.tenant_id,
            completion.outbound_message_id,
            completion.task_id,
            completion.outbound_envelope,
        )?;
        let mut transaction = self.pool.begin().await.map_err(database_error)?;
        let updated = sqlx::query(
            r#"
            UPDATE agent_message_inbox
            SET status = 'processed',
                processed_at = NOW(),
                lease_until = NULL,
                failure_code = NULL,
                updated_at = NOW()
            WHERE tenant_id = $1
                AND message_id = $2
                AND task_id = $3
                AND status = 'processing'
            RETURNING message_id
            "#,
        )
        .bind(completion.tenant_id.0)
        .bind(completion.inbound_message_id)
        .bind(completion.task_id)
        .fetch_optional(&mut *transaction)
        .await
        .map_err(database_error)?;

        if updated.is_none() {
            transaction.commit().await.map_err(database_error)?;
            return Ok(false);
        }

        sqlx::query(
            r#"
            INSERT INTO agent_message_outbox
                (tenant_id, message_id, task_id, subject, envelope, expires_at)
            VALUES ($1, $2, $3, $4, $5, $6)
            "#,
        )
        .bind(completion.tenant_id.0)
        .bind(completion.outbound_message_id)
        .bind(completion.task_id)
        .bind(completion.outbound_subject)
        .bind(completion.outbound_envelope)
        .bind(outbound_expires_at)
        .execute(&mut *transaction)
        .await
        .map_err(database_error)?;
        transaction.commit().await.map_err(database_error)?;
        Ok(true)
    }

    pub async fn pending_outbox(
        &self,
        tenant_id: TenantId,
        limit: i64,
    ) -> Result<Vec<OutboxMessage>, PlatformError> {
        if !(1..=500).contains(&limit) {
            return Err(PlatformError::Validation(
                "Outbox page size must be between 1 and 500".into(),
            ));
        }

        let rows = sqlx::query(
            r#"
            SELECT tenant_id, message_id, task_id, subject, envelope, attempts
            FROM agent_message_outbox
            WHERE tenant_id = $1
                AND published_at IS NULL
                AND dead_lettered_at IS NULL
                AND expires_at > NOW()
                AND next_attempt_at <= NOW()
            ORDER BY created_at, message_id
            LIMIT $2
            "#,
        )
        .bind(tenant_id.0)
        .bind(limit)
        .fetch_all(&self.pool)
        .await
        .map_err(database_error)?;

        Ok(rows
            .into_iter()
            .map(|row| OutboxMessage {
                tenant_id: row.get("tenant_id"),
                message_id: row.get("message_id"),
                task_id: row.get("task_id"),
                subject: row.get("subject"),
                envelope: row.get("envelope"),
                attempts: row.get("attempts"),
            })
            .collect())
    }

    pub async fn pending_tenants(&self, limit: i64) -> Result<Vec<TenantId>, PlatformError> {
        if !(1..=500).contains(&limit) {
            return Err(PlatformError::Validation(
                "Outbox tenant page size must be between 1 and 500".into(),
            ));
        }
        let rows = sqlx::query(
            r#"
            SELECT DISTINCT tenant_id
            FROM agent_message_outbox
            WHERE published_at IS NULL
                AND dead_lettered_at IS NULL
                AND (expires_at <= NOW() OR next_attempt_at <= NOW())
            ORDER BY tenant_id
            LIMIT $1
            "#,
        )
        .bind(limit)
        .fetch_all(&self.pool)
        .await
        .map_err(database_error)?;
        Ok(rows
            .into_iter()
            .map(|row| TenantId(row.get("tenant_id")))
            .collect())
    }

    pub async fn mark_outbox_published(
        &self,
        tenant_id: TenantId,
        message_id: Uuid,
    ) -> Result<bool, PlatformError> {
        let result = sqlx::query(
            r#"
            UPDATE agent_message_outbox
            SET published_at = NOW(), last_error_code = NULL
            WHERE tenant_id = $1
                AND message_id = $2
                AND published_at IS NULL
                AND dead_lettered_at IS NULL
            "#,
        )
        .bind(tenant_id.0)
        .bind(message_id)
        .execute(&self.pool)
        .await
        .map_err(database_error)?;
        Ok(result.rows_affected() == 1)
    }

    pub async fn record_outbox_failure(
        &self,
        tenant_id: TenantId,
        message_id: Uuid,
        safe_error_code: &str,
    ) -> Result<bool, PlatformError> {
        validate_code(safe_error_code)?;
        let updated = sqlx::query(
            r#"
            WITH updated AS (
                UPDATE agent_message_outbox
                SET attempts = attempts + 1,
                    last_error_code = $3,
                    next_attempt_at = NOW() + LEAST(
                        POWER(2, LEAST(attempts, 8)) * INTERVAL '1 second',
                        INTERVAL '5 minutes'
                    ),
                    dead_lettered_at = CASE
                        WHEN attempts + 1 >= 10 THEN NOW()
                        ELSE NULL
                    END
                WHERE tenant_id = $1
                    AND message_id = $2
                    AND published_at IS NULL
                    AND dead_lettered_at IS NULL
                RETURNING tenant_id, message_id, task_id, subject, attempts,
                    last_error_code, dead_lettered_at
            ),
            recorded_dead_letter AS (
                INSERT INTO agent_message_dead_letters
                    (tenant_id, message_id, task_id, original_subject, delivery_attempt, reason_code)
                SELECT tenant_id, message_id, task_id, subject, attempts, last_error_code
                FROM updated
                WHERE dead_lettered_at IS NOT NULL
                RETURNING id
            )
            SELECT updated.message_id
            FROM updated
            LEFT JOIN recorded_dead_letter ON TRUE
            "#,
        )
        .bind(tenant_id.0)
        .bind(message_id)
        .bind(safe_error_code)
        .fetch_optional(&self.pool)
        .await
        .map_err(database_error)?;
        Ok(updated.is_some())
    }

    pub async fn record_dead_letter(
        &self,
        tenant_id: TenantId,
        message_id: Option<Uuid>,
        task_id: Option<Uuid>,
        original_subject: &str,
        delivery_attempt: i32,
        reason_code: &str,
    ) -> Result<(), PlatformError> {
        validate_subject(original_subject)?;
        validate_code(reason_code)?;
        if delivery_attempt <= 0 {
            return Err(PlatformError::Validation(
                "Dead-letter delivery attempt must be positive".into(),
            ));
        }
        sqlx::query(
            r#"
            INSERT INTO agent_message_dead_letters
                (tenant_id, message_id, task_id, original_subject, delivery_attempt, reason_code)
            VALUES ($1, $2, $3, $4, $5, $6)
            "#,
        )
        .bind(tenant_id.0)
        .bind(message_id)
        .bind(task_id)
        .bind(original_subject)
        .bind(delivery_attempt)
        .bind(reason_code)
        .execute(&self.pool)
        .await
        .map_err(database_error)?;
        Ok(())
    }

    pub async fn expire_pending_outbox(&self, tenant_id: TenantId) -> Result<i64, PlatformError> {
        let result = sqlx::query(
            r#"
            WITH expired AS (
                UPDATE agent_message_outbox
                SET dead_lettered_at = NOW(),
                    attempts = attempts + 1,
                    last_error_code = 'message_expired'
                WHERE published_at IS NULL
                    AND dead_lettered_at IS NULL
                    AND expires_at <= NOW()
                    AND tenant_id = $1
                RETURNING tenant_id, message_id, task_id, subject, attempts
            ),
            recorded AS (
                INSERT INTO agent_message_dead_letters
                    (tenant_id, message_id, task_id, original_subject, delivery_attempt, reason_code)
                SELECT tenant_id, message_id, task_id, subject, GREATEST(attempts, 1),
                    'message_expired'
                FROM expired
                RETURNING id
            )
            SELECT COUNT(*) AS expired_count FROM recorded
            "#,
        )
        .bind(tenant_id.0)
        .fetch_one(&self.pool)
        .await
        .map_err(database_error)?;
        Ok(result.get("expired_count"))
    }

    pub async fn purge_expired_records(&self) -> Result<(), PlatformError> {
        let mut transaction = self.pool.begin().await.map_err(database_error)?;
        sqlx::query("DELETE FROM agent_message_inbox WHERE expires_at < NOW() - INTERVAL '7 days'")
            .execute(&mut *transaction)
            .await
            .map_err(database_error)?;
        sqlx::query(
            r#"
            DELETE FROM agent_message_outbox
            WHERE expires_at < NOW() - INTERVAL '7 days'
                AND (published_at IS NOT NULL OR dead_lettered_at IS NOT NULL)
            "#,
        )
        .execute(&mut *transaction)
        .await
        .map_err(database_error)?;
        sqlx::query(
            "DELETE FROM agent_message_dead_letters WHERE recorded_at < NOW() - INTERVAL '90 days'",
        )
        .execute(&mut *transaction)
        .await
        .map_err(database_error)?;
        transaction.commit().await.map_err(database_error)
    }
}

fn validate_subject(subject: &str) -> Result<(), PlatformError> {
    if subject.is_empty()
        || subject.len() > 255
        || !subject
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b'_' | b'-'))
    {
        return Err(PlatformError::Validation(
            "Message subject contains invalid characters".into(),
        ));
    }
    Ok(())
}

fn validate_envelope_metadata(
    tenant_id: TenantId,
    message_id: Uuid,
    task_id: Uuid,
    envelope: &Value,
) -> Result<DateTime<Utc>, PlatformError> {
    let envelope: AgentMessageEnvelope =
        serde_json::from_value(envelope.clone()).map_err(|_| {
            PlatformError::Validation("Message envelope is not a valid typed contract".into())
        })?;
    if envelope.tenant_id != tenant_id
        || envelope.message_id != message_id
        || envelope.task_id != task_id
    {
        return Err(PlatformError::Validation(
            "Message envelope metadata does not match its tenant-scoped record".into(),
        ));
    }
    Ok(envelope.expires_at.0)
}

fn validate_code(code: &str) -> Result<(), PlatformError> {
    if code.is_empty()
        || code.len() > 64
        || !code
            .bytes()
            .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'_')
    {
        return Err(PlatformError::Validation(
            "Message error code contains invalid characters".into(),
        ));
    }
    Ok(())
}

fn database_error(error: sqlx::Error) -> PlatformError {
    PlatformError::DatabaseError(error.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Duration;
    use payment_core::{
        AgentId, AgentMessagePayload, AgentMessageType, AgentPayloadType, AgentRequest,
        MessagePrincipal,
        types::{Timestamp, TraceContext, UserId},
    };

    #[test]
    fn validates_subjects_and_error_codes() {
        assert!(validate_subject("agent.task.transaction_analysis").is_ok());
        assert!(validate_subject("agent.task.*").is_err());
        assert!(validate_code("handler_failed").is_ok());
        assert!(validate_code("database password leaked").is_err());
    }

    #[test]
    fn message_records_must_match_signed_tenant_and_ids() {
        let tenant_id = TenantId(Uuid::new_v4());
        let message_id = Uuid::new_v4();
        let task_id = Uuid::new_v4();
        let trace = TraceContext::new(tenant_id, UserId(Uuid::new_v4()));
        let now = chrono::Utc::now();
        let envelope = AgentMessageEnvelope {
            message_id,
            task_id,
            tenant_id,
            sender_id: MessagePrincipal::Orchestrator,
            recipient_id: MessagePrincipal::Agent(AgentId::TransactionAnalysis),
            message_type: AgentMessageType::AgentTaskRequest,
            payload_type: AgentPayloadType::AgentTask,
            schema_version: "1.0".into(),
            issued_at: Timestamp(now),
            expires_at: Timestamp(now + Duration::seconds(60)),
            correlation_id: trace.correlation_id.0,
            trace_id: Uuid::new_v4(),
            payload: AgentMessagePayload::AgentTask {
                request: AgentRequest {
                    task_id,
                    trace,
                    agent_id: AgentId::TransactionAnalysis,
                    payload: serde_json::json!({}),
                    timeout_ms: 500,
                },
            },
            key_id: "orchestrator-1".into(),
            signature: "signature".into(),
        };
        let value = serde_json::to_value(envelope).expect("serialize typed envelope");

        assert!(validate_envelope_metadata(tenant_id, message_id, task_id, &value).is_ok());
        assert!(
            validate_envelope_metadata(TenantId(Uuid::new_v4()), message_id, task_id, &value)
                .is_err()
        );
    }
}
