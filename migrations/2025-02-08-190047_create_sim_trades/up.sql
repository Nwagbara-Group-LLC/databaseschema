-- Your SQL goes here
CREATE TABLE IF NOT EXISTS sim_trades (
    backtest_id UUID NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    symbol VARCHAR(7) NOT NULL REFERENCES securities (symbol),
    exchange VARCHAR(8) NOT NULL REFERENCES exchanges (exchange),
    trade_id TEXT NOT NULL,
    side VARCHAR(4) NOT NULL,
    price NUMERIC NOT NULL,
    quantity NUMERIC NOT NULL,
    matched_trader BOOLEAN NOT NULL DEFAULT FALSE,
    PRIMARY KEY (created_at, backtest_id, trade_id)
);

-- Create hypertable with 1-day chunks for efficient backtest data storage
SELECT create_hypertable('sim_trades', 'created_at', chunk_time_interval => interval '1 day');

-- Enable compression with optimal settings
ALTER TABLE sim_trades SET (
    timescaledb.compress,
    timescaledb.compress_segmentby = 'backtest_id, symbol, exchange, side',
    timescaledb.compress_orderby = 'created_at DESC, trade_id'
);

-- Compress data older than 1 day (backtests typically complete quickly)
SELECT add_compression_policy('sim_trades', INTERVAL '1 day');

-- Keep backtest data for 90 days
SELECT add_retention_policy('sim_trades', INTERVAL '90 days');

-- Index on backtest_id for faster lookups by backtest_id
CREATE INDEX idx_sim_trades_backtest_id ON sim_trades (backtest_id);

-- Index on symbol for faster lookups by symbol
CREATE INDEX idx_sim_trades_symbol ON sim_trades (symbol);

-- Index on exchange for faster lookups by exchange
CREATE INDEX idx_sim_trades_exchange ON sim_trades (exchange);

-- Index on side for faster lookups by side
CREATE INDEX idx_sim_trades_side ON sim_trades (side);

-- Composite index on frequently queried combinations
CREATE INDEX idx_sim_trades_symbol_exchange ON sim_trades (symbol, exchange);