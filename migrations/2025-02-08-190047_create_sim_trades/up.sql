-- Your SQL goes here
CREATE TABLE IF NOT EXISTS sim_trades (
    backtest_id UUID NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    symbol VARCHAR(7) NOT NULL REFERENCES securities (symbol),
    exchange VARCHAR(8) NOT NULL REFERENCES exchanges (exchange),
    trade_id TEXT DEFAULT NOT NULL,
    side VARCHAR(4) NOT NULL,
    price NUMERIC NOT NULL,
    quantity NUMERIC NOT NULL,
    matched_trader BOOLEAN NOT NULL DEFAULT FALSE,
    PRIMARY KEY (created_at, backtest_id, trade_id)
);

SELECT create_hypertable('sim_trades', 'created_at', chunk_time_interval => interval '1 millisecond');

ALTER TABLE sim_trades SET (
    timescaledb.compress,
    timescaledb.compress_segmentby = 'symbol, exchange',
    timescaledb.compress_orderby = 'created_at DESC'
);

SELECT add_compression_policy('sim_trades', INTERVAL '7 days');
SELECT add_retention_policy('sim_trades', INTERVAL '6 years');

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