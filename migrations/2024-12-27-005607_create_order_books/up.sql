-- Your SQL goes here
CREATE TABLE IF NOT EXISTS order_books (
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ DEFAULT now(),
    symbol VARCHAR(7) UNIQUE NOT NULL REFERENCES securities (symbol),
    exchange VARCHAR(8) NOT NULL REFERENCES exchanges (exchange),
    security_id UUID UNIQUE NOT NULL REFERENCES securities (security_id),
    exchange_id UUID NOT NULL REFERENCES exchanges (exchange_id),
    order_book_id UUID PRIMARY KEY NOT NULL DEFAULT gen_random_uuid(),
    buy_order_book_id UUID UNIQUE NOT NULL DEFAULT gen_random_uuid(),
    sell_order_book_id UUID UNIQUE NOT NULL DEFAULT gen_random_uuid(),
    total_volume NUMERIC NOT NULL DEFAULT 0
);