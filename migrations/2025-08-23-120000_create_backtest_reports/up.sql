-- Migration to create backtest_reports table
-- This migration creates tables for storing generated reports and their metadata

CREATE TABLE backtest_reports (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    backtest_result_id UUID NOT NULL REFERENCES backtest_results(id) ON DELETE CASCADE,
    
    -- Report identification
    report_id VARCHAR(255) NOT NULL UNIQUE, -- The report_id from BacktestReport struct
    report_name VARCHAR(255) NOT NULL,
    
    -- Report metadata
    strategy_name VARCHAR(255) NOT NULL,
    symbol VARCHAR(50) NOT NULL,
    timeframe VARCHAR(20) NOT NULL,
    start_date DATE NOT NULL,
    end_date DATE NOT NULL,
    initial_capital DECIMAL(20, 8) NOT NULL,
    
    -- Generation metadata
    generated_at TIMESTAMP WITH TIME ZONE NOT NULL,
    generated_by VARCHAR(255), -- user/system that generated the report
    generation_source VARCHAR(50) NOT NULL, -- 'CLI', 'API', 'Scheduled', 'Manual'
    backtest_duration_seconds DECIMAL(10, 3),
    data_points INTEGER,
    
    -- Report configuration
    include_trades BOOLEAN NOT NULL DEFAULT false,
    include_charts BOOLEAN NOT NULL DEFAULT false,
    export_formats TEXT[] NOT NULL, -- Array of formats: ['html', 'json', 'csv']
    custom_css TEXT, -- Custom CSS used
    template_version VARCHAR(50), -- Template version used
    
    -- File storage information
    file_paths JSONB NOT NULL, -- Map of format -> file path
    file_sizes JSONB, -- Map of format -> file size in bytes
    storage_location VARCHAR(50) NOT NULL DEFAULT 'local', -- 'local', 's3', 'azure', etc.
    
    -- Report summary (for quick access without file reading)
    performance_summary JSONB NOT NULL, -- Key metrics for quick display
    risk_summary JSONB NOT NULL, -- Risk metrics summary
    trade_summary JSONB NOT NULL, -- Trade statistics summary
    
    -- Status and metadata
    status VARCHAR(20) NOT NULL DEFAULT 'generated', -- 'generating', 'generated', 'failed', 'archived'
    error_message TEXT, -- If status is 'failed'
    tags TEXT[], -- User-defined tags for categorization
    notes TEXT, -- User notes about the report
    
    -- Audit fields
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    accessed_at TIMESTAMP WITH TIME ZONE, -- Last access time
    access_count INTEGER NOT NULL DEFAULT 0,
    
    -- Constraints
    CONSTRAINT positive_initial_capital CHECK (initial_capital > 0),
    CONSTRAINT valid_date_range CHECK (end_date >= start_date),
    CONSTRAINT valid_status CHECK (status IN ('generating', 'generated', 'failed', 'archived')),
    CONSTRAINT valid_storage_location CHECK (storage_location IN ('local', 's3', 'azure', 'gcs'))
);

-- Report access log for detailed tracking
CREATE TABLE backtest_report_access_log (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    report_id UUID NOT NULL REFERENCES backtest_reports(id) ON DELETE CASCADE,
    accessed_by VARCHAR(255), -- user who accessed the report
    access_method VARCHAR(50) NOT NULL, -- 'download', 'view', 'api', 'dashboard'
    format_requested VARCHAR(20), -- 'html', 'json', 'csv', 'pdf'
    user_agent TEXT, -- Browser/client information
    ip_address TEXT, -- Client IP address as text
    response_time_ms INTEGER, -- Time to serve the report
    success BOOLEAN NOT NULL DEFAULT true,
    error_message TEXT, -- If not successful
    accessed_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

-- Create indexes for common queries
CREATE INDEX idx_backtest_reports_backtest_result_id ON backtest_reports(backtest_result_id);
CREATE INDEX idx_backtest_reports_report_id ON backtest_reports(report_id);
CREATE INDEX idx_backtest_reports_strategy_name ON backtest_reports(strategy_name);
CREATE INDEX idx_backtest_reports_symbol ON backtest_reports(symbol);
CREATE INDEX idx_backtest_reports_generated_at ON backtest_reports(generated_at);
CREATE INDEX idx_backtest_reports_generated_by ON backtest_reports(generated_by);
CREATE INDEX idx_backtest_reports_status ON backtest_reports(status);
CREATE INDEX idx_backtest_reports_tags ON backtest_reports USING GIN(tags);
CREATE INDEX idx_backtest_reports_export_formats ON backtest_reports USING GIN(export_formats);

-- Indexes for access log
CREATE INDEX idx_report_access_log_report_id ON backtest_report_access_log(report_id);
CREATE INDEX idx_report_access_log_accessed_by ON backtest_report_access_log(accessed_by);
CREATE INDEX idx_report_access_log_accessed_at ON backtest_report_access_log(accessed_at);
CREATE INDEX idx_report_access_log_access_method ON backtest_report_access_log(access_method);

-- Update trigger for backtest_reports
CREATE OR REPLACE FUNCTION update_backtest_reports_updated_at()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ language 'plpgsql';

CREATE TRIGGER update_backtest_reports_updated_at 
    BEFORE UPDATE ON backtest_reports
    FOR EACH ROW 
    EXECUTE FUNCTION update_backtest_reports_updated_at();

-- Trigger to update access tracking
CREATE OR REPLACE FUNCTION update_report_access()
RETURNS TRIGGER AS $$
BEGIN
    NEW.accessed_at = NOW();
    NEW.access_count = NEW.access_count + 1;
    RETURN NEW;
END;
$$ language 'plpgsql';

-- Function to clean up old reports (optional - for maintenance)
CREATE OR REPLACE FUNCTION cleanup_old_reports(days_old INTEGER DEFAULT 365)
RETURNS INTEGER AS $$
DECLARE
    deleted_count INTEGER;
BEGIN
    DELETE FROM backtest_reports 
    WHERE status = 'archived' 
    AND created_at < NOW() - INTERVAL '1 day' * days_old;
    
    GET DIAGNOSTICS deleted_count = ROW_COUNT;
    RETURN deleted_count;
END;
$$ language 'plpgsql';

-- Create a view for report summaries (frequently used data)
CREATE VIEW backtest_report_summary AS
SELECT 
    r.id,
    r.report_id,
    r.report_name,
    r.strategy_name,
    r.symbol,
    r.timeframe,
    r.generated_at,
    r.generated_by,
    r.export_formats,
    r.status,
    r.access_count,
    r.accessed_at,
    (r.performance_summary->>'net_profit')::DECIMAL as net_profit,
    (r.performance_summary->>'sharpe_ratio')::DECIMAL as sharpe_ratio,
    (r.performance_summary->>'max_drawdown')::DECIMAL as max_drawdown,
    (r.trade_summary->>'total_trades')::INTEGER as total_trades,
    br.total_return as backtest_total_return,
    br.created_at as backtest_created_at
FROM backtest_reports r
JOIN backtest_results br ON r.backtest_result_id = br.id;

-- Comments for documentation
COMMENT ON TABLE backtest_reports IS 'Stores metadata and references for generated backtest reports';
COMMENT ON TABLE backtest_report_access_log IS 'Tracks access patterns and usage of generated reports';
COMMENT ON COLUMN backtest_reports.file_paths IS 'JSONB map of report format to file path, e.g. {"html": "/path/to/report.html", "json": "/path/to/report.json"}';
COMMENT ON COLUMN backtest_reports.performance_summary IS 'Key performance metrics extracted for quick display without file access';
COMMENT ON COLUMN backtest_reports.storage_location IS 'Where report files are stored: local filesystem, cloud storage, etc.';
COMMENT ON FUNCTION cleanup_old_reports IS 'Maintenance function to remove archived reports older than specified days';
