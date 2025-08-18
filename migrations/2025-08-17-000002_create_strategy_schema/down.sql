-- Rollback migration for Strategy Parameter Schema

-- Remove column from backtest_results
ALTER TABLE backtest_results DROP COLUMN IF EXISTS strategy_instance_id;

-- Drop tables in reverse dependency order
DROP TABLE IF EXISTS strategy_comparisons;
DROP TABLE IF EXISTS optimization_iterations;
DROP TABLE IF EXISTS optimization_runs;
DROP TABLE IF EXISTS strategy_instances;
DROP TABLE IF EXISTS strategy_parameters;
DROP TABLE IF EXISTS strategies;
