-- Your SQL goes here
CREATE TABLE historical_snapshot (
    event_id UUID NOT NULL PRIMARY KEY DEFAULT gen_random_uuid(),
    timestamp TIMESTAMPTZ NOT NULL,
    order_id TEXT NOT NULL,
    event_type TEXT NOT NULL CHECK (event_type IN ('new', 'trade')),
    side TEXT NOT NULL CHECK (side IN ('buy', 'sell')),
    price_level NUMERIC NOT NULL,
    quantity NUMERIC NOT NULL,
    status TEXT NOT NULL CHECK (status IN ('open', 'filled', 'canceled')),
    exchange TEXT NOT NULL,
    symbol TEXT NOT NULL,
    exchange_id UUID NOT NULL REFERENCES exchanges (exchange_id),
    security_id UUID NOT NULL REFERENCES securities (security_id),
    UNIQUE (timestamp, order_id, event_type)
);

CREATE INDEX idx_historical_snapshot_timestamp ON historical_snapshot(timestamp);
CREATE INDEX idx_historical_snapshot_order_id ON historical_snapshot(order_id);
CREATE INDEX idx_historical_snapshot_symbol_exchange ON historical_snapshot(symbol, exchange);
