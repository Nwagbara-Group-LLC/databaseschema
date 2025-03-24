use bigdecimal::BigDecimal;
use chrono::{DateTime, Utc};
use diesel::pg::{sql_types::Timestamptz, Pg};
use diesel::prelude::*;
use diesel::sql_types::{Numeric, VarChar};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::schema::sim_open_buy_orders;

#[derive(Serialize, Deserialize, Debug, Queryable, Clone, Selectable, QueryableByName, AsChangeset)]
#[diesel(table_name = sim_open_buy_orders)]
#[diesel(check_for_backend(Pg))]
pub struct SimOpenBuyOrder {
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
    buy_quantity: BigDecimal,
    #[diesel(sql_type = diesel::sql_types::Uuid)]
    created_id: Uuid,
}

impl SimOpenBuyOrder {
    pub fn new(
        backtest_id: Uuid,
        created_at: DateTime<Utc>,
        symbol: &str,
        exchange: &str,
        unique_id: &str,
        price_level: &BigDecimal,
        buy_quantity: &BigDecimal,
        created_id: Option<Uuid>,
    ) -> SimOpenBuyOrder {
        SimOpenBuyOrder {
            backtest_id,
            created_at,
            symbol: symbol.to_string(),
            exchange: exchange.to_string(),
            unique_id: unique_id.to_string(),
            price_level: price_level.clone(),
            buy_quantity: buy_quantity.clone(),
            created_id: created_id.unwrap_or(Uuid::nil()),
        }
    }

    pub fn get_timestamp(&self) -> &DateTime<Utc> {
        &self.created_at
    }

    pub fn get_unique_id(&self) -> &str {
        &self.unique_id
    }

    pub fn get_created_id(&self) -> Option<Uuid> {
        Some(self.created_id)
    }

    pub fn get_exchange(&self) -> &str {
        &self.exchange
    }

    pub fn add_fee(&mut self, fee: &BigDecimal) {
        self.price_level += &self.price_level * fee;
    }

    pub fn get_price_level(&self) -> &BigDecimal {
        &self.price_level
    }

    pub fn get_symbol(&self) -> &str {
        &self.symbol
    }

    pub fn get_quantity(&self) -> &BigDecimal {
        &self.buy_quantity
    }
    
    pub fn set_quantity(&mut self, buy_quantity: &BigDecimal) {
        self.buy_quantity = buy_quantity.clone();
    }
}