-- Initial migrations placeholder for Phase 4
-- Full schema will be applied in Phase 4 (Database Architecture)
CREATE TABLE IF NOT EXISTS schema_initialization_check (
    id SERIAL PRIMARY KEY,
    initialized_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

