use bigdecimal::BigDecimal;
use chrono::{DateTime, Utc};
use diesel::pg::{sql_types::Timestamptz, Pg};
use diesel::prelude::*;
use diesel::sql_types::{Numeric, VarChar};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::schema::trades;

#[derive(Serialize, Deserialize, Debug, Insertable, Queryable, Clone, Selectable, QueryableByName, AsChangeset)]
#[diesel(table_name = trades)]
pub struct NewTrade {
    pub symbol: String,
    pub exchange: String,
    pub trade_id: String,
    pub security_id: Uuid,
    pub exchange_id: Uuid,
    pub side: String,
    pub price: BigDecimal,
    pub quantity: BigDecimal,
}

impl NewTrade {
    pub fn new(
        symbol: &str,
        exchange: &str,
        trade_id: &str,
        security_id: Uuid,
        exchange_id: Uuid,
        side: &str,
        price: &BigDecimal,
        quantity: &BigDecimal,
    ) -> NewTrade {
        NewTrade {
            symbol: symbol.to_string(),
            exchange: exchange.to_string(),
            trade_id: trade_id.to_string(),
            security_id,
            exchange_id,
            side: side.to_string(),
            price: price.clone(),
            quantity: quantity.clone(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize, Debug, Queryable, Selectable, QueryableByName, AsChangeset)]
#[diesel(table_name = trades)]
#[diesel(check_for_backend(Pg))]
pub struct Trade {
    #[diesel(sql_type = Timestamptz)]
    pub created_at: DateTime<Utc>,

    #[diesel(sql_type = VarChar)]
    pub trade_id: String,

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

impl Trade {
    pub fn get_timestamp(&self) -> DateTime<Utc> {
        self.created_at.clone()
    }

    pub fn get_symbol(&self) -> &str {
        &self.symbol
    }

    pub fn get_exchange(&self) -> &str {
        &self.exchange
    }

    pub fn get_trade_id(&self) -> &str {
        &self.trade_id
    }

    pub fn get_side(&self) -> &str {
        &self.side
    }

    pub fn get_price(&self) -> &BigDecimal {
        &self.price
    }

    pub fn get_quantity(&self) -> &BigDecimal {
        &self.quantity
    }
}