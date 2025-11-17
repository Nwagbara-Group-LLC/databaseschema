-- Your SQL goes here
CREATE TABLE IF NOT EXISTS open_sell_orders (
    created_at TIMESTAMPTZ NOT NULL,
    symbol VARCHAR(7) NOT NULL REFERENCES securities (symbol),
    exchange VARCHAR(8) NOT NULL REFERENCES exchanges (exchange),
    security_id UUID NOT NULL REFERENCES securities (security_id),
    exchange_id UUID NOT NULL REFERENCES exchanges (exchange_id),
    sell_order_book_id UUID NOT NULL REFERENCES order_books (sell_order_book_id),
    unique_id VARCHAR(255) NOT NULL,
    price_level NUMERIC NOT NULL,
    sell_quantity NUMERIC NOT NULL,
    PRIMARY KEY (created_at, unique_id)
);

-- Create hypertable with 1-day chunks for efficient 1ms tick data storage
SELECT create_hypertable('open_sell_orders', 'created_at', chunk_time_interval => interval '1 day');

-- Enable compression with optimal settings for order book data
ALTER TABLE open_sell_orders SET (
    timescaledb.compress,
    timescaledb.compress_segmentby = 'symbol, exchange',
    timescaledb.compress_orderby = 'created_at DESC, price_level ASC, unique_id'
);

-- Compress data older than 1 hour
SELECT add_compression_policy('open_sell_orders', INTERVAL '1 hour');

-- Keep 3 months of data (90 days)
SELECT add_retention_policy('open_sell_orders', INTERVAL '90 days');

-- Index on unique_id for faster lookups by unique_id
CREATE INDEX idx_open_sell_unique_id ON open_sell_orders (unique_id);

-- Index on symbol for faster lookups by symbol
CREATE INDEX idx_open_sell_symbol ON open_sell_orders (symbol);

-- Index on exchange for faster lookups by exchange
CREATE INDEX idx_open_sell_exchange ON open_sell_orders (exchange);

-- Index on security_id for faster lookups by security_id
CREATE INDEX idx_open_sell_security_id ON open_sell_orders (security_id);

-- Index on exchange_id for faster lookups by exchange_id
CREATE INDEX idx_open_sell_exchange_id ON open_sell_orders (exchange_id);

-- Index on sell_order_book_id for faster lookups by sell_order_book_id
CREATE INDEX idx_open_sell_sell_order_book_id ON open_sell_orders (sell_order_book_id);

-- Composite index on frequently queried combinations
CREATE INDEX idx_open_sell_symbol_exchange ON open_sell_orders (symbol, exchange);