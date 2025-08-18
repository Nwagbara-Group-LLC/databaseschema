-- Drop backtest results table and related objects

DROP TRIGGER IF EXISTS update_backtest_results_updated_at ON backtest_results;
DROP FUNCTION IF EXISTS update_backtest_results_updated_at();
DROP TABLE IF EXISTS backtest_results;
