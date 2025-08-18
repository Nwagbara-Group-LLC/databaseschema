use diesel::prelude::*;
use diesel::result::Error as DieselError;
use serde_json::Value as JsonValue;
use uuid::Uuid;
use bigdecimal::{BigDecimal, FromPrimitive};

use crate::models::strategy::*;
use crate::schema::{
    strategies, strategy_parameters, strategy_instances, 
    optimization_runs, optimization_iterations, strategy_comparisons
};

/// Operations for managing trading strategies and their configurations
pub struct StrategyOperations;

impl StrategyOperations {
    // === Strategy Management ===

    /// Create a new strategy definition
    pub async fn create_strategy(
        conn: &mut PgConnection,
        new_strategy: NewStrategy,
    ) -> Result<Strategy, DieselError> {
        diesel::insert_into(strategies::table)
            .values(&new_strategy)
            .get_result(conn)
    }

    /// Get strategy by ID
    pub async fn get_strategy(
        conn: &mut PgConnection,
        strategy_id: Uuid,
    ) -> Result<Strategy, DieselError> {
        strategies::table
            .find(strategy_id)
            .first(conn)
    }

    /// Get strategy by name and version
    pub async fn get_strategy_by_name_version(
        conn: &mut PgConnection,
        name: &str,
        version: &str,
    ) -> Result<Strategy, DieselError> {
        strategies::table
            .filter(strategies::strategy_name.eq(name))
            .filter(strategies::version.eq(version))
            .first(conn)
    }

    /// List all active strategies
    pub async fn list_active_strategies(
        conn: &mut PgConnection,
    ) -> Result<Vec<Strategy>, DieselError> {
        strategies::table
            .filter(strategies::is_active.eq(true))
            .order(strategies::strategy_name.asc())
            .load(conn)
    }

    /// List strategies by type
    pub async fn list_strategies_by_type(
        conn: &mut PgConnection,
        strategy_type: &str,
    ) -> Result<Vec<Strategy>, DieselError> {
        strategies::table
            .filter(strategies::strategy_type.eq(strategy_type))
            .filter(strategies::is_active.eq(true))
            .order(strategies::version.desc())
            .load(conn)
    }

    /// Update strategy (simplified version)
    pub async fn update_strategy_simple(
        conn: &mut PgConnection,
        strategy_id: Uuid,
        description: Option<String>,
        is_active: bool,
    ) -> Result<Strategy, DieselError> {
        diesel::update(strategies::table.filter(strategies::id.eq(strategy_id)))
            .set((
                strategies::description.eq(description),
                strategies::is_active.eq(is_active),
                strategies::updated_at.eq(diesel::dsl::now),
            ))
            .get_result(conn)
    }

    /// Delete strategy (soft delete by setting is_active = false)
    pub async fn delete_strategy(
        conn: &mut PgConnection,
        strategy_id: Uuid,
    ) -> Result<Strategy, DieselError> {
        diesel::update(strategies::table.filter(strategies::id.eq(strategy_id)))
            .set(strategies::is_active.eq(false))
            .get_result(conn)
    }

    // === Strategy Parameters ===

    /// Create strategy parameter
    pub async fn create_strategy_parameter(
        conn: &mut PgConnection,
        new_parameter: NewStrategyParameter,
    ) -> Result<StrategyParameter, DieselError> {
        diesel::insert_into(strategy_parameters::table)
            .values(&new_parameter)
            .get_result(conn)
    }

    /// Get parameters for strategy
    pub async fn get_strategy_parameters(
        conn: &mut PgConnection,
        strategy_id: Uuid,
    ) -> Result<Vec<StrategyParameter>, DieselError> {
        strategy_parameters::table
            .filter(strategy_parameters::strategy_id.eq(strategy_id))
            .order(strategy_parameters::display_order.asc())
            .load(conn)
    }

    /// Get optimizable parameters for strategy
    pub async fn get_optimizable_parameters(
        conn: &mut PgConnection,
        strategy_id: Uuid,
    ) -> Result<Vec<StrategyParameter>, DieselError> {
        strategy_parameters::table
            .filter(strategy_parameters::strategy_id.eq(strategy_id))
            .filter(strategy_parameters::is_optimizable.eq(true))
            .order(strategy_parameters::parameter_name.asc())
            .load(conn)
    }

