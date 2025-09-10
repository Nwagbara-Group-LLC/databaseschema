-- Migration for Strategy Parameter Schema
-- This creates tables for managing trading strategies, their parameters, and configurations

-- Extension for better JSON handling
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Strategy definitions and metadata
CREATE TABLE strategies (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    strategy_name VARCHAR(255) NOT NULL,
    strategy_type VARCHAR(100) NOT NULL, -- 'AvellanedaStoikov', 'UltraFastMomentum', etc.
    version VARCHAR(50) NOT NULL DEFAULT '1.0.0',
    description TEXT,
    created_by VARCHAR(255),
    is_active BOOLEAN NOT NULL DEFAULT true,
    base_configuration JSONB, -- Default parameter set for this strategy
    metadata JSONB, -- Additional strategy metadata
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    UNIQUE(strategy_name, version),
    CONSTRAINT strategies_name_check CHECK (char_length(strategy_name) >= 3),
    CONSTRAINT strategies_type_check CHECK (char_length(strategy_type) >= 3)
);

-- Strategy parameter definitions with validation rules
CREATE TABLE strategy_parameters (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    strategy_id UUID NOT NULL REFERENCES strategies(id) ON DELETE CASCADE,
    parameter_name VARCHAR(255) NOT NULL,
    parameter_type VARCHAR(50) NOT NULL CHECK (parameter_type IN ('float', 'integer', 'string', 'boolean', 'array', 'object')),
    is_required BOOLEAN NOT NULL DEFAULT true,
    default_value JSONB,
    
    -- Validation constraints
    min_value NUMERIC,
    max_value NUMERIC,
    allowed_values JSONB, -- Array of allowed values for enum-like parameters
    validation_pattern VARCHAR(500), -- Regex pattern for string validation
    
    -- Parameter metadata
    display_name VARCHAR(255),
    description TEXT,
    parameter_group VARCHAR(100), -- Group parameters for UI: 'risk', 'sizing', 'timing', etc.
    display_order INTEGER,
    
    -- Optimization settings
    is_optimizable BOOLEAN NOT NULL DEFAULT false,
    optimization_min NUMERIC,
    optimization_max NUMERIC,
    optimization_step NUMERIC,
    
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    UNIQUE(strategy_id, parameter_name),
    CONSTRAINT param_min_max_check CHECK (
        min_value IS NULL OR max_value IS NULL OR min_value <= max_value
    ),
    CONSTRAINT opt_min_max_check CHECK (
        optimization_min IS NULL OR optimization_max IS NULL OR optimization_min <= optimization_max
    )
);

-- Strategy instances with specific parameter configurations
CREATE TABLE strategy_instances (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    strategy_id UUID NOT NULL REFERENCES strategies(id) ON DELETE CASCADE,
    instance_name VARCHAR(255),
    description TEXT,
    
    -- Parameter configuration for this instance
    parameters JSONB NOT NULL,
    
    -- Performance summary (cached from backtest results)
    performance_summary JSONB,
    risk_metrics JSONB,
    
    -- Instance metadata
    is_template BOOLEAN NOT NULL DEFAULT false, -- Mark as reusable template
    tags TEXT[], -- Tags for categorization (TEXT[] instead of VARCHAR[] for Diesel compatibility)
    created_by VARCHAR(255),
    
    -- Optimization tracking
    optimization_run_id UUID, -- Link to optimization run that created this
    optimization_score NUMERIC, -- Fitness score if created via optimization
    
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    CONSTRAINT instance_name_check CHECK (char_length(instance_name) >= 3)
);

-- Parameter optimization runs and history
CREATE TABLE optimization_runs (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    strategy_id UUID NOT NULL REFERENCES strategies(id) ON DELETE CASCADE,
    run_name VARCHAR(255) NOT NULL,
    
    -- Optimization configuration
    optimization_method VARCHAR(100) NOT NULL, -- 'genetic', 'bayesian', 'grid_search', etc.
    objective_function VARCHAR(100) NOT NULL, -- 'maximize_sharpe', 'minimize_drawdown', etc.
    optimization_config JSONB, -- Method-specific configuration
    
    -- Parameter space definition
    parameter_ranges JSONB NOT NULL, -- Define min/max/step for each parameter
    constraints JSONB, -- Additional constraints for optimization
    
    -- Run status and results
    status VARCHAR(50) NOT NULL DEFAULT 'pending', -- 'running', 'completed', 'failed'
    total_iterations INTEGER,
    completed_iterations INTEGER DEFAULT 0,
    best_score NUMERIC,
    best_parameters JSONB,
    
    -- Execution metadata
    started_at TIMESTAMPTZ,
    completed_at TIMESTAMPTZ,
    error_message TEXT,
    created_by VARCHAR(255),
    
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    CONSTRAINT opt_run_name_check CHECK (char_length(run_name) >= 3)
);

