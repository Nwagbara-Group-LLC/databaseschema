-- Down migration for backtest reports tables

DROP VIEW IF EXISTS backtest_report_summary;
DROP FUNCTION IF EXISTS cleanup_old_reports(INTEGER);
DROP TRIGGER IF EXISTS update_backtest_reports_updated_at ON backtest_reports;
DROP FUNCTION IF EXISTS update_backtest_reports_updated_at();
DROP FUNCTION IF EXISTS update_report_access();

DROP TABLE IF EXISTS backtest_report_access_log;
DROP TABLE IF EXISTS backtest_reports;
