-- Your SQL goes here
CREATE TABLE IF NOT EXISTS sim_open_sell_orders (
    backtest_id UUID NOT NULL,
    created_at TIMESTAMPTZ DEFAULT now(),
    symbol VARCHAR(7) NOT NULL REFERENCES securities (symbol),
    exchange VARCHAR(8) NOT NULL REFERENCES exchanges (exchange),
    unique_id VARCHAR(255) NOT NULL,
    price_level NUMERIC NOT NULL,
    sell_quantity NUMERIC NOT NULL,
    created_id UUID,
    PRIMARY KEY (created_at, unique_id)
);

SELECT create_hypertable('sim_open_sell_orders', 'created_at', chunk_time_interval => interval '1 millisecond');

ALTER TABLE sim_open_sell_orders SET (
    timescaledb.compress,
    timescaledb.compress_segmentby = 'symbol, exchange',
    timescaledb.compress_orderby = 'price_level ASC'
);

SELECT add_compression_policy('sim_open_sell_orders', INTERVAL '7 days');
SELECT add_retention_policy('sim_open_sell_orders', INTERVAL '6 years');

-- Index on backtest_id for faster lookups by backtest_id
CREATE INDEX idx_sim_open_sell_backtest_id ON sim_open_sell_orders (backtest_id);

-- Index on unique_id for faster lookups by unique_id
CREATE INDEX idx_sim_open_sell_unique_id ON sim_open_sell_orders (unique_id);

-- Index on symbol for faster lookups by symbol
CREATE INDEX idx_sim_open_sell_symbol ON sim_open_sell_orders (symbol);

-- Index on exchange for faster lookups by exchange
CREATE INDEX idx_sim_open_sell_exchange ON sim_open_sell_orders (exchange);

-- Composite index on frequently queried combinations
CREATE INDEX idx_sim_open_sell_symbol_exchange ON sim_open_sell_orders (symbol, exchange);