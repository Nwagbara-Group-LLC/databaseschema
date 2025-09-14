CREATE TABLE historical_orders (
    event_id UUID NOT NULL DEFAULT gen_random_uuid(),
    timestamp TIMESTAMPTZ NOT NULL,
    order_id TEXT NOT NULL,
    event_type TEXT NOT NULL CHECK (event_type IN ('new', 'modify', 'cancel', 'trade')),
    side TEXT NOT NULL CHECK (side IN ('buy', 'sell')),
    price_level NUMERIC NOT NULL,
    quantity NUMERIC NOT NULL,
    prev_price NUMERIC,
    prev_quantity NUMERIC,
    status TEXT NOT NULL CHECK (status IN ('open', 'partially_filled', 'filled', 'canceled')),
    exchange TEXT NOT NULL,
    symbol TEXT NOT NULL,
    exchange_id UUID NOT NULL REFERENCES exchanges (exchange_id),
    security_id UUID NOT NULL REFERENCES securities (security_id),
    PRIMARY KEY (timestamp, event_id),
    UNIQUE (timestamp, order_id, event_type)
);

-- Convert to TimescaleDB hypertable for time-series optimization
SELECT create_hypertable('historical_orders', 'timestamp', chunk_time_interval => interval '1 hour');

-- Enable compression with aggressive settings for high-frequency data
ALTER TABLE historical_orders SET (
    timescaledb.compress,
    timescaledb.compress_segmentby = 'symbol, exchange',
    timescaledb.compress_orderby = 'timestamp DESC, event_id'
);

-- Add aggressive compression policy (compress after 1 day)
SELECT add_compression_policy('historical_orders', INTERVAL '1 day');

-- Add retention policy (keep data for 6 months)
SELECT add_retention_policy('historical_orders', INTERVAL '6 months');

-- Optimized indexes for compressed data
CREATE INDEX idx_historical_orders_timestamp ON historical_orders(timestamp DESC);
CREATE INDEX idx_historical_orders_order_id ON historical_orders(order_id);
CREATE INDEX idx_historical_orders_symbol_exchange ON historical_orders(symbol, exchange, timestamp DESC);
CREATE INDEX idx_historical_orders_event_type ON historical_orders(event_type, timestamp DESC);
