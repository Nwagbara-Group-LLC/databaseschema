-- Your SQL goes here
CREATE TABLE IF NOT EXISTS modified_sell_orders (
    created_at TIMESTAMPTZ DEFAULT now(),
    symbol VARCHAR(7) NOT NULL REFERENCES order_books (symbol),
    exchange VARCHAR(8) NOT NULL REFERENCES order_books (exchange),
    security_id UUID NOT NULL REFERENCES order_books (security_id),
    exchange_id UUID NOT NULL REFERENCES order_books (exchange_id),
    sell_order_book_id UUID NOT NULL REFERENCES order_books (sell_order_book_id),
    unique_id VARCHAR(255) NOT NULL,
    price_level NUMERIC NOT NULL,
    new_sell_quantity NUMERIC NOT NULL,
    PRIMARY KEY (created_at, unique_id)
);

SELECT create_hypertable('modified_sell_orders', 'created_at', chunk_time_interval => interval '1 millisecond');

ALTER TABLE modified_sell_orders SET (
    timescaledb.compress,
    timescaledb.compress_segmentby = 'sell_order_book_id, symbol, exchange, security_id, exchange_id',
    timescaledb.compress_orderby = 'price_level ASC'
);

SELECT add_compression_policy('modified_sell_orders', INTERVAL '7 days');
SELECT add_retention_policy('modified_sell_orders', INTERVAL '6 years');

-- Index on unique_id for faster lookups by unique_id
CREATE INDEX idx_modified_sell_unique_id ON modified_sell_orders (unique_id);

-- Index on symbol for faster lookups by symbol
CREATE INDEX idx_modified_sell_symbol ON modified_sell_orders (symbol);

-- Index on exchange for faster lookups by exchange
CREATE INDEX idx_modified_sell_exchange ON modified_sell_orders (exchange);

-- Index on security_id for faster lookups by security_id
CREATE INDEX idx_modified_sell_security_id ON modified_sell_orders (security_id);

-- Index on exchange_id for faster lookups by exchange_id
CREATE INDEX idx_modified_sell_exchange_id ON modified_sell_orders (exchange_id);

-- Index on sell_order_book_id for faster lookups by sell_order_book_id
CREATE INDEX idx_modified_sell_sell_order_book_id ON modified_sell_orders (sell_order_book_id);

-- Composite index on frequently queried combinations
CREATE INDEX idx_modified_sell_symbol_exchange ON modified_sell_orders (symbol, exchange);