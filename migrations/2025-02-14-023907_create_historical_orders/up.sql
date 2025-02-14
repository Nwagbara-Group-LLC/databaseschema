CREATE TABLE historical_orders (
    event_id UUID NOT NULL PRIMARY KEY DEFAULT gen_random_uuid(),
    timestamp TIMESTAMPTZ NOT NULL DEFAULT now(),
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
    UNIQUE (timestamp, order_id, event_type)
);

CREATE INDEX idx_historical_orders_timestamp ON historical_orders(timestamp);
CREATE INDEX idx_historical_orders_order_id ON historical_orders(order_id);
CREATE INDEX idx_historical_orders_symbol_exchange ON historical_orders(symbol, exchange);
