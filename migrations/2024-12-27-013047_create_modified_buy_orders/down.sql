-- This file should undo anything in `up.sql`
-- Drop the table with CASCADE
DROP TABLE IF EXISTS modified_buy_orders CASCADE;

-- Drop the materialized view
DROP MATERIALIZED VIEW IF EXISTS modified_buy_candlestick_agg CASCADE;