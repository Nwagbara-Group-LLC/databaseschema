-- Create candles table for storing OHLCV data
CREATE TABLE candles (
    id UUID DEFAULT uuid_generate_v4() PRIMARY KEY,
    timestamp TIMESTAMPTZ NOT NULL,
    symbol VARCHAR(20) NOT NULL,
    exchange VARCHAR(50) NOT NULL,
    security_id UUID NOT NULL REFERENCES securities(security_id),
    exchange_id UUID NOT NULL REFERENCES exchanges(exchange_id),
    
    -- OHLCV data
    open_price NUMERIC(20, 8) NOT NULL,
    high_price NUMERIC(20, 8) NOT NULL,
    low_price NUMERIC(20, 8) NOT NULL,
    close_price NUMERIC(20, 8) NOT NULL,
    volume NUMERIC(20, 8) NOT NULL DEFAULT 0,
    trade_count INTEGER NOT NULL DEFAULT 0,
    
    -- Timeframe identifier
    timeframe VARCHAR(10) NOT NULL,
    
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    -- Basic constraints
    CONSTRAINT positive_prices CHECK (open_price > 0 AND high_price > 0 AND low_price > 0 AND close_price > 0),
    CONSTRAINT non_negative_volume CHECK (volume >= 0)
);

-- Basic indexes
CREATE INDEX idx_candles_symbol_timeframe ON candles (symbol, timeframe, timestamp DESC);
CREATE INDEX idx_candles_exchange ON candles (exchange, timestamp DESC);
CREATE INDEX idx_candles_timestamp ON candles (timestamp DESC);