    /// Get strategy with parameters
    pub async fn get_strategy_with_parameters(
        conn: &mut PgConnection,
        strategy_id: Uuid,
    ) -> Result<StrategyWithParameters, DieselError> {
        let strategy = Self::get_strategy(conn, strategy_id).await?;
        let parameters = Self::get_strategy_parameters(conn, strategy_id).await?;
        
        Ok(StrategyWithParameters {
            strategy,
            parameters,
        })
    }

    /// Validate strategy parameters against definition
    pub async fn validate_strategy_parameters(
        conn: &mut PgConnection,
        strategy_id: Uuid,
        parameters: &JsonValue,
    ) -> Result<Vec<ParameterValidationResult>, DieselError> {
        let param_definitions = Self::get_strategy_parameters(conn, strategy_id).await?;
        let mut results = Vec::new();

        let params_obj = parameters.as_object()
            .ok_or_else(|| DieselError::DeserializationError("Parameters must be a JSON object".into()))?;

        for param_def in param_definitions {
            let param_value = params_obj.get(&param_def.parameter_name);
            let mut result = ParameterValidationResult {
                parameter_name: param_def.parameter_name.clone(),
                is_valid: true,
                error_message: None,
                suggested_value: None,
            };

            // Check required parameters
            if param_def.is_required && param_value.is_none() {
                result.is_valid = false;
                result.error_message = Some("Required parameter is missing".to_string());
                result.suggested_value = param_def.default_value.clone();
                results.push(result);
                continue;
            }

            if let Some(value) = param_value {
                // Type validation
                let type_valid = match param_def.parameter_type.as_str() {
                    "float" => value.is_f64(),
                    "integer" => value.is_i64(),
                    "string" => value.is_string(),
                    "boolean" => value.is_boolean(),
                    "array" => value.is_array(),
                    "object" => value.is_object(),
                    _ => false,
                };

                if !type_valid {
                    result.is_valid = false;
                    result.error_message = Some(format!("Expected type: {}", param_def.parameter_type));
                }

                // Range validation for numeric types
                if result.is_valid && (param_def.parameter_type == "float" || param_def.parameter_type == "integer") {
                    if let Some(num_val) = value.as_f64() {
                        let num_val_decimal = BigDecimal::from_f64(num_val).unwrap_or_else(|| BigDecimal::from(0));
                        if let Some(min) = &param_def.min_value {
                            if &num_val_decimal < min {
                                result.is_valid = false;
                                result.error_message = Some(format!("Value must be >= {}", min));
                                result.suggested_value = Some(JsonValue::from(min.to_string().parse::<f64>().unwrap_or(0.0)));
                            }
                        }
                        if let Some(max) = &param_def.max_value {
                            if &num_val_decimal > max {
                                result.is_valid = false;
                                result.error_message = Some(format!("Value must be <= {}", max));
                                result.suggested_value = Some(JsonValue::from(max.to_string().parse::<f64>().unwrap_or(0.0)));
                            }
                        }
                    }
                }

                // Allowed values validation
                if result.is_valid {
                    if let Some(allowed) = &param_def.allowed_values {
                        if let Some(allowed_array) = allowed.as_array() {
                            if !allowed_array.contains(value) {
                                result.is_valid = false;
                                result.error_message = Some("Value not in allowed list".to_string());
                                result.suggested_value = allowed_array.get(0).cloned();
                            }
                        }
                    }
                }
            }

            results.push(result);
        }

        Ok(results)
    }

    // === Strategy Instances ===

    /// Create strategy instance
    pub async fn create_strategy_instance(
        conn: &mut PgConnection,
        new_instance: NewStrategyInstance,
    ) -> Result<StrategyInstance, DieselError> {
        // Validate parameters before creating
        let validation_results = Self::validate_strategy_parameters(
            conn, 
            new_instance.strategy_id, 
            &new_instance.parameters
        ).await?;

        let invalid_params: Vec<_> = validation_results
            .iter()
            .filter(|r| !r.is_valid)
            .collect();

        if !invalid_params.is_empty() {
            return Err(DieselError::RollbackTransaction);
        }

        diesel::insert_into(strategy_instances::table)
            .values(&new_instance)
            .get_result(conn)
    }

