use bigdecimal::BigDecimal;
use chrono::{DateTime, Utc};
use diesel::pg::{sql_types::Timestamptz, Pg};
use diesel::prelude::*;
use diesel::sql_types::{Numeric, VarChar};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::schema::sim_open_sell_orders;

#[derive(Serialize, Deserialize, Debug, Queryable, Clone, Selectable, QueryableByName, AsChangeset)]
#[diesel(table_name = sim_open_sell_orders)]
#[diesel(check_for_backend(Pg))]
pub struct SimOpenSellOrder {
    #[diesel(sql_type = diesel::sql_types::Uuid)]
    backtest_id: Uuid,
    #[diesel(sql_type = Timestamptz)]
    created_at: DateTime<Utc>,
    #[diesel(sql_type = VarChar)]
    symbol: String,
    #[diesel(sql_type = VarChar)]
    exchange: String,
    #[diesel(sql_type = VarChar)]
    unique_id: String,
    #[diesel(sql_type = Numeric)]
    price_level: BigDecimal,
    #[diesel(sql_type = Numeric)]
    sell_quantity: BigDecimal,
    #[diesel(sql_type = diesel::sql_types::Uuid)]
    created_id: Uuid,
}

impl SimOpenSellOrder {
    pub fn new(
        backtest_id: Uuid,
        created_at: DateTime<Utc>,
        symbol: &str,
        exchange: &str,
        unique_id: &str,
        price_level: &BigDecimal,
        sell_quantity: &BigDecimal,
        created_id: Option<Uuid>,
    ) -> SimOpenSellOrder {
        SimOpenSellOrder {
            backtest_id,
            created_at,
            symbol: symbol.to_string(),
            exchange: exchange.to_string(),
            unique_id: unique_id.to_string(),
            price_level: price_level.clone(),
            sell_quantity: sell_quantity.clone(),
            created_id: created_id.unwrap_or(Uuid::nil()),
        }
    }

    pub fn get_timestamp(&self) -> DateTime<Utc> {
        self.created_at
    }

    pub fn get_unique_id(&self) -> String {
        self.unique_id.clone()
    }

    pub fn get_created_id(&self) -> Option<Uuid> {
        Some(self.created_id)
    }

    pub fn get_exchange(&self) -> String {
        self.exchange.clone()
    }

    pub fn add_fee(&mut self, fee: &BigDecimal) {
        self.price_level -= &self.price_level * fee;
    }

    pub fn get_price_level(&self) -> BigDecimal {
        self.price_level.clone()
    }

    pub fn get_symbol(&self) -> String {
        self.symbol.clone()
    }

    pub fn get_quantity(&self) -> BigDecimal {
        self.sell_quantity.clone()
    }

    pub fn set_quantity(&mut self, sell_quantity: &BigDecimal) {
        self.sell_quantity = sell_quantity.clone();
    }
}