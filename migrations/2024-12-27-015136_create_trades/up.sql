-- Your SQL goes here
CREATE TABLE IF NOT EXISTS trades (
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    symbol VARCHAR(7) NOT NULL REFERENCES order_books (symbol),
    exchange VARCHAR(8) NOT NULL REFERENCES order_books (exchange),
    trade_id UUID DEFAULT gen_random_uuid(),
    security_id UUID NOT NULL REFERENCES order_books (security_id),
    exchange_id UUID NOT NULL REFERENCES order_books (exchange_id),
    side VARCHAR(4) NOT NULL,
    price NUMERIC NOT NULL,
    quantity NUMERIC NOT NULL,
    PRIMARY KEY (created_at, trade_id)
);

SELECT create_hypertable('trades', 'created_at', chunk_time_interval => interval '1 millisecond');

ALTER TABLE trades SET (
    timescaledb.compress,
    timescaledb.compress_segmentby = 'symbol, exchange',
    timescaledb.compress_orderby = 'created_at DESC'
);

SELECT add_compression_policy('trades', INTERVAL '7 days');
SELECT add_retention_policy('trades', INTERVAL '6 years');