-- Individual optimization iterations/trials
CREATE TABLE optimization_iterations (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    optimization_run_id UUID NOT NULL REFERENCES optimization_runs(id) ON DELETE CASCADE,
    iteration_number INTEGER NOT NULL,
    
    -- Trial parameters and results
    parameters JSONB NOT NULL,
    objective_score NUMERIC,
    additional_metrics JSONB,
    
    -- Execution details
    started_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    completed_at TIMESTAMPTZ,
    execution_time_ms INTEGER,
    status VARCHAR(50) NOT NULL DEFAULT 'pending',
    error_message TEXT
);

-- Strategy performance comparisons
CREATE TABLE strategy_comparisons (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    comparison_name VARCHAR(255) NOT NULL,
    description TEXT,
    
    -- Comparison configuration
    strategies JSONB NOT NULL, -- Array of strategy_instance_ids being compared
    comparison_period JSONB, -- Start/end dates for comparison
    benchmark_symbol VARCHAR(20),
    
    -- Comparison results
    results JSONB, -- Detailed comparison results
    summary JSONB, -- Key findings and rankings
    
    created_by VARCHAR(255),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    CONSTRAINT comparison_name_check CHECK (char_length(comparison_name) >= 3)
);

-- Link backtest results to strategy instances
ALTER TABLE backtest_results 
ADD COLUMN strategy_instance_id UUID REFERENCES strategy_instances(id);

-- Create indexes for performance
CREATE INDEX idx_strategies_type ON strategies(strategy_type);
CREATE INDEX idx_strategies_active ON strategies(is_active) WHERE is_active = true;
CREATE INDEX idx_strategies_created_at ON strategies(created_at);

CREATE INDEX idx_strategy_parameters_strategy_id ON strategy_parameters(strategy_id);
CREATE INDEX idx_strategy_parameters_optimizable ON strategy_parameters(strategy_id, is_optimizable) WHERE is_optimizable = true;
CREATE INDEX idx_strategy_parameters_group ON strategy_parameters(parameter_group);

CREATE INDEX idx_strategy_instances_strategy_id ON strategy_instances(strategy_id);
CREATE INDEX idx_strategy_instances_template ON strategy_instances(is_template) WHERE is_template = true;
CREATE INDEX idx_strategy_instances_created_at ON strategy_instances(created_at);
CREATE INDEX idx_strategy_instances_optimization ON strategy_instances(optimization_run_id) WHERE optimization_run_id IS NOT NULL;

CREATE INDEX idx_optimization_runs_strategy_id ON optimization_runs(strategy_id);
CREATE INDEX idx_optimization_runs_status ON optimization_runs(status);
CREATE INDEX idx_optimization_runs_created_at ON optimization_runs(created_at);

CREATE INDEX idx_backtest_results_strategy_instance ON backtest_results(strategy_instance_id) WHERE strategy_instance_id IS NOT NULL;

-- Create indexes for optimization iterations
CREATE INDEX idx_optimization_iterations_run_id ON optimization_iterations(optimization_run_id);
CREATE INDEX idx_optimization_iterations_score ON optimization_iterations(objective_score) WHERE objective_score IS NOT NULL;

-- Note: TimescaleDB hypertable creation commented out to avoid index conflicts
-- SELECT create_hypertable('optimization_iterations', 'started_at', chunk_time_interval => INTERVAL '7 days');

-- Comments for documentation
COMMENT ON TABLE strategies IS 'Master table for strategy definitions and versions';
COMMENT ON TABLE strategy_parameters IS 'Parameter definitions and validation rules for each strategy';
COMMENT ON TABLE strategy_instances IS 'Specific parameter configurations of strategies';
COMMENT ON TABLE optimization_runs IS 'Parameter optimization runs and their configuration';
COMMENT ON TABLE optimization_iterations IS 'Individual trials within optimization runs';
COMMENT ON TABLE strategy_comparisons IS 'Results of strategy performance comparisons';

COMMENT ON COLUMN strategies.base_configuration IS 'Default JSONB parameter set for this strategy type';
COMMENT ON COLUMN strategy_parameters.parameter_type IS 'Data type: float, integer, string, boolean, array, object';
COMMENT ON COLUMN strategy_parameters.parameter_group IS 'UI grouping: risk, sizing, timing, volatility, etc.';
COMMENT ON COLUMN strategy_instances.parameters IS 'JSONB object containing all parameter values for this instance';
COMMENT ON COLUMN strategy_instances.performance_summary IS 'Cached performance metrics from backtests';
