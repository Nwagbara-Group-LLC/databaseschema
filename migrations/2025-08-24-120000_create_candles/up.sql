-- Create candles table for storing OHLCV data
CREATE TABLE candles (
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
    
    -- Primary key for hypertable
    PRIMARY KEY (timestamp, symbol, timeframe),
    
    -- Basic constraints
    CONSTRAINT positive_prices CHECK (open_price > 0 AND high_price > 0 AND low_price > 0 AND close_price > 0),
    CONSTRAINT non_negative_volume CHECK (volume >= 0)
);

-- Convert to TimescaleDB hypertable for time-series optimization
SELECT create_hypertable('candles', 'timestamp', chunk_time_interval => interval '1 day');

-- Enable compression for OHLCV data
ALTER TABLE candles SET (
    timescaledb.compress,
    timescaledb.compress_segmentby = 'symbol, timeframe, exchange',
    timescaledb.compress_orderby = 'timestamp DESC'
);

-- Add compression policy (compress after 7 days for frequent access)
SELECT add_compression_policy('candles', INTERVAL '7 days');

-- Add retention policy (keep candle data for 2 years)
SELECT add_retention_policy('candles', INTERVAL '2 years');

-- Optimized indexes for compressed time-series data
CREATE INDEX idx_candles_symbol_timeframe ON candles (symbol, timeframe, timestamp DESC);
CREATE INDEX idx_candles_exchange ON candles (exchange, timestamp DESC);
CREATE INDEX idx_candles_timestamp ON candles (timestamp DESC);
CREATE INDEX idx_candles_volume ON candles (volume DESC, timestamp DESC);
