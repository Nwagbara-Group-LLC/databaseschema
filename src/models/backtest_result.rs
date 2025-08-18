use bigdecimal::BigDecimal;
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Queryable, Identifiable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::backtest_results)]
pub struct BacktestResult {
    pub id: Uuid,
    pub backtest_id: Uuid,
    pub strategy_name: String,
    pub symbol: String,
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
    pub initial_capital: BigDecimal,
    pub commission_rate: BigDecimal,
    pub slippage_model_type: String,
    pub slippage_fixed_rate: Option<BigDecimal>,
    pub slippage_sqrt_rate: Option<BigDecimal>,
    pub slippage_linear_rate: Option<BigDecimal>,
    pub temporary_impact: BigDecimal,
    pub permanent_impact: BigDecimal,
    pub participation_rate_limit: BigDecimal,
    pub benchmark: Option<String>,
    pub rebalancing_frequency: String,
    pub point_in_time: bool,
    pub warmup_period_days: i32,
    pub total_return: BigDecimal,
    pub annualized_return: BigDecimal,
    pub volatility: BigDecimal,
    pub sharpe_ratio: Option<BigDecimal>,
    pub sortino_ratio: Option<BigDecimal>,
    pub max_drawdown: BigDecimal,
    pub calmar_ratio: Option<BigDecimal>,
    pub win_rate: BigDecimal,
    pub profit_factor: BigDecimal,
    pub avg_trade_return: BigDecimal,
    pub total_trades: i32,
    pub best_trade: Option<BigDecimal>,
    pub worst_trade: Option<BigDecimal>,
    pub avg_time_in_trade: Option<BigDecimal>,
    pub value_at_risk_95: Option<BigDecimal>,
    pub expected_shortfall_95: Option<BigDecimal>,
    pub beta: Option<BigDecimal>,
    pub correlation_with_benchmark: Option<BigDecimal>,
    pub tracking_error: Option<BigDecimal>,
    pub information_ratio: Option<BigDecimal>,
    pub jensen_alpha: Option<BigDecimal>,
    pub max_drawdown_duration_days: Option<i32>,
    pub current_drawdown: BigDecimal,
    pub avg_drawdown: Option<BigDecimal>,
    pub benchmark_return: Option<BigDecimal>,
    pub excess_return: Option<BigDecimal>,
    pub outperformance_periods: Option<i32>,
    pub underperformance_periods: Option<i32>,
    pub total_orders: i32,
    pub filled_orders: i32,
    pub cancelled_orders: i32,
    pub avg_slippage: BigDecimal,
    pub total_commission_paid: BigDecimal,
    pub avg_fill_time_seconds: Option<BigDecimal>,
    pub strategy_metrics: Option<serde_json::Value>,
    pub strategy_instance_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::backtest_results)]