    /// Get strategy instance by ID
    pub async fn get_strategy_instance(
        conn: &mut PgConnection,
        instance_id: Uuid,
    ) -> Result<StrategyInstance, DieselError> {
        strategy_instances::table
            .find(instance_id)
            .first(conn)
    }

    /// Get full strategy instance with strategy details
    pub async fn get_full_strategy_instance(
        conn: &mut PgConnection,
        instance_id: Uuid,
    ) -> Result<FullStrategyInstance, DieselError> {
        let instance = Self::get_strategy_instance(conn, instance_id).await?;
        let strategy = Self::get_strategy(conn, instance.strategy_id).await?;
        let parameters = Self::get_strategy_parameters(conn, instance.strategy_id).await?;

        Ok(FullStrategyInstance {
            instance,
            strategy,
            parameters,
        })
    }

    /// List strategy instances for a strategy
    pub async fn list_strategy_instances(
        conn: &mut PgConnection,
        strategy_id: Uuid,
        include_templates: bool,
    ) -> Result<Vec<StrategyInstance>, DieselError> {
        let mut query = strategy_instances::table
            .filter(strategy_instances::strategy_id.eq(strategy_id))
            .into_boxed();

        if !include_templates {
            query = query.filter(strategy_instances::is_template.eq(false));
        }

        query
            .order(strategy_instances::created_at.desc())
            .load(conn)
    }

    /// Get template strategy instances
    pub async fn get_template_instances(
        conn: &mut PgConnection,
        strategy_id: Option<Uuid>,
    ) -> Result<Vec<StrategyInstance>, DieselError> {
        let mut query = strategy_instances::table
            .filter(strategy_instances::is_template.eq(true))
            .into_boxed();

        if let Some(id) = strategy_id {
            query = query.filter(strategy_instances::strategy_id.eq(id));
        }

        query
            .order(strategy_instances::created_at.desc())
            .load(conn)
    }

    /// Update strategy instance performance
    pub async fn update_instance_performance(
        conn: &mut PgConnection,
        instance_id: Uuid,
        performance_summary: JsonValue,
        risk_metrics: Option<JsonValue>,
    ) -> Result<StrategyInstance, DieselError> {
        diesel::update(strategy_instances::table.filter(strategy_instances::id.eq(instance_id)))
            .set((
                strategy_instances::performance_summary.eq(Some(performance_summary)),
                strategy_instances::risk_metrics.eq(risk_metrics),
                strategy_instances::updated_at.eq(diesel::dsl::now),
            ))
            .get_result(conn)
    }

    // === Optimization ===

    /// Create optimization run
    pub async fn create_optimization_run(
        conn: &mut PgConnection,
        new_run: NewOptimizationRun,
    ) -> Result<OptimizationRun, DieselError> {
        diesel::insert_into(optimization_runs::table)
            .values(&new_run)
            .get_result(conn)
    }

    /// Get optimization run
    pub async fn get_optimization_run(
        conn: &mut PgConnection,
        run_id: Uuid,
    ) -> Result<OptimizationRun, DieselError> {
        optimization_runs::table
            .find(run_id)
            .first(conn)
    }

    /// Update optimization run status
    pub async fn update_optimization_run_status(
        conn: &mut PgConnection,
        run_id: Uuid,
        status: &str,
        completed_iterations: Option<i32>,
        best_score: Option<BigDecimal>,
        best_parameters: Option<JsonValue>,
        error_message: Option<String>,
    ) -> Result<OptimizationRun, DieselError> {
        diesel::update(optimization_runs::table.filter(optimization_runs::id.eq(run_id)))
            .set((
                optimization_runs::status.eq(status),
                optimization_runs::completed_iterations.eq(completed_iterations),
                optimization_runs::best_score.eq(best_score),
                optimization_runs::best_parameters.eq(best_parameters),
                optimization_runs::error_message.eq(error_message),
                optimization_runs::updated_at.eq(diesel::dsl::now),
            ))
            .get_result(conn)
    }

    /// Create optimization iteration
    pub async fn create_optimization_iteration(
        conn: &mut PgConnection,
        new_iteration: NewOptimizationIteration,
    ) -> Result<OptimizationIteration, DieselError> {
        diesel::insert_into(optimization_iterations::table)
            .values(&new_iteration)
            .get_result(conn)
    }

