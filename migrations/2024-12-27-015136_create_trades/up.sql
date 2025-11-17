-- Your SQL goes here
CREATE TABLE IF NOT EXISTS trades (
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    symbol VARCHAR(7) NOT NULL REFERENCES securities (symbol),
    exchange VARCHAR(8) NOT NULL REFERENCES exchanges (exchange),
    trade_id VARCHAR(255) NOT NULL,
    security_id UUID NOT NULL REFERENCES securities (security_id),
    exchange_id UUID NOT NULL REFERENCES exchanges (exchange_id),
    side VARCHAR(4) NOT NULL,
    price NUMERIC NOT NULL,
    quantity NUMERIC NOT NULL,
    PRIMARY KEY (created_at, trade_id)
);

-- Use 1-day chunks for efficient storage of high-frequency tick data
SELECT create_hypertable('trades', 'created_at', chunk_time_interval => interval '1 day');

-- Enable compression for old data (compress after 1 hour)
ALTER TABLE trades SET (
    timescaledb.compress,
    timescaledb.compress_segmentby = 'symbol, exchange, side',
    timescaledb.compress_orderby = 'created_at DESC, trade_id'
);

-- Compress data older than 1 hour for efficient storage
SELECT add_compression_policy('trades', INTERVAL '1 hour');

-- Keep 3 months of data (90 days)
SELECT add_retention_policy('trades', INTERVAL '90 days');

-- Index on symbol for faster lookups by symbol
CREATE INDEX idx_trades_symbol ON trades (symbol);

-- Index on exchange for faster lookups by exchange
CREATE INDEX idx_trades_exchange ON trades (exchange);

-- Index on security_id for faster lookups by security_id
CREATE INDEX idx_trades_security_id ON trades (security_id);

-- Index on exchange_id for faster lookups by exchange_id
CREATE INDEX idx_trades_exchange_id ON trades (exchange_id);

-- Index on side for faster lookups by side
CREATE INDEX idx_trades_side ON trades (side);

-- Composite index on frequently queried combinations
CREATE INDEX idx_trades_symbol_exchange ON trades (symbol, exchange);