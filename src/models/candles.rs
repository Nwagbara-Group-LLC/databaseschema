use chrono::{DateTime, Utc};
use bigdecimal::BigDecimal;
use diesel::{Queryable, Selectable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Model for 1-minute candles from continuous aggregates
#[derive(Clone, Serialize, Deserialize, Debug, Queryable, Selectable)]
#[diesel(table_name = crate::schema::candles)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Candle1m {
    pub timestamp: DateTime<Utc>,
    pub symbol: String,
    pub exchange: String,
    pub security_id: Uuid,
    pub exchange_id: Uuid,
    pub open_price: BigDecimal,
    pub high_price: BigDecimal,
    pub low_price: BigDecimal,
    pub close_price: BigDecimal,
    pub volume: BigDecimal,
    pub trade_count: i32,
    pub timeframe: String,
    pub created_at: DateTime<Utc>,
}

/// Model for 5-minute candles from continuous aggregates
#[derive(Clone, Serialize, Deserialize, Debug, Queryable, Selectable)]
#[diesel(table_name = crate::schema::candles)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Candle5m {
    pub timestamp: DateTime<Utc>,
    pub symbol: String,
    pub exchange: String,
    pub security_id: Uuid,
    pub exchange_id: Uuid,
    pub open_price: BigDecimal,
    pub high_price: BigDecimal,
    pub low_price: BigDecimal,
    pub close_price: BigDecimal,
    pub volume: BigDecimal,
    pub trade_count: i32,
    pub timeframe: String,
    pub created_at: DateTime<Utc>,
}

/// Model for 15-minute candles from continuous aggregates
#[derive(Clone, Serialize, Deserialize, Debug, Queryable, Selectable)]
#[diesel(table_name = crate::schema::candles)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Candle15m {
    pub timestamp: DateTime<Utc>,
    pub symbol: String,
    pub exchange: String,
    pub security_id: Uuid,
    pub exchange_id: Uuid,
    pub open_price: BigDecimal,
    pub high_price: BigDecimal,
    pub low_price: BigDecimal,
    pub close_price: BigDecimal,
    pub volume: BigDecimal,
    pub trade_count: i32,
    pub timeframe: String,
    pub created_at: DateTime<Utc>,
}

/// Model for 1-hour candles from continuous aggregates
#[derive(Clone, Serialize, Deserialize, Debug, Queryable, Selectable)]
#[diesel(table_name = crate::schema::candles)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Candle1h {
    pub timestamp: DateTime<Utc>,
    pub symbol: String,
    pub exchange: String,
    pub security_id: Uuid,
    pub exchange_id: Uuid,
    pub open_price: BigDecimal,
    pub high_price: BigDecimal,
    pub low_price: BigDecimal,
    pub close_price: BigDecimal,
    pub volume: BigDecimal,
    pub trade_count: i32,
    pub timeframe: String,
    pub created_at: DateTime<Utc>,
}

/// Model for 1-day candles from continuous aggregates
#[derive(Clone, Serialize, Deserialize, Debug, Queryable, Selectable)]
#[diesel(table_name = crate::schema::candles)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Candle1d {
    pub timestamp: DateTime<Utc>,
    pub symbol: String,
    pub exchange: String,
    pub security_id: Uuid,
    pub exchange_id: Uuid,
    pub open_price: BigDecimal,
    pub high_price: BigDecimal,
    pub low_price: BigDecimal,
    pub close_price: BigDecimal,
    pub volume: BigDecimal,
    pub trade_count: i32,
    pub timeframe: String,
    pub created_at: DateTime<Utc>,
}

/// Generic trait for candle data to enable unified handling
pub trait CandleData {
    fn timestamp(&self) -> DateTime<Utc>;
    fn symbol(&self) -> &str;
    fn exchange(&self) -> &str;
    fn open_price(&self) -> &BigDecimal;
    fn high_price(&self) -> &BigDecimal;
    fn low_price(&self) -> &BigDecimal;
    fn close_price(&self) -> &BigDecimal;
    fn volume(&self) -> &BigDecimal;
    fn trade_count(&self) -> i32;
}

// Implement CandleData trait for all candle types
impl CandleData for Candle1m {
    fn timestamp(&self) -> DateTime<Utc> { self.timestamp }
    fn symbol(&self) -> &str { &self.symbol }
    fn exchange(&self) -> &str { &self.exchange }
    fn open_price(&self) -> &BigDecimal { &self.open_price }
    fn high_price(&self) -> &BigDecimal { &self.high_price }
    fn low_price(&self) -> &BigDecimal { &self.low_price }
    fn close_price(&self) -> &BigDecimal { &self.close_price }
    fn volume(&self) -> &BigDecimal { &self.volume }
    fn trade_count(&self) -> i32 { self.trade_count }
}

impl CandleData for Candle5m {
    fn timestamp(&self) -> DateTime<Utc> { self.timestamp }
    fn symbol(&self) -> &str { &self.symbol }
    fn exchange(&self) -> &str { &self.exchange }
    fn open_price(&self) -> &BigDecimal { &self.open_price }
    fn high_price(&self) -> &BigDecimal { &self.high_price }
    fn low_price(&self) -> &BigDecimal { &self.low_price }
    fn close_price(&self) -> &BigDecimal { &self.close_price }
    fn volume(&self) -> &BigDecimal { &self.volume }
    fn trade_count(&self) -> i32 { self.trade_count }
}

impl CandleData for Candle15m {
    fn timestamp(&self) -> DateTime<Utc> { self.timestamp }
    fn symbol(&self) -> &str { &self.symbol }
    fn exchange(&self) -> &str { &self.exchange }
    fn open_price(&self) -> &BigDecimal { &self.open_price }
    fn high_price(&self) -> &BigDecimal { &self.high_price }
    fn low_price(&self) -> &BigDecimal { &self.low_price }
    fn close_price(&self) -> &BigDecimal { &self.close_price }
    fn volume(&self) -> &BigDecimal { &self.volume }
    fn trade_count(&self) -> i32 { self.trade_count }
}

impl CandleData for Candle1h {
    fn timestamp(&self) -> DateTime<Utc> { self.timestamp }
    fn symbol(&self) -> &str { &self.symbol }
    fn exchange(&self) -> &str { &self.exchange }
    fn open_price(&self) -> &BigDecimal { &self.open_price }
    fn high_price(&self) -> &BigDecimal { &self.high_price }
    fn low_price(&self) -> &BigDecimal { &self.low_price }
    fn close_price(&self) -> &BigDecimal { &self.close_price }
    fn volume(&self) -> &BigDecimal { &self.volume }
    fn trade_count(&self) -> i32 { self.trade_count }
}

impl CandleData for Candle1d {
    fn timestamp(&self) -> DateTime<Utc> { self.timestamp }
    fn symbol(&self) -> &str { &self.symbol }
    fn exchange(&self) -> &str { &self.exchange }
    fn open_price(&self) -> &BigDecimal { &self.open_price }
    fn high_price(&self) -> &BigDecimal { &self.high_price }
    fn low_price(&self) -> &BigDecimal { &self.low_price }
    fn close_price(&self) -> &BigDecimal { &self.close_price }
    fn volume(&self) -> &BigDecimal { &self.volume }
    fn trade_count(&self) -> i32 { self.trade_count }
}

/// Unified Candle model for the main candles table with timeframe
#[derive(Clone, Serialize, Deserialize, Debug, Queryable, Selectable)]
#[diesel(table_name = crate::schema::candles)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Candle {
    pub timestamp: DateTime<Utc>,
    pub symbol: String,
    pub exchange: String,
    pub security_id: Uuid,
    pub exchange_id: Uuid,
    pub open_price: BigDecimal,
    pub high_price: BigDecimal,
    pub low_price: BigDecimal,
    pub close_price: BigDecimal,
    pub volume: BigDecimal,
    pub trade_count: i32,
    pub timeframe: String,
    pub created_at: DateTime<Utc>,
}

impl CandleData for Candle {
    fn timestamp(&self) -> DateTime<Utc> { self.timestamp }
    fn symbol(&self) -> &str { &self.symbol }
    fn exchange(&self) -> &str { &self.exchange }
    fn open_price(&self) -> &BigDecimal { &self.open_price }
    fn high_price(&self) -> &BigDecimal { &self.high_price }
    fn low_price(&self) -> &BigDecimal { &self.low_price }
    fn close_price(&self) -> &BigDecimal { &self.close_price }
    fn volume(&self) -> &BigDecimal { &self.volume }
    fn trade_count(&self) -> i32 { self.trade_count }
}
