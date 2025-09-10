-- Migration to create related backtest tables
-- This migration creates tables for trades, equity curve, position history, and drawdown periods

-- Backtest trades table
CREATE TABLE backtest_trades (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    backtest_result_id UUID NOT NULL REFERENCES backtest_results(id) ON DELETE CASCADE,
    trade_id UUID NOT NULL, -- The ID from Trade struct
    order_id UUID NOT NULL,
    symbol VARCHAR(50) NOT NULL,
    side VARCHAR(10) NOT NULL, -- 'Buy', 'Sell'
    quantity DECIMAL(20, 8) NOT NULL,
    price DECIMAL(20, 8) NOT NULL,
    commission DECIMAL(20, 8) NOT NULL DEFAULT 0.0,
    timestamp TIMESTAMP WITH TIME ZONE NOT NULL,
    metadata JSONB, -- For additional trade metadata
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    
    CONSTRAINT positive_quantity CHECK (quantity > 0),
    CONSTRAINT positive_price CHECK (price > 0),
    CONSTRAINT non_negative_commission CHECK (commission >= 0)
);

-- Equity curve table (portfolio value over time)
CREATE TABLE backtest_equity_curve (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    backtest_result_id UUID NOT NULL REFERENCES backtest_results(id) ON DELETE CASCADE,
    timestamp TIMESTAMP WITH TIME ZONE NOT NULL,
    portfolio_value DECIMAL(20, 8) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    
    CONSTRAINT positive_portfolio_value CHECK (portfolio_value > 0),
    UNIQUE(backtest_result_id, timestamp)
);

-- Position history table
CREATE TABLE backtest_position_history (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    backtest_result_id UUID NOT NULL REFERENCES backtest_results(id) ON DELETE CASCADE,
    timestamp TIMESTAMP WITH TIME ZONE NOT NULL,
    symbol VARCHAR(50) NOT NULL,
    quantity DECIMAL(20, 8) NOT NULL,
    average_price DECIMAL(20, 8) NOT NULL,
    current_price DECIMAL(20, 8) NOT NULL,
    unrealized_pnl DECIMAL(20, 8) NOT NULL,
    direction VARCHAR(10) NOT NULL, -- 'Long', 'Short'
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    
    CONSTRAINT positive_average_price CHECK (average_price > 0),
    CONSTRAINT positive_current_price CHECK (current_price > 0),
    UNIQUE(backtest_result_id, timestamp, symbol)
);

-- Drawdown periods table
CREATE TABLE backtest_drawdown_periods (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    backtest_result_id UUID NOT NULL REFERENCES backtest_results(id) ON DELETE CASCADE,
    start_date TIMESTAMP WITH TIME ZONE NOT NULL,
    end_date TIMESTAMP WITH TIME ZONE NOT NULL,
    duration_days INTEGER NOT NULL,
    magnitude DECIMAL(20, 8) NOT NULL,
    recovery_date TIMESTAMP WITH TIME ZONE,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    
    CONSTRAINT positive_duration CHECK (duration_days > 0),
    CONSTRAINT valid_drawdown_dates CHECK (end_date >= start_date),
    CONSTRAINT recovery_after_end CHECK (recovery_date IS NULL OR recovery_date >= end_date)
);

-- Create indexes for performance
CREATE INDEX idx_backtest_trades_backtest_result_id ON backtest_trades(backtest_result_id);
CREATE INDEX idx_backtest_trades_symbol ON backtest_trades(symbol);
CREATE INDEX idx_backtest_trades_timestamp ON backtest_trades(timestamp);
CREATE INDEX idx_backtest_trades_side ON backtest_trades(side);

CREATE INDEX idx_backtest_equity_curve_backtest_result_id ON backtest_equity_curve(backtest_result_id);
CREATE INDEX idx_backtest_equity_curve_timestamp ON backtest_equity_curve(timestamp);

CREATE INDEX idx_backtest_position_history_backtest_result_id ON backtest_position_history(backtest_result_id);
CREATE INDEX idx_backtest_position_history_symbol ON backtest_position_history(symbol);
CREATE INDEX idx_backtest_position_history_timestamp ON backtest_position_history(timestamp);

CREATE INDEX idx_backtest_drawdown_periods_backtest_result_id ON backtest_drawdown_periods(backtest_result_id);
CREATE INDEX idx_backtest_drawdown_periods_start_date ON backtest_drawdown_periods(start_date);
CREATE INDEX idx_backtest_drawdown_periods_magnitude ON backtest_drawdown_periods(magnitude);

-- Create hypertables for time-series optimization (if using TimescaleDB)
-- SELECT create_hypertable('backtest_trades', 'timestamp', if_not_exists => TRUE);
-- SELECT create_hypertable('backtest_equity_curve', 'timestamp', if_not_exists => TRUE);
-- SELECT create_hypertable('backtest_position_history', 'timestamp', if_not_exists => TRUE);
