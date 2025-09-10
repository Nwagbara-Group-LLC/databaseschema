use crate::schema::*;
use diesel::prelude::*;
use diesel::pg::PgValue;
use diesel::serialize::ToSql;
use diesel::deserialize::FromSql;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use bigdecimal::BigDecimal;
use chrono::{DateTime, Utc};
use std::io::Write;

// SQL type enums for strategy orders
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[derive(diesel::deserialize::FromSqlRow, diesel::expression::AsExpression)]
#[diesel(sql_type = crate::schema::sql_types::OrderStatus)]
pub enum OrderStatus {
    Pending,
    Submitted,
    PartiallyFilled,
    Filled,
    Cancelled,
    Rejected,
    Expired,
    Failed,
}

impl FromSql<crate::schema::sql_types::OrderStatus, diesel::pg::Pg> for OrderStatus {
    fn from_sql(bytes: PgValue<'_>) -> diesel::deserialize::Result<Self> {
        match bytes.as_bytes() {
            b"pending" => Ok(OrderStatus::Pending),
            b"submitted" => Ok(OrderStatus::Submitted),
            b"partially_filled" => Ok(OrderStatus::PartiallyFilled),
            b"filled" => Ok(OrderStatus::Filled),
            b"cancelled" => Ok(OrderStatus::Cancelled),
            b"rejected" => Ok(OrderStatus::Rejected),
            b"expired" => Ok(OrderStatus::Expired),
            b"failed" => Ok(OrderStatus::Failed),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}

impl ToSql<crate::schema::sql_types::OrderStatus, diesel::pg::Pg> for OrderStatus {
    fn to_sql<'b>(&'b self, out: &mut diesel::serialize::Output<'b, '_, diesel::pg::Pg>) -> diesel::serialize::Result {
        match *self {
            OrderStatus::Pending => out.write_all(b"pending")?,
            OrderStatus::Submitted => out.write_all(b"submitted")?,
            OrderStatus::PartiallyFilled => out.write_all(b"partially_filled")?,
            OrderStatus::Filled => out.write_all(b"filled")?,
            OrderStatus::Cancelled => out.write_all(b"cancelled")?,
            OrderStatus::Rejected => out.write_all(b"rejected")?,
            OrderStatus::Expired => out.write_all(b"expired")?,
            OrderStatus::Failed => out.write_all(b"failed")?,
        }
        Ok(diesel::serialize::IsNull::No)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[derive(diesel::deserialize::FromSqlRow, diesel::expression::AsExpression)]
#[diesel(sql_type = crate::schema::sql_types::OrderSide)]
pub enum OrderSide {
    Buy,
    Sell,
}

impl diesel::deserialize::FromSql<crate::schema::sql_types::OrderSide, diesel::pg::Pg> for OrderSide {
    fn from_sql(bytes: PgValue<'_>) -> diesel::deserialize::Result<Self> {
        match bytes.as_bytes() {
            b"buy" => Ok(OrderSide::Buy),
            b"sell" => Ok(OrderSide::Sell),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}

impl diesel::serialize::ToSql<crate::schema::sql_types::OrderSide, diesel::pg::Pg> for OrderSide {
    fn to_sql<'b>(&'b self, out: &mut diesel::serialize::Output<'b, '_, diesel::pg::Pg>) -> diesel::serialize::Result {
        match *self {
            OrderSide::Buy => out.write_all(b"buy")?,
            OrderSide::Sell => out.write_all(b"sell")?,
        }
        Ok(diesel::serialize::IsNull::No)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[derive(diesel::deserialize::FromSqlRow, diesel::expression::AsExpression)]
#[diesel(sql_type = crate::schema::sql_types::OrderType)]
pub enum OrderType {
    Market,
    Limit,
    StopLimit,
    Iceberg,
    Twap,
    Vwap,
    Implementation,
}

impl diesel::deserialize::FromSql<crate::schema::sql_types::OrderType, diesel::pg::Pg> for OrderType {
    fn from_sql(bytes: PgValue<'_>) -> diesel::deserialize::Result<Self> {
        match bytes.as_bytes() {
            b"market" => Ok(OrderType::Market),
            b"limit" => Ok(OrderType::Limit),
            b"stop_limit" => Ok(OrderType::StopLimit),
            b"iceberg" => Ok(OrderType::Iceberg),
            b"twap" => Ok(OrderType::Twap),
            b"vwap" => Ok(OrderType::Vwap),
            b"implementation" => Ok(OrderType::Implementation),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}

impl diesel::serialize::ToSql<crate::schema::sql_types::OrderType, diesel::pg::Pg> for OrderType {
    fn to_sql<'b>(&'b self, out: &mut diesel::serialize::Output<'b, '_, diesel::pg::Pg>) -> diesel::serialize::Result {
        match *self {
            OrderType::Market => out.write_all(b"market")?,
            OrderType::Limit => out.write_all(b"limit")?,
            OrderType::StopLimit => out.write_all(b"stop_limit")?,
            OrderType::Iceberg => out.write_all(b"iceberg")?,
            OrderType::Twap => out.write_all(b"twap")?,
            OrderType::Vwap => out.write_all(b"vwap")?,
            OrderType::Implementation => out.write_all(b"implementation")?,
        }
        Ok(diesel::serialize::IsNull::No)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[derive(diesel::deserialize::FromSqlRow, diesel::expression::AsExpression)]
#[diesel(sql_type = crate::schema::sql_types::TimeInForce)]
pub enum TimeInForce {
    Ioc,
    Fok,
    Gtc,
    Day,
    Gtd,
}

impl diesel::deserialize::FromSql<crate::schema::sql_types::TimeInForce, diesel::pg::Pg> for TimeInForce {
    fn from_sql(bytes: PgValue<'_>) -> diesel::deserialize::Result<Self> {
        match bytes.as_bytes() {
            b"ioc" => Ok(TimeInForce::Ioc),
            b"fok" => Ok(TimeInForce::Fok),
            b"gtc" => Ok(TimeInForce::Gtc),
            b"day" => Ok(TimeInForce::Day),
            b"gtd" => Ok(TimeInForce::Gtd),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}

impl diesel::serialize::ToSql<crate::schema::sql_types::TimeInForce, diesel::pg::Pg> for TimeInForce {
    fn to_sql<'b>(&'b self, out: &mut diesel::serialize::Output<'b, '_, diesel::pg::Pg>) -> diesel::serialize::Result {
        match *self {
            TimeInForce::Ioc => out.write_all(b"ioc")?,
            TimeInForce::Fok => out.write_all(b"fok")?,
            TimeInForce::Gtc => out.write_all(b"gtc")?,
            TimeInForce::Day => out.write_all(b"day")?,
            TimeInForce::Gtd => out.write_all(b"gtd")?,
        }
        Ok(diesel::serialize::IsNull::No)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[derive(diesel::deserialize::FromSqlRow, diesel::expression::AsExpression)]
#[diesel(sql_type = crate::schema::sql_types::ExecutionUrgency)]
pub enum ExecutionUrgency {
    Low,
    Medium,
    High,
    Critical,
}

impl diesel::deserialize::FromSql<crate::schema::sql_types::ExecutionUrgency, diesel::pg::Pg> for ExecutionUrgency {
    fn from_sql(bytes: PgValue<'_>) -> diesel::deserialize::Result<Self> {
        match bytes.as_bytes() {
            b"low" => Ok(ExecutionUrgency::Low),
            b"medium" => Ok(ExecutionUrgency::Medium),
            b"high" => Ok(ExecutionUrgency::High),
            b"critical" => Ok(ExecutionUrgency::Critical),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}

impl diesel::serialize::ToSql<crate::schema::sql_types::ExecutionUrgency, diesel::pg::Pg> for ExecutionUrgency {
    fn to_sql<'b>(&'b self, out: &mut diesel::serialize::Output<'b, '_, diesel::pg::Pg>) -> diesel::serialize::Result {
        match *self {
            ExecutionUrgency::Low => out.write_all(b"low")?,
            ExecutionUrgency::Medium => out.write_all(b"medium")?,
            ExecutionUrgency::High => out.write_all(b"high")?,
            ExecutionUrgency::Critical => out.write_all(b"critical")?,
        }
        Ok(diesel::serialize::IsNull::No)
    }
}

// Strategy Order model
#[derive(Debug, Clone, Queryable, Insertable, Identifiable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = strategy_orders)]
#[diesel(primary_key(id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct StrategyOrder {
    pub id: Uuid,
    pub signal_id: i64,
    pub strategy_instance_id: Option<Uuid>,
    pub parent_order_id: Option<Uuid>,
    pub exchange_order_id: Option<String>,
    pub unique_id: String,
    pub symbol: String,
    pub exchange: String,
    pub side: OrderSide,
    pub order_type: OrderType,
    pub time_in_force: Option<TimeInForce>,
    pub original_quantity: BigDecimal,
    pub remaining_quantity: BigDecimal,
    pub filled_quantity: Option<BigDecimal>,
    pub price: Option<BigDecimal>,
    pub stop_price: Option<BigDecimal>,
    pub avg_fill_price: Option<BigDecimal>,
    pub status: OrderStatus,
    pub urgency: Option<ExecutionUrgency>,
    pub fees_paid: Option<BigDecimal>,
    pub strategy_name: String,
    pub strategy_version: Option<String>,
    pub signal_confidence: Option<BigDecimal>,
    pub signal_flags: Option<i32>,
    pub risk_score: Option<BigDecimal>,
    pub compliance_checked: Option<bool>,
    pub risk_limits_checked: Option<bool>,
    pub routing_algorithm: Option<String>,
    pub execution_venue: Option<String>,
    pub child_order_count: Option<i32>,
    pub slippage_bps: Option<i32>,
    pub implementation_shortfall_bps: Option<i32>,
    pub market_impact_bps: Option<i32>,
    pub order_metadata: Option<serde_json::Value>,
    pub execution_context: Option<serde_json::Value>,
    pub tags: Option<Vec<Option<String>>>,
    pub rejection_reason: Option<String>,
    pub error_message: Option<String>,
    pub retry_count: Option<i32>,
    pub signal_timestamp: DateTime<Utc>,
    pub order_created_at: DateTime<Utc>,
    pub order_submitted_at: Option<DateTime<Utc>>,
    pub first_fill_at: Option<DateTime<Utc>>,
    pub last_fill_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub created_by: Option<String>,
    pub updated_at: DateTime<Utc>,
}

// New Strategy Order struct for inserts
#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = strategy_orders)]
pub struct NewStrategyOrder {
    pub signal_id: i64,
    pub strategy_instance_id: Option<Uuid>,
    pub parent_order_id: Option<Uuid>,
    pub unique_id: String,
    pub symbol: String,
    pub exchange: String,
    pub side: OrderSide,
    pub order_type: OrderType,
    pub time_in_force: Option<TimeInForce>,
    pub original_quantity: BigDecimal,
    pub remaining_quantity: BigDecimal,
    pub price: Option<BigDecimal>,
    pub stop_price: Option<BigDecimal>,
    pub status: OrderStatus,
    pub urgency: Option<ExecutionUrgency>,
    pub strategy_name: String,
    pub strategy_version: Option<String>,
    pub signal_confidence: Option<BigDecimal>,
    pub signal_flags: Option<i32>,
    pub signal_timestamp: DateTime<Utc>,
    pub created_by: Option<String>,
}

// Order Fill model
#[derive(Debug, Clone, Queryable, Insertable, Identifiable, Associations, Selectable, Serialize, Deserialize)]
#[diesel(table_name = strategy_order_fills)]
#[diesel(primary_key(id))]
#[diesel(belongs_to(StrategyOrder, foreign_key = order_id))]
pub struct StrategyOrderFill {
    pub id: Uuid,
    pub order_id: Uuid,
    pub fill_id: String,
    pub trade_id: Option<String>,
    pub quantity: BigDecimal,
    pub price: BigDecimal,
    pub fees: Option<BigDecimal>,
    pub fee_currency: Option<String>,
    pub bid_price: Option<BigDecimal>,
    pub ask_price: Option<BigDecimal>,
    pub mid_price: Option<BigDecimal>,
    pub spread_bps: Option<i32>,
    pub is_maker: Option<bool>,
    pub liquidity_flag: Option<String>,
    pub fill_timestamp: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

// New Order Fill struct for inserts
#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = strategy_order_fills)]
pub struct NewStrategyOrderFill {
    pub order_id: Uuid,
    pub fill_id: String,
    pub trade_id: Option<String>,
    pub quantity: BigDecimal,
    pub price: BigDecimal,
    pub fees: Option<BigDecimal>,
    pub fee_currency: Option<String>,
    pub bid_price: Option<BigDecimal>,
    pub ask_price: Option<BigDecimal>,
    pub mid_price: Option<BigDecimal>,
    pub spread_bps: Option<i32>,
    pub is_maker: Option<bool>,
    pub liquidity_flag: Option<String>,
    pub fill_timestamp: DateTime<Utc>,
}

// Order State Change model
#[derive(Debug, Clone, Queryable, Insertable, Identifiable, Associations, Selectable, Serialize, Deserialize)]
#[diesel(table_name = strategy_order_state_changes)]
#[diesel(primary_key(id))]
#[diesel(belongs_to(StrategyOrder, foreign_key = order_id))]
pub struct StrategyOrderStateChange {
    pub id: Uuid,
    pub order_id: Uuid,
    pub previous_status: Option<OrderStatus>,
    pub new_status: OrderStatus,
    pub previous_quantity: Option<BigDecimal>,
    pub new_quantity: Option<BigDecimal>,
    pub change_reason: Option<String>,
    pub triggered_by: Option<String>,
    pub exchange_message: Option<String>,
    pub state_data: Option<serde_json::Value>,
    pub changed_at: DateTime<Utc>,
    pub changed_by: Option<String>,
}

// New Order State Change struct for inserts
#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = strategy_order_state_changes)]
pub struct NewStrategyOrderStateChange {
    pub order_id: Uuid,
    pub previous_status: Option<OrderStatus>,
    pub new_status: OrderStatus,
    pub previous_quantity: Option<BigDecimal>,
    pub new_quantity: Option<BigDecimal>,
    pub change_reason: Option<String>,
    pub triggered_by: Option<String>,
    pub exchange_message: Option<String>,
    pub state_data: Option<serde_json::Value>,
    pub changed_by: Option<String>,
}