    /// Get optimization iterations for a run
    pub async fn get_optimization_iterations(
        conn: &mut PgConnection,
        run_id: Uuid,
        limit: Option<i64>,
    ) -> Result<Vec<OptimizationIteration>, DieselError> {
        let mut query = optimization_iterations::table
            .filter(optimization_iterations::optimization_run_id.eq(run_id))
            .order(optimization_iterations::objective_score.desc().nulls_last())
            .into_boxed();

        if let Some(limit_val) = limit {
            query = query.limit(limit_val);
        }

        query.load(conn)
    }

    /// Get best optimization results
    pub async fn get_best_optimization_results(
        conn: &mut PgConnection,
        run_id: Uuid,
        top_n: i64,
    ) -> Result<Vec<OptimizationIteration>, DieselError> {
        optimization_iterations::table
            .filter(optimization_iterations::optimization_run_id.eq(run_id))
            .filter(optimization_iterations::objective_score.is_not_null())
            .order(optimization_iterations::objective_score.desc())
            .limit(top_n)
            .load(conn)
    }

    // === Strategy Comparisons ===

    /// Create strategy comparison
    pub async fn create_strategy_comparison(
        conn: &mut PgConnection,
        new_comparison: NewStrategyComparison,
    ) -> Result<StrategyComparison, DieselError> {
        diesel::insert_into(strategy_comparisons::table)
            .values(&new_comparison)
            .get_result(conn)
    }

    /// Get strategy comparison
    pub async fn get_strategy_comparison(
        conn: &mut PgConnection,
        comparison_id: Uuid,
    ) -> Result<StrategyComparison, DieselError> {
        strategy_comparisons::table
            .find(comparison_id)
            .first(conn)
    }

    /// List strategy comparisons
    pub async fn list_strategy_comparisons(
        conn: &mut PgConnection,
        created_by: Option<&str>,
        limit: Option<i64>,
    ) -> Result<Vec<StrategyComparison>, DieselError> {
        let mut query = strategy_comparisons::table
            .into_boxed();

        if let Some(user) = created_by {
            query = query.filter(strategy_comparisons::created_by.eq(user));
        }

        if let Some(limit_val) = limit {
            query = query.limit(limit_val);
        }

        query
            .order(strategy_comparisons::created_at.desc())
            .load(conn)
    }

    // === Utility Operations ===

    /// Get strategy statistics
    pub async fn get_strategy_statistics(
        conn: &mut PgConnection,
        strategy_id: Uuid,
    ) -> Result<JsonValue, DieselError> {
        // Count instances, optimizations, etc.
        use diesel::dsl::count;

        let instance_count: i64 = strategy_instances::table
            .filter(strategy_instances::strategy_id.eq(strategy_id))
            .select(count(strategy_instances::id))
            .first(conn)?;

        let optimization_count: i64 = optimization_runs::table
            .filter(optimization_runs::strategy_id.eq(strategy_id))
            .select(count(optimization_runs::id))
            .first(conn)?;

        let template_count: i64 = strategy_instances::table
            .filter(strategy_instances::strategy_id.eq(strategy_id))
            .filter(strategy_instances::is_template.eq(true))
            .select(count(strategy_instances::id))
            .first(conn)?;

        Ok(serde_json::json!({
            "instance_count": instance_count,
            "optimization_count": optimization_count,
            "template_count": template_count
        }))
    }

    /// Search strategies by name or description
    pub async fn search_strategies(
        conn: &mut PgConnection,
        search_term: &str,
        strategy_types: Option<Vec<String>>,
        active_only: bool,
    ) -> Result<Vec<Strategy>, DieselError> {
        let mut query = strategies::table.into_boxed();

        // Text search
        query = query.filter(
            strategies::strategy_name.ilike(format!("%{}%", search_term))
                .or(strategies::description.ilike(format!("%{}%", search_term)))
        );

        // Filter by types
        if let Some(types) = strategy_types {
            query = query.filter(strategies::strategy_type.eq_any(types));
        }

        // Active only
        if active_only {
            query = query.filter(strategies::is_active.eq(true));
        }

        query
            .order(strategies::strategy_name.asc())
            .load(conn)
    }
}
