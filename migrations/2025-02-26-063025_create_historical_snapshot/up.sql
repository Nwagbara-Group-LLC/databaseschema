-- Your SQL goes here
CREATE TABLE historical_snapshot (
    event_id UUID NOT NULL DEFAULT gen_random_uuid(),
    timestamp TIMESTAMPTZ NOT NULL,
    order_id TEXT NOT NULL,
    event_type TEXT NOT NULL CHECK (event_type IN ('new', 'trade')),
    side TEXT NOT NULL CHECK (side IN ('buy', 'sell')),
    price_level NUMERIC NOT NULL,
    quantity NUMERIC NOT NULL,
    status TEXT NOT NULL CHECK (status IN ('open', 'filled', 'canceled')),
    exchange TEXT NOT NULL,
    symbol TEXT NOT NULL,
    exchange_id UUID NOT NULL REFERENCES exchanges (exchange_id),
    security_id UUID NOT NULL REFERENCES securities (security_id),
    PRIMARY KEY (timestamp, event_id),
    UNIQUE (timestamp, order_id, event_type)
);

-- Convert to TimescaleDB hypertable for high-frequency snapshot data
SELECT create_hypertable('historical_snapshot', 'timestamp', chunk_time_interval => interval '1 hour');

-- Enable compression with aggressive settings for snapshot data
ALTER TABLE historical_snapshot SET (
    timescaledb.compress,
    timescaledb.compress_segmentby = 'symbol, exchange',
    timescaledb.compress_orderby = 'timestamp DESC, event_id'
);

-- Add aggressive compression policy (compress after 1 day)
SELECT add_compression_policy('historical_snapshot', INTERVAL '1 day');

-- Add retention policy (keep snapshot data for 3 months)
SELECT add_retention_policy('historical_snapshot', INTERVAL '3 months');

-- Optimized indexes for compressed snapshot data
CREATE INDEX idx_historical_snapshot_timestamp ON historical_snapshot(timestamp DESC);
CREATE INDEX idx_historical_snapshot_order_id ON historical_snapshot(order_id);
CREATE INDEX idx_historical_snapshot_symbol_exchange ON historical_snapshot(symbol, exchange, timestamp DESC);
