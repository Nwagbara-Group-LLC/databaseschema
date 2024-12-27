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

CREATE INDEX idx_modified_sell_unique_id ON modified_sell_orders (unique_id);