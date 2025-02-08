-- Your SQL goes here
CREATE TABLE IF NOT EXISTS sim_open_buy_orders (
    backtest_id UUID NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    symbol VARCHAR(7) NOT NULL REFERENCES securities (symbol),
    exchange VARCHAR(8) NOT NULL REFERENCES exchanges (exchange),
    security_id UUID NOT NULL REFERENCES securities (security_id),
    exchange_id UUID NOT NULL REFERENCES exchanges (exchange_id),
    buy_order_book_id UUID NOT NULL REFERENCES order_books (buy_order_book_id),
    unique_id VARCHAR(255) NOT NULL,
    price_level NUMERIC NOT NULL,
    buy_quantity NUMERIC NOT NULL,
    PRIMARY KEY (created_at, backtest_id, unique_id)
);

SELECT create_hypertable('sim_open_buy_orders', 'created_at', chunk_time_interval => interval '1 millisecond');

ALTER TABLE sim_open_buy_orders SET (
    timescaledb.compress,
    timescaledb.compress_segmentby = 'buy_order_book_id, symbol, exchange, security_id, exchange_id',
    timescaledb.compress_orderby = 'price_level DESC'
);

SELECT add_compression_policy('sim_open_buy_orders', INTERVAL '7 days');
SELECT add_retention_policy('sim_open_buy_orders', INTERVAL '6 years');

-- Index on backtest_id for faster lookups by backtest_id
CREATE INDEX idx_sim_open_buy_backtest_id ON sim_open_buy_orders (backtest_id);

-- Index on unique_id for faster lookups by unique_id
CREATE INDEX idx_sim_open_buy_unique_id ON sim_open_buy_orders (unique_id);

-- Index on symbol for faster lookups by symbol
CREATE INDEX idx_sim_open_buy_symbol ON sim_open_buy_orders (symbol);

-- Index on exchange for faster lookups by exchange
CREATE INDEX idx_sim_open_buy_exchange ON sim_open_buy_orders (exchange);

-- Index on security_id for faster lookups by security_id
CREATE INDEX idx_sim_open_buy_security_id ON sim_open_buy_orders (security_id);

-- Index on exchange_id for faster lookups by exchange_id
CREATE INDEX idx_sim_open_buy_exchange_id ON sim_open_buy_orders (exchange_id);

-- Index on buy_order_book_id for faster lookups by buy_order_book_id
CREATE INDEX idx_sim_open_buy_buy_order_book_id ON sim_open_buy_orders (buy_order_book_id);

-- Composite index on frequently queried combinations
CREATE INDEX idx_sim_open_buy_symbol_exchange ON sim_open_buy_orders (symbol, exchange);