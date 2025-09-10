use bigdecimal::BigDecimal;
use chrono::{DateTime, Utc};
use diesel::pg::{sql_types::Timestamptz, Pg};
use diesel::prelude::*;
use diesel::sql_types::{Numeric, VarChar};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::schema::sim_trades;

#[derive(Clone, Serialize, Deserialize, Debug, Queryable, Selectable, QueryableByName, AsChangeset)]
#[diesel(table_name = sim_trades)]
#[diesel(check_for_backend(Pg))]
pub struct SimTrade {
    #[diesel(sql_type = diesel::sql_types::Uuid)]
    backtest_id: Uuid,
    #[diesel(sql_type = Timestamptz)]
    created_at: DateTime<Utc>,
    #[diesel(sql_type = VarChar)]
    trade_id: String,
    #[diesel(sql_type = VarChar)]
    symbol: String,
    #[diesel(sql_type = VarChar)]
    exchange: String,
    #[diesel(sql_type = VarChar)]
    side: String,
    #[diesel(sql_type = Numeric)]
    price: BigDecimal,
    #[diesel(sql_type = Numeric)]
    quantity: BigDecimal,
    #[diesel(sql_type = diesel::sql_types::Bool)]
    matched_trader: bool,
}

#[derive(Serialize, Deserialize, Debug, Insertable)]
#[diesel(table_name = sim_trades)]
pub struct NewSimTrade {
    pub backtest_id: Uuid,
    pub trade_id: String,
    pub symbol: String,
    pub exchange: String,
    pub side: String,
    pub price: BigDecimal,
    pub quantity: BigDecimal,
    pub matched_trader: bool,
}

impl SimTrade {
    pub fn new(
        created_at: DateTime<Utc>,
        backtest_id: Uuid,
        trade_id: &str,
        symbol: &str,
        exchange: &str,
        side: &str,
        price: &BigDecimal,
        quantity: &BigDecimal,
        matched_trader: bool,
    ) -> SimTrade {
        SimTrade {
            created_at,
            backtest_id,
            trade_id: trade_id.to_string(),
            symbol: symbol.to_string(),
            exchange: exchange.to_string(),
            side: side.to_string(),
            price: price.clone(),
            quantity: quantity.clone(),
            matched_trader,
        }
    }

    pub fn get_timestamp(&self) -> &DateTime<Utc> {
        &self.created_at
    }

    pub fn get_trade_id(&self) -> &str {
        &self.trade_id
    }

    pub fn get_price_level(&self) -> &BigDecimal {
        &self.price
    }

    pub fn get_quantity(&self) -> &BigDecimal {
        &self.quantity
    }

    pub fn get_side(&self) -> &str {
        &self.side
    }

    pub fn get_symbol(&self) -> &str {
        &self.symbol
    }

    pub fn get_matched_trader(&self) -> bool {
        self.matched_trader
    }
}