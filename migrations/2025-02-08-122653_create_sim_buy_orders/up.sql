-- Your SQL goes here
CREATE TABLE IF NOT EXISTS sim_open_buy_orders (
    backtest_id UUID NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    symbol VARCHAR(7) NOT NULL REFERENCES securities (symbol),
    exchange VARCHAR(8) NOT NULL REFERENCES exchanges (exchange),
    unique_id VARCHAR(255) NOT NULL,
    price_level NUMERIC NOT NULL,
    buy_quantity NUMERIC NOT NULL,
    created_id UUID,
    PRIMARY KEY (created_at, backtest_id, unique_id)
);

-- Create hypertable with 1-day chunks for efficient backtest data storage
SELECT create_hypertable('sim_open_buy_orders', 'created_at', chunk_time_interval => interval '1 day');

-- Enable compression with optimal settings
ALTER TABLE sim_open_buy_orders SET (
    timescaledb.compress,
    timescaledb.compress_segmentby = 'backtest_id, symbol, exchange',
    timescaledb.compress_orderby = 'created_at DESC, price_level DESC, unique_id'
);

-- Compress data older than 1 day (backtests typically complete quickly)
SELECT add_compression_policy('sim_open_buy_orders', INTERVAL '1 day');

-- Keep backtest data for 90 days
SELECT add_retention_policy('sim_open_buy_orders', INTERVAL '90 days');

-- Index on backtest_id for faster lookups by backtest_id
CREATE INDEX idx_sim_open_buy_backtest_id ON sim_open_buy_orders (backtest_id);

-- Index on unique_id for faster lookups by unique_id
CREATE INDEX idx_sim_open_buy_unique_id ON sim_open_buy_orders (unique_id);

-- Index on symbol for faster lookups by symbol
CREATE INDEX idx_sim_open_buy_symbol ON sim_open_buy_orders (symbol);

-- Index on exchange for faster lookups by exchange
CREATE INDEX idx_sim_open_buy_exchange ON sim_open_buy_orders (exchange);

-- Composite index on frequently queried combinations
CREATE INDEX idx_sim_open_buy_symbol_exchange ON sim_open_buy_orders (symbol, exchange);