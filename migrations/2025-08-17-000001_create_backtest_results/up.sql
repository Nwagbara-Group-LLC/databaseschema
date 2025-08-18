-- Migration to create backtest_results table
-- This migration creates the main table for storing comprehensive backtest results

CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Main backtest results table
CREATE TABLE backtest_results (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    backtest_id UUID NOT NULL UNIQUE, -- The ID from BacktestResult struct
    strategy_name VARCHAR(255) NOT NULL,
    symbol VARCHAR(50) NOT NULL,
    
    -- Configuration
    start_date TIMESTAMP WITH TIME ZONE NOT NULL,
    end_date TIMESTAMP WITH TIME ZONE NOT NULL,
    initial_capital DECIMAL(20, 8) NOT NULL,
    commission_rate DECIMAL(10, 8) NOT NULL,
    
    -- Slippage model
    slippage_model_type VARCHAR(50) NOT NULL, -- 'None', 'Fixed', 'SqrtVolume', 'Linear'
    slippage_fixed_rate DECIMAL(10, 8),
    slippage_sqrt_rate DECIMAL(10, 8),
    slippage_linear_rate DECIMAL(10, 8),
    
    -- Market impact model
    temporary_impact DECIMAL(10, 8) NOT NULL,
    permanent_impact DECIMAL(10, 8) NOT NULL,
    participation_rate_limit DECIMAL(10, 8) NOT NULL,
    
    -- Configuration continued
    benchmark VARCHAR(50),
    rebalancing_frequency VARCHAR(50) NOT NULL, -- 'Daily', 'Weekly', 'Monthly', 'Quarterly', 'Never'
    point_in_time BOOLEAN NOT NULL DEFAULT true,
    warmup_period_days INTEGER NOT NULL DEFAULT 30,
    
    -- Performance metrics
    total_return DECIMAL(20, 8) NOT NULL,
    annualized_return DECIMAL(20, 8) NOT NULL,
    volatility DECIMAL(20, 8) NOT NULL,
    sharpe_ratio DECIMAL(20, 8),
    sortino_ratio DECIMAL(20, 8),
    max_drawdown DECIMAL(20, 8) NOT NULL,
    calmar_ratio DECIMAL(20, 8),
    win_rate DECIMAL(10, 8) NOT NULL,
    profit_factor DECIMAL(20, 8) NOT NULL,
    avg_trade_return DECIMAL(20, 8) NOT NULL,
    total_trades INTEGER NOT NULL DEFAULT 0,
    best_trade DECIMAL(20, 8),
    worst_trade DECIMAL(20, 8),
    avg_time_in_trade DECIMAL(10, 2), -- in days
    
    -- Risk metrics
    value_at_risk_95 DECIMAL(20, 8),
    expected_shortfall_95 DECIMAL(20, 8),
    beta DECIMAL(10, 6),
    correlation_with_benchmark DECIMAL(10, 6),
    tracking_error DECIMAL(20, 8),
    information_ratio DECIMAL(10, 6),
    jensen_alpha DECIMAL(10, 6),
    
    -- Drawdown analysis
    max_drawdown_duration_days INTEGER,
    current_drawdown DECIMAL(20, 8) NOT NULL DEFAULT 0.0,
    avg_drawdown DECIMAL(20, 8),
    
    -- Benchmark comparison (if available)
    benchmark_return DECIMAL(20, 8),
    excess_return DECIMAL(20, 8),
    outperformance_periods INTEGER,
    underperformance_periods INTEGER,
    
    -- Execution statistics
    total_orders INTEGER NOT NULL DEFAULT 0,
    filled_orders INTEGER NOT NULL DEFAULT 0,
    cancelled_orders INTEGER NOT NULL DEFAULT 0,
    avg_slippage DECIMAL(10, 8) NOT NULL DEFAULT 0.0,
    total_commission_paid DECIMAL(20, 8) NOT NULL DEFAULT 0.0,
    avg_fill_time_seconds DECIMAL(10, 3),
    
    -- Strategy-specific metrics (stored as JSONB for flexibility)
    strategy_metrics JSONB,
    
    -- Additional metadata
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    
    -- Indexes
    CONSTRAINT positive_initial_capital CHECK (initial_capital > 0),
    CONSTRAINT valid_commission_rate CHECK (commission_rate >= 0 AND commission_rate <= 1),
    CONSTRAINT valid_date_range CHECK (end_date > start_date)
);

-- Create indexes for common queries
CREATE INDEX idx_backtest_results_backtest_id ON backtest_results(backtest_id);
CREATE INDEX idx_backtest_results_strategy_name ON backtest_results(strategy_name);
CREATE INDEX idx_backtest_results_symbol ON backtest_results(symbol);
CREATE INDEX idx_backtest_results_created_at ON backtest_results(created_at);
CREATE INDEX idx_backtest_results_start_date ON backtest_results(start_date);
CREATE INDEX idx_backtest_results_end_date ON backtest_results(end_date);
CREATE INDEX idx_backtest_results_sharpe_ratio ON backtest_results(sharpe_ratio);
CREATE INDEX idx_backtest_results_total_return ON backtest_results(total_return);

-- Update trigger
CREATE OR REPLACE FUNCTION update_backtest_results_updated_at()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ language 'plpgsql';

CREATE TRIGGER update_backtest_results_updated_at 
    BEFORE UPDATE ON backtest_results
    FOR EACH ROW 
    EXECUTE FUNCTION update_backtest_results_updated_at();

-- Create hypertable for time-series optimization (if using TimescaleDB)
-- SELECT create_hypertable('backtest_results', 'created_at', if_not_exists => TRUE);