pub struct NewBacktestResult {
    pub backtest_id: Uuid,
    pub strategy_name: String,
    pub symbol: String,
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
    pub initial_capital: BigDecimal,
    pub commission_rate: BigDecimal,
    pub slippage_model_type: String,
    pub slippage_fixed_rate: Option<BigDecimal>,
    pub slippage_sqrt_rate: Option<BigDecimal>,
    pub slippage_linear_rate: Option<BigDecimal>,
    pub temporary_impact: BigDecimal,
    pub permanent_impact: BigDecimal,
    pub participation_rate_limit: BigDecimal,
    pub benchmark: Option<String>,
    pub rebalancing_frequency: String,
    pub point_in_time: bool,
    pub warmup_period_days: i32,
    pub total_return: BigDecimal,
    pub annualized_return: BigDecimal,
    pub volatility: BigDecimal,
    pub sharpe_ratio: Option<BigDecimal>,
    pub sortino_ratio: Option<BigDecimal>,
    pub max_drawdown: BigDecimal,
    pub calmar_ratio: Option<BigDecimal>,
    pub win_rate: BigDecimal,
    pub profit_factor: BigDecimal,
    pub avg_trade_return: BigDecimal,
    pub total_trades: i32,
    pub best_trade: Option<BigDecimal>,
    pub worst_trade: Option<BigDecimal>,
    pub avg_time_in_trade: Option<BigDecimal>,
    pub value_at_risk_95: Option<BigDecimal>,
    pub expected_shortfall_95: Option<BigDecimal>,
    pub beta: Option<BigDecimal>,
    pub correlation_with_benchmark: Option<BigDecimal>,
    pub tracking_error: Option<BigDecimal>,
    pub information_ratio: Option<BigDecimal>,
    pub jensen_alpha: Option<BigDecimal>,
    pub max_drawdown_duration_days: Option<i32>,
    pub current_drawdown: BigDecimal,
    pub avg_drawdown: Option<BigDecimal>,
    pub benchmark_return: Option<BigDecimal>,
    pub excess_return: Option<BigDecimal>,
    pub outperformance_periods: Option<i32>,
    pub underperformance_periods: Option<i32>,
    pub total_orders: i32,
    pub filled_orders: i32,
    pub cancelled_orders: i32,
    pub avg_slippage: BigDecimal,
    pub total_commission_paid: BigDecimal,
    pub avg_fill_time_seconds: Option<BigDecimal>,
    pub strategy_metrics: Option<serde_json::Value>,
    pub strategy_instance_id: Option<Uuid>,
}

#[derive(Debug, Clone, Queryable, Identifiable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::backtest_trades)]
pub struct BacktestTrade {
    pub id: Uuid,
    pub backtest_result_id: Uuid,
    pub trade_id: Uuid,
    pub order_id: Uuid,
    pub symbol: String,
    pub side: String,
    pub quantity: BigDecimal,
    pub price: BigDecimal,
    pub commission: BigDecimal,
    pub timestamp: DateTime<Utc>,
    pub metadata: Option<serde_json::Value>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::backtest_trades)]
pub struct NewBacktestTrade {
    pub backtest_result_id: Uuid,
    pub trade_id: Uuid,
    pub order_id: Uuid,
    pub symbol: String,
    pub side: String,
    pub quantity: BigDecimal,
    pub price: BigDecimal,
    pub commission: BigDecimal,
    pub timestamp: DateTime<Utc>,
    pub metadata: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Queryable, Identifiable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::backtest_equity_curve)]
pub struct BacktestEquityCurve {
    pub id: Uuid,
    pub backtest_result_id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub portfolio_value: BigDecimal,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::backtest_equity_curve)]
pub struct NewBacktestEquityCurve {
    pub backtest_result_id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub portfolio_value: BigDecimal,
}

#[derive(Debug, Clone, Queryable, Identifiable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::backtest_position_history)]
pub struct BacktestPositionHistory {
    pub id: Uuid,
    pub backtest_result_id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub symbol: String,
    pub quantity: BigDecimal,
    pub average_price: BigDecimal,
    pub current_price: BigDecimal,
    pub unrealized_pnl: BigDecimal,
    pub direction: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::backtest_position_history)]
pub struct NewBacktestPositionHistory {
    pub backtest_result_id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub symbol: String,
    pub quantity: BigDecimal,
    pub average_price: BigDecimal,
    pub current_price: BigDecimal,
    pub unrealized_pnl: BigDecimal,
    pub direction: String,
}

#[derive(Debug, Clone, Queryable, Identifiable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::backtest_drawdown_periods)]
pub struct BacktestDrawdownPeriod {
    pub id: Uuid,
    pub backtest_result_id: Uuid,
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
    pub duration_days: i32,
    pub magnitude: BigDecimal,
    pub recovery_date: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::backtest_drawdown_periods)]
pub struct NewBacktestDrawdownPeriod {
    pub backtest_result_id: Uuid,
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
    pub duration_days: i32,
    pub magnitude: BigDecimal,
    pub recovery_date: Option<DateTime<Utc>>,
}
