use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use uuid::Uuid;
use bigdecimal::BigDecimal;

use crate::schema::{
    strategies, strategy_parameters, strategy_instances, 
    optimization_runs, optimization_iterations, strategy_comparisons
};

/// Master table for strategy definitions and versions
#[derive(Debug, Clone, Queryable, Selectable, Insertable, Identifiable, Serialize, Deserialize)]
#[diesel(table_name = strategies)]
pub struct Strategy {
    pub id: Uuid,
    pub strategy_name: String,
    pub strategy_type: String,
    pub version: String,
    pub description: Option<String>,
    pub created_by: Option<String>,
    pub is_active: bool,
    pub base_configuration: Option<JsonValue>,
    pub metadata: Option<JsonValue>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// New strategy for insertion
#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = strategies)]
pub struct NewStrategy {
    pub strategy_name: String,
    pub strategy_type: String,
    pub version: Option<String>,
    pub description: Option<String>,
    pub created_by: Option<String>,
    pub is_active: Option<bool>,
    pub base_configuration: Option<JsonValue>,
    pub metadata: Option<JsonValue>,
}

/// Parameter definitions and validation rules for each strategy
#[derive(Debug, Clone, Queryable, Selectable, Insertable, Identifiable, Associations, Serialize, Deserialize)]
#[diesel(belongs_to(Strategy))]
#[diesel(table_name = strategy_parameters)]
pub struct StrategyParameter {
    pub id: Uuid,
    pub strategy_id: Uuid,
    pub parameter_name: String,
    pub parameter_type: String,
    pub is_required: bool,
    pub default_value: Option<JsonValue>,
    pub min_value: Option<BigDecimal>,
    pub max_value: Option<BigDecimal>,
    pub allowed_values: Option<JsonValue>,
    pub validation_pattern: Option<String>,
    pub display_name: Option<String>,
    pub description: Option<String>,
    pub parameter_group: Option<String>,
    pub display_order: Option<i32>,
    pub is_optimizable: bool,
    pub optimization_min: Option<BigDecimal>,
    pub optimization_max: Option<BigDecimal>,
    pub optimization_step: Option<BigDecimal>,
    pub created_at: DateTime<Utc>,
}

/// New strategy parameter for insertion
#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = strategy_parameters)]
pub struct NewStrategyParameter {
    pub strategy_id: Uuid,
    pub parameter_name: String,
    pub parameter_type: String,
    pub is_required: Option<bool>,
    pub default_value: Option<JsonValue>,
    pub min_value: Option<BigDecimal>,
    pub max_value: Option<BigDecimal>,
    pub allowed_values: Option<JsonValue>,
    pub validation_pattern: Option<String>,
    pub display_name: Option<String>,
    pub description: Option<String>,
    pub parameter_group: Option<String>,
    pub display_order: Option<i32>,
    pub is_optimizable: Option<bool>,
    pub optimization_min: Option<BigDecimal>,
    pub optimization_max: Option<BigDecimal>,
    pub optimization_step: Option<BigDecimal>,
}

/// Specific parameter configurations of strategies
#[derive(Debug, Clone, Queryable, Selectable, Insertable, Identifiable, Associations, Serialize, Deserialize)]
#[diesel(belongs_to(Strategy))]
#[diesel(table_name = strategy_instances)]
pub struct StrategyInstance {
    pub id: Uuid,
    pub strategy_id: Uuid,
    pub instance_name: Option<String>,
    pub description: Option<String>,
    pub parameters: JsonValue,
    pub performance_summary: Option<JsonValue>,
    pub risk_metrics: Option<JsonValue>,
    pub is_template: bool,
    pub tags: Option<Vec<Option<String>>>,
    pub created_by: Option<String>,
    pub optimization_run_id: Option<Uuid>,
    pub optimization_score: Option<BigDecimal>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// New strategy instance for insertion
#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = strategy_instances)]
pub struct NewStrategyInstance {
    pub strategy_id: Uuid,
    pub instance_name: Option<String>,
    pub description: Option<String>,
    pub parameters: JsonValue,
    pub performance_summary: Option<JsonValue>,
    pub risk_metrics: Option<JsonValue>,
    pub is_template: Option<bool>,
    pub tags: Option<Vec<Option<String>>>,
    pub created_by: Option<String>,
    pub optimization_run_id: Option<Uuid>,
    pub optimization_score: Option<BigDecimal>,
}

/// Parameter optimization runs and history
#[derive(Debug, Clone, Queryable, Selectable, Insertable, Identifiable, Associations, Serialize, Deserialize)]
#[diesel(belongs_to(Strategy))]
#[diesel(table_name = optimization_runs)]
pub struct OptimizationRun {
    pub id: Uuid,
    pub strategy_id: Uuid,
    pub run_name: String,
    pub optimization_method: String,
    pub objective_function: String,
    pub optimization_config: Option<JsonValue>,
    pub parameter_ranges: JsonValue,
    pub constraints: Option<JsonValue>,
    pub status: String,
    pub total_iterations: Option<i32>,
    pub completed_iterations: Option<i32>,
    pub best_score: Option<BigDecimal>,
    pub best_parameters: Option<JsonValue>,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub error_message: Option<String>,
    pub created_by: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// New optimization run for insertion
#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = optimization_runs)]
pub struct NewOptimizationRun {
    pub strategy_id: Uuid,
    pub run_name: String,
    pub optimization_method: String,
    pub objective_function: String,
    pub optimization_config: Option<JsonValue>,
    pub parameter_ranges: JsonValue,
    pub constraints: Option<JsonValue>,
    pub total_iterations: Option<i32>,
    pub created_by: Option<String>,
}

/// Individual trials within optimization runs
#[derive(Debug, Clone, Queryable, Selectable, Insertable, Identifiable, Associations, Serialize, Deserialize)]
#[diesel(belongs_to(OptimizationRun))]
#[diesel(table_name = optimization_iterations)]
pub struct OptimizationIteration {
    pub id: Uuid,
    pub optimization_run_id: Uuid,
    pub iteration_number: i32,
    pub parameters: JsonValue,
    pub objective_score: Option<BigDecimal>,
    pub additional_metrics: Option<JsonValue>,
    pub started_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
    pub execution_time_ms: Option<i32>,
    pub status: String,
    pub error_message: Option<String>,
}

/// New optimization iteration for insertion
#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = optimization_iterations)]
pub struct NewOptimizationIteration {
    pub optimization_run_id: Uuid,
    pub iteration_number: i32,
    pub parameters: JsonValue,
    pub objective_score: Option<BigDecimal>,
    pub additional_metrics: Option<JsonValue>,
    pub execution_time_ms: Option<i32>,
    pub status: Option<String>,
    pub error_message: Option<String>,
}

/// Results of strategy performance comparisons
#[derive(Debug, Clone, Queryable, Selectable, Insertable, Identifiable, Serialize, Deserialize)]
#[diesel(table_name = strategy_comparisons)]
pub struct StrategyComparison {
    pub id: Uuid,
    pub comparison_name: String,
    pub description: Option<String>,
    pub strategies: JsonValue,
    pub comparison_period: Option<JsonValue>,
    pub benchmark_symbol: Option<String>,
    pub results: Option<JsonValue>,
    pub summary: Option<JsonValue>,
    pub created_by: Option<String>,
    pub created_at: DateTime<Utc>,
}

/// New strategy comparison for insertion
#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = strategy_comparisons)]
pub struct NewStrategyComparison {
    pub comparison_name: String,
    pub description: Option<String>,
    pub strategies: JsonValue,
    pub comparison_period: Option<JsonValue>,
    pub benchmark_symbol: Option<String>,
    pub results: Option<JsonValue>,
    pub summary: Option<JsonValue>,
    pub created_by: Option<String>,
}

// Utility structs for common operations

/// Strategy with its parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StrategyWithParameters {
    pub strategy: Strategy,
    pub parameters: Vec<StrategyParameter>,
}

/// Strategy instance with full strategy details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FullStrategyInstance {
    pub instance: StrategyInstance,
    pub strategy: Strategy,
    pub parameters: Vec<StrategyParameter>,
}

/// Optimization run with its iterations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationRunWithIterations {
    pub run: OptimizationRun,
    pub iterations: Vec<OptimizationIteration>,
}

/// Parameter validation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParameterValidationResult {
    pub parameter_name: String,
    pub is_valid: bool,
    pub error_message: Option<String>,
    pub suggested_value: Option<JsonValue>,
}

/// Strategy performance comparison
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StrategyPerformanceComparison {
    pub strategy_instance_id: Uuid,
    pub strategy_name: String,
    pub parameters: JsonValue,
    pub sharpe_ratio: Option<BigDecimal>,
    pub total_return: Option<BigDecimal>,
    pub max_drawdown: Option<BigDecimal>,
    pub win_rate: Option<BigDecimal>,
    pub profit_factor: Option<BigDecimal>,
}

/// Optimization result summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationSummary {
    pub run_id: Uuid,
    pub strategy_name: String,
    pub total_iterations: i32,
    pub best_score: BigDecimal,
    pub best_parameters: JsonValue,
    pub improvement_over_baseline: Option<BigDecimal>,
    pub optimization_time_minutes: Option<i32>,
}
