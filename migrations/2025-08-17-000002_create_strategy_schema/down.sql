-- Drop strategy related tables in reverse order with CASCADE to handle dependencies

DROP TABLE IF EXISTS strategy_comparisons CASCADE;
DROP TABLE IF EXISTS optimization_iterations CASCADE;
DROP TABLE IF EXISTS optimization_runs CASCADE;
DROP TABLE IF EXISTS strategy_instances CASCADE;
DROP TABLE IF EXISTS strategy_parameters CASCADE;
DROP TABLE IF EXISTS strategies CASCADE;
