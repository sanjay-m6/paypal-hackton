CREATE TABLE IF NOT EXISTS agent_message_inbox (
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE RESTRICT,
    message_id UUID NOT NULL,
    task_id UUID NOT NULL,
    envelope JSONB NOT NULL,
    expires_at TIMESTAMPTZ NOT NULL,
    received_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    processed_at TIMESTAMPTZ,
    status VARCHAR(16) NOT NULL DEFAULT 'received'
        CHECK (status IN ('received', 'processing', 'processed', 'failed')),
    lease_until TIMESTAMPTZ,
    attempts INT NOT NULL DEFAULT 0 CHECK (attempts >= 0),
    failure_code VARCHAR(64),
    PRIMARY KEY (tenant_id, message_id)
);

CREATE INDEX IF NOT EXISTS idx_agent_message_inbox_task
    ON agent_message_inbox (tenant_id, task_id, received_at DESC);

CREATE INDEX IF NOT EXISTS idx_agent_message_inbox_recovery
    ON agent_message_inbox (lease_until)
    WHERE status = 'processing';

CREATE TABLE IF NOT EXISTS agent_message_outbox (
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE RESTRICT,
    message_id UUID NOT NULL,
    task_id UUID NOT NULL,
    subject VARCHAR(255) NOT NULL,
    envelope JSONB NOT NULL,
    expires_at TIMESTAMPTZ NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    next_attempt_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    published_at TIMESTAMPTZ,
    dead_lettered_at TIMESTAMPTZ,
    attempts INT NOT NULL DEFAULT 0 CHECK (attempts >= 0),
    last_error_code VARCHAR(64),
    PRIMARY KEY (tenant_id, message_id)
);

CREATE INDEX IF NOT EXISTS idx_agent_message_outbox_pending
    ON agent_message_outbox (tenant_id, next_attempt_at, created_at)
    WHERE published_at IS NULL AND dead_lettered_at IS NULL;

CREATE INDEX IF NOT EXISTS idx_agent_message_outbox_expiry
    ON agent_message_outbox (expires_at)
    WHERE published_at IS NULL AND dead_lettered_at IS NULL;

CREATE TABLE IF NOT EXISTS agent_message_dead_letters (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE RESTRICT,
    message_id UUID,
    task_id UUID,
    original_subject VARCHAR(255) NOT NULL,
    delivery_attempt INT NOT NULL CHECK (delivery_attempt > 0),
    reason_code VARCHAR(64) NOT NULL,
    recorded_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_agent_message_dead_letters_tenant
    ON agent_message_dead_letters (tenant_id, recorded_at DESC);
