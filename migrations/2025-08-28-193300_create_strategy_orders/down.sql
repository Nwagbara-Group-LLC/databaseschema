-- Drop strategy order tables
DROP VIEW IF EXISTS strategy_order_performance;

DROP TABLE IF EXISTS strategy_order_state_changes;
DROP TABLE IF EXISTS strategy_order_fills;
DROP TABLE IF EXISTS strategy_orders;

-- Drop custom types
DROP TYPE IF EXISTS execution_urgency;
DROP TYPE IF EXISTS time_in_force;
DROP TYPE IF EXISTS order_type;
DROP TYPE IF EXISTS order_side;
DROP TYPE IF EXISTS order_status;
