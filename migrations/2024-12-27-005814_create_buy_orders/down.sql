-- This file should undo anything in `up.sql`
-- Drop the table
DROP TABLE IF EXISTS open_buy_orders CASCADE;

-- Drop the materialized view
DROP MATERIALIZED VIEW IF EXISTS open_buy_candlestick_agg CASCADE;