use bigdecimal::BigDecimal;
use chrono::{DateTime, Utc};
use diesel::pg::{sql_types::Timestamptz, Pg};
use diesel::prelude::*;
use diesel::sql_types::{Numeric, VarChar};
use serde::{Deserialize, Serialize};

use crate::schema::open_sell_candlestick_agg;

#[derive(Serialize, Deserialize, Debug, Queryable, Selectable, QueryableByName, AsChangeset)]
#[diesel(table_name = open_sell_candlestick_agg)]
#[diesel(check_for_backend(Pg))]
pub struct OpenSellCandlestick {
    #[diesel(sql_type = Timestamptz)]
    pub bucket: DateTime<Utc>,
    #[diesel(sql_type = VarChar)]
    pub symbol: String,
    #[diesel(sql_type = VarChar)]
    pub exchange: String,
    #[diesel(sql_type = Numeric)]
    pub low: BigDecimal,
    #[diesel(sql_type = Numeric)]
    pub high: BigDecimal,
    #[diesel(sql_type = Numeric)]
    pub open: BigDecimal,
    #[diesel(sql_type = Numeric)]
    pub close: BigDecimal,
    #[diesel(sql_type = Numeric)]
    pub volume: BigDecimal,
}