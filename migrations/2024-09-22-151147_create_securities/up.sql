-- Your SQL goes here
CREATE TABLE IF NOT EXISTS securities (
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    security_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    symbol VARCHAR(7) UNIQUE NOT NULL
);