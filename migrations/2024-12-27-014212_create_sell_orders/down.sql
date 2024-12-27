-- This file should undo anything in `up.sql`
-- Drop the continuous aggregate materialized view
DROP MATERIALIZED VIEW IF EXISTS open_sell_candlestick_agg;

-- Drop the table with CASCADE
DROP TABLE IF EXISTS open_sell_orders CASCADE;