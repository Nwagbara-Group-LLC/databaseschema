use bigdecimal::BigDecimal;
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Queryable, Identifiable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::strategies)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Strategy {
    pub id: Uuid,
    pub strategy_name: String,
    pub strategy_type: String,
    pub version: String,
    pub description: Option<String>,
    pub created_by: Option<String>,
    pub is_active: bool,
    pub base_configuration: Option<serde_json::Value>,
    pub metadata: Option<serde_json::Value>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::strategies)]
pub struct NewStrategy {
    pub strategy_name: String,
    pub strategy_type: String,
    pub version: String,
    pub description: Option<String>,
    pub created_by: Option<String>,
    pub is_active: bool,
    pub base_configuration: Option<serde_json::Value>,
    pub metadata: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Queryable, Identifiable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::strategy_parameters)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct StrategyParameter {
    pub id: Uuid,
    pub strategy_id: Uuid,
    pub parameter_name: String,
    pub parameter_type: String,
    pub is_required: bool,
    pub default_value: Option<serde_json::Value>,
    pub min_value: Option<BigDecimal>,
    pub max_value: Option<BigDecimal>,
    pub allowed_values: Option<serde_json::Value>,
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

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::strategy_parameters)]
pub struct NewStrategyParameter {
    pub strategy_id: Uuid,
    pub parameter_name: String,
    pub parameter_type: String,
    pub is_required: bool,
    pub default_value: Option<serde_json::Value>,
    pub min_value: Option<BigDecimal>,
    pub max_value: Option<BigDecimal>,
    pub allowed_values: Option<serde_json::Value>,
    pub validation_pattern: Option<String>,
    pub display_name: Option<String>,
    pub description: Option<String>,
    pub parameter_group: Option<String>,
    pub display_order: Option<i32>,
    pub is_optimizable: bool,
    pub optimization_min: Option<BigDecimal>,
    pub optimization_max: Option<BigDecimal>,
    pub optimization_step: Option<BigDecimal>,
}

#[derive(Debug, Clone, Queryable, Identifiable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::strategy_instances)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct StrategyInstance {
    pub id: Uuid,
    pub strategy_id: Uuid,
    pub instance_name: Option<String>,
    pub description: Option<String>,
    pub parameters: serde_json::Value,
    pub performance_summary: Option<serde_json::Value>,
    pub risk_metrics: Option<serde_json::Value>,
    pub is_template: bool,
    pub tags: Option<Vec<Option<String>>>,
    pub created_by: Option<String>,
    pub optimization_run_id: Option<Uuid>,
    pub optimization_score: Option<BigDecimal>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::strategy_instances)]
pub struct NewStrategyInstance {
    pub strategy_id: Uuid,
    pub instance_name: Option<String>,
    pub description: Option<String>,
    pub parameters: serde_json::Value,
    pub performance_summary: Option<serde_json::Value>,
    pub risk_metrics: Option<serde_json::Value>,
    pub is_template: bool,
    pub tags: Option<Vec<Option<String>>>,
    pub created_by: Option<String>,
    pub optimization_run_id: Option<Uuid>,
    pub optimization_score: Option<BigDecimal>,
}

#[derive(Debug, Clone, Queryable, Identifiable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::optimization_runs)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct OptimizationRun {
    pub id: Uuid,
    pub strategy_id: Uuid,
    pub run_name: String,
    pub optimization_method: String,
    pub objective_function: String,
    pub optimization_config: Option<serde_json::Value>,
    pub parameter_ranges: serde_json::Value,
    pub constraints: Option<serde_json::Value>,
    pub status: String,
    pub total_iterations: Option<i32>,
    pub completed_iterations: Option<i32>,
    pub best_score: Option<BigDecimal>,
    pub best_parameters: Option<serde_json::Value>,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub error_message: Option<String>,
    pub created_by: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::optimization_runs)]
pub struct NewOptimizationRun {
    pub strategy_id: Uuid,
    pub run_name: String,
    pub optimization_method: String,
    pub objective_function: String,
    pub optimization_config: Option<serde_json::Value>,
    pub parameter_ranges: serde_json::Value,
    pub constraints: Option<serde_json::Value>,
    pub status: String,
    pub total_iterations: Option<i32>,
    pub created_by: Option<String>,
}

#[derive(Debug, Clone, Queryable, Identifiable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::optimization_iterations)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct OptimizationIteration {
    pub id: Uuid,
    pub optimization_run_id: Uuid,
    pub iteration_number: i32,
    pub parameters: serde_json::Value,
    pub objective_score: Option<BigDecimal>,
    pub additional_metrics: Option<serde_json::Value>,
    pub started_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
    pub execution_time_ms: Option<i32>,
    pub status: String,
    pub error_message: Option<String>,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::optimization_iterations)]
pub struct NewOptimizationIteration {
    pub optimization_run_id: Uuid,
    pub iteration_number: i32,
    pub parameters: serde_json::Value,
    pub objective_score: Option<BigDecimal>,
    pub additional_metrics: Option<serde_json::Value>,
    pub execution_time_ms: Option<i32>,
    pub status: String,
    pub error_message: Option<String>,
}

#[derive(Debug, Clone, Queryable, Identifiable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::strategy_comparisons)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct StrategyComparison {
    pub id: Uuid,
    pub comparison_name: String,
    pub description: Option<String>,
    pub strategies: serde_json::Value,
    pub comparison_period: Option<serde_json::Value>,
    pub benchmark_symbol: Option<String>,
    pub results: Option<serde_json::Value>,
    pub summary: Option<serde_json::Value>,
    pub created_by: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::strategy_comparisons)]
pub struct NewStrategyComparison {
    pub comparison_name: String,
    pub description: Option<String>,
    pub strategies: serde_json::Value,
    pub comparison_period: Option<serde_json::Value>,
    pub benchmark_symbol: Option<String>,
    pub results: Option<serde_json::Value>,
    pub summary: Option<serde_json::Value>,
    pub created_by: Option<String>,
}

// Composite structs for complex operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StrategyWithParameters {
    pub strategy: Strategy,
    pub parameters: Vec<StrategyParameter>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FullStrategyInstance {
    pub instance: StrategyInstance,
    pub strategy: Strategy,
    pub parameters: Vec<StrategyParameter>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParameterValidationResult {
    pub parameter_name: String,
    pub is_valid: bool,
    pub error_message: Option<String>,
    pub normalized_value: Option<serde_json::Value>,
    pub suggested_value: Option<serde_json::Value>,
}
