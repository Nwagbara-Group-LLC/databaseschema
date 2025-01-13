-- Your SQL goes here
CREATE TABLE IF NOT EXISTS exchanges (
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    exchange_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(8) UNIQUE NOT NULL
);