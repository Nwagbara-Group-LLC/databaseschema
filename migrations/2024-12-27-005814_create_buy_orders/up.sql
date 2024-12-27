-- Your SQL goes here
CREATE TABLE IF NOT EXISTS open_buy_orders (
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    symbol VARCHAR(7) NOT NULL REFERENCES order_books (symbol),
    exchange VARCHAR(8) NOT NULL REFERENCES order_books (exchange),
    security_id UUID NOT NULL REFERENCES order_books (security_id),
    exchange_id UUID NOT NULL REFERENCES order_books (exchange_id),
    buy_order_book_id UUID NOT NULL REFERENCES order_books (buy_order_book_id),
    unique_id VARCHAR(255) NOT NULL,
    price_level NUMERIC NOT NULL,
    buy_quantity NUMERIC NOT NULL,
    PRIMARY KEY (created_at, unique_id)
);

SELECT create_hypertable('open_buy_orders', 'created_at', chunk_time_interval => interval '1 millisecond');

ALTER TABLE open_buy_orders SET (
    timescaledb.compress,
    timescaledb.compress_segmentby = 'buy_order_book_id, symbol, exchange, security_id, exchange_id',
    timescaledb.compress_orderby = 'price_level DESC'
);

SELECT add_compression_policy('open_buy_orders', INTERVAL '7 days');
SELECT add_retention_policy('open_buy_orders', INTERVAL '6 years');

CREATE INDEX idx_open_buy_unique_id ON open_buy_orders (unique_id);