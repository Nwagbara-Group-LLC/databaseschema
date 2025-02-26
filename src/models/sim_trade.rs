use bigdecimal::BigDecimal;
use chrono::{DateTime, Utc};
use diesel::pg::{sql_types::Timestamptz, Pg};
use diesel::prelude::*;
use diesel::sql_types::{Numeric, VarChar};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::schema::sim_trades;

#[derive(Serialize, Deserialize, Debug, Insertable, Queryable, Clone, Selectable, QueryableByName, AsChangeset)]
#[diesel(table_name = sim_trades)]
pub struct NewSimTrade {
    pub backtest_id: Uuid,
    pub symbol: String,
    pub exchange: String,
    pub security_id: Uuid,
    pub exchange_id: Uuid,
    pub side: String,
    pub price: BigDecimal,
    pub quantity: BigDecimal,
}

impl NewSimTrade {
    pub fn new(
        backtest_id: Uuid,
        symbol: &str,
        exchange: &str,
        security_id: Uuid,
        exchange_id: Uuid,
        side: &str,
        price: &BigDecimal,
        quantity: &BigDecimal,
    ) -> NewSimTrade {
        NewSimTrade {
            backtest_id,
            symbol: symbol.to_string(),
            exchange: exchange.to_string(),
            security_id,
            exchange_id,
            side: side.to_string(),
            price: price.clone(),
            quantity: quantity.clone(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize, Debug, Queryable, Selectable, QueryableByName, AsChangeset)]
#[diesel(table_name = sim_trades)]
#[diesel(check_for_backend(Pg))]
pub struct SimTrade {
    #[diesel(sql_type = diesel::sql_types::Uuid)]
    pub backtest_id: Uuid,
    #[diesel(sql_type = Timestamptz)]
    pub created_at: DateTime<Utc>,
    #[diesel(sql_type = diesel::sql_types::Uuid)]
    pub trade_id: Uuid,
    #[diesel(sql_type = VarChar)]
    pub symbol: String,
    #[diesel(sql_type = VarChar)]
    pub exchange: String,
    #[diesel(sql_type = diesel::sql_types::Uuid)]
    pub security_id: Uuid,
    #[diesel(sql_type = diesel::sql_types::Uuid)]
    pub exchange_id: Uuid,
    #[diesel(sql_type = VarChar)]
    pub side: String,
    #[diesel(sql_type = Numeric)]
    pub price: BigDecimal,
    #[diesel(sql_type = Numeric)]
    pub quantity: BigDecimal,
}

impl SimTrade {
    pub fn get_price(&self) -> &BigDecimal {
        &self.price
    }

    pub fn get_quantity(&self) -> &BigDecimal {
        &self.quantity
    }

    pub fn get_side(&self) -> &str {
        &self.side
    }

    pub fn get_symbol(&self) -> String {
        self.symbol.clone()
    }
}