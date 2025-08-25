-- Create continuous aggregates for real-time candlestick data generation
-- This is much more efficient than storing pre-computed candles

-- 1-minute candlesticks using continuous aggregates
CREATE MATERIALIZED VIEW candles_1m
WITH (timescaledb.continuous) AS
SELECT 
    time_bucket('1 minute', created_at) AS timestamp,
    symbol,
    exchange,
    security_id,
    exchange_id,
    FIRST(price, created_at) AS open_price,
    MAX(price) AS high_price,
    MIN(price) AS low_price,
    LAST(price, created_at) AS close_price,
    SUM(quantity) AS volume,
    COUNT(*) AS trade_count
FROM trades
GROUP BY time_bucket('1 minute', created_at), symbol, exchange, security_id, exchange_id;

-- 5-minute candlesticks
CREATE MATERIALIZED VIEW candles_5m
WITH (timescaledb.continuous) AS
SELECT 
    time_bucket('5 minutes', created_at) AS timestamp,
    symbol,
    exchange,
    security_id,
    exchange_id,
    FIRST(price, created_at) AS open_price,
    MAX(price) AS high_price,
    MIN(price) AS low_price,
    LAST(price, created_at) AS close_price,
    SUM(quantity) AS volume,
    COUNT(*) AS trade_count
FROM trades
GROUP BY time_bucket('5 minutes', created_at), symbol, exchange, security_id, exchange_id;

-- 15-minute candlesticks
CREATE MATERIALIZED VIEW candles_15m
WITH (timescaledb.continuous) AS
SELECT 
    time_bucket('15 minutes', created_at) AS timestamp,
    symbol,
    exchange,
    security_id,
    exchange_id,
    FIRST(price, created_at) AS open_price,
    MAX(price) AS high_price,
    MIN(price) AS low_price,
    LAST(price, created_at) AS close_price,
    SUM(quantity) AS volume,
    COUNT(*) AS trade_count
FROM trades
GROUP BY time_bucket('15 minutes', created_at), symbol, exchange, security_id, exchange_id;

-- 1-hour candlesticks
CREATE MATERIALIZED VIEW candles_1h
WITH (timescaledb.continuous) AS
SELECT 
    time_bucket('1 hour', created_at) AS timestamp,
    symbol,
    exchange,
    security_id,
    exchange_id,
    FIRST(price, created_at) AS open_price,
    MAX(price) AS high_price,
    MIN(price) AS low_price,
    LAST(price, created_at) AS close_price,
    SUM(quantity) AS volume,
    COUNT(*) AS trade_count
FROM trades
GROUP BY time_bucket('1 hour', created_at), symbol, exchange, security_id, exchange_id;

-- 1-day candlesticks
CREATE MATERIALIZED VIEW candles_1d
WITH (timescaledb.continuous) AS
SELECT 
    time_bucket('1 day', created_at) AS timestamp,
    symbol,
    exchange,
    security_id,
    exchange_id,
    FIRST(price, created_at) AS open_price,
    MAX(price) AS high_price,
    MIN(price) AS low_price,
    LAST(price, created_at) AS close_price,
    SUM(quantity) AS volume,
    COUNT(*) AS trade_count
FROM trades
GROUP BY time_bucket('1 day', created_at), symbol, exchange, security_id, exchange_id;

-- Add refresh policies for real-time updates
SELECT add_continuous_aggregate_policy('candles_1m',
    start_offset => INTERVAL '1 hour',
    end_offset => INTERVAL '1 minute',
    schedule_interval => INTERVAL '1 minute');

SELECT add_continuous_aggregate_policy('candles_5m',
    start_offset => INTERVAL '2 hours',
    end_offset => INTERVAL '5 minutes',
    schedule_interval => INTERVAL '5 minutes');

SELECT add_continuous_aggregate_policy('candles_15m',
    start_offset => INTERVAL '6 hours',
    end_offset => INTERVAL '15 minutes',
    schedule_interval => INTERVAL '15 minutes');

SELECT add_continuous_aggregate_policy('candles_1h',
    start_offset => INTERVAL '12 hours',
    end_offset => INTERVAL '1 hour',
    schedule_interval => INTERVAL '1 hour');

SELECT add_continuous_aggregate_policy('candles_1d',
    start_offset => INTERVAL '3 days',
    end_offset => INTERVAL '1 day',
    schedule_interval => INTERVAL '1 day');

-- Add retention policies to manage data lifecycle
SELECT add_retention_policy('candles_1m', INTERVAL '7 days');
SELECT add_retention_policy('candles_5m', INTERVAL '30 days');
SELECT add_retention_policy('candles_15m', INTERVAL '90 days');
SELECT add_retention_policy('candles_1h', INTERVAL '1 year');
SELECT add_retention_policy('candles_1d', INTERVAL '5 years');

-- Create indexes for fast queries
CREATE INDEX idx_candles_1m_symbol_exchange ON candles_1m (symbol, exchange, timestamp DESC);
CREATE INDEX idx_candles_5m_symbol_exchange ON candles_5m (symbol, exchange, timestamp DESC);
CREATE INDEX idx_candles_15m_symbol_exchange ON candles_15m (symbol, exchange, timestamp DESC);
CREATE INDEX idx_candles_1h_symbol_exchange ON candles_1h (symbol, exchange, timestamp DESC);
CREATE INDEX idx_candles_1d_symbol_exchange ON candles_1d (symbol, exchange, timestamp DESC);
