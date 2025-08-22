use bigdecimal::BigDecimal;
use chrono::{DateTime, Utc};
use diesel::pg::{sql_types::Timestamptz, Pg};
use diesel::prelude::*;
use diesel::sql_types::{Numeric, VarChar};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::schema::historical_snapshot;

#[derive(Debug, Insertable, AsChangeset)]
#[diesel(table_name = historical_snapshot)]
pub struct NewHistoricalSnapshot {
    timestamp: DateTime<Utc>,
    pub order_id: String,
    event_type: String,
    side: String,
    price_level: BigDecimal,
    quantity: BigDecimal,
    status: String,
    exchange: String,
    symbol: String
}

impl NewHistoricalSnapshot {
    pub fn new(
        timestamp: DateTime<Utc>,
        order_id: &str,
        event_type: &str,
        side: &str,
        price_level: &BigDecimal,
        quantity: &BigDecimal,
        status: &str,
        exchange: &str,
        symbol: &str
    ) -> NewHistoricalSnapshot {
        NewHistoricalSnapshot {
            timestamp,
            order_id: order_id.to_string(),
            event_type: event_type.to_string(),
            side: side.to_string(),
            price_level: price_level.clone(),
            quantity: quantity.clone(),
            status: status.to_string(),
            exchange: exchange.to_string(),
            symbol: symbol.to_string()
        }
    }
}

#[derive(Clone, Serialize, Deserialize, Debug, Queryable, Selectable, QueryableByName, AsChangeset)]
#[diesel(table_name = historical_snapshot)]
#[diesel(check_for_backend(Pg))]
pub struct HistoricalSnapshot {
    #[diesel(sql_type = diesel::sql_types::Uuid)]
    pub event_id: Uuid,
    #[diesel(sql_type = Timestamptz)]
    pub timestamp: DateTime<Utc>,
    #[diesel(sql_type = VarChar)]
    pub order_id: String,
    #[diesel(sql_type = VarChar)]
    pub event_type: String,
    #[diesel(sql_type = VarChar)]
    pub side: String,
    #[diesel(sql_type = Numeric)]
    pub price_level: BigDecimal,
    #[diesel(sql_type = Numeric)]
    pub quantity: BigDecimal,
    #[diesel(sql_type = VarChar)]
    pub status: String,
    #[diesel(sql_type = VarChar)]
    pub exchange: String,
    #[diesel(sql_type = VarChar)]
    pub symbol: String,
    #[diesel(sql_type = diesel::sql_types::Uuid)]
    pub security_id: Uuid,
    #[diesel(sql_type = diesel::sql_types::Uuid)]
    pub exchange_id: Uuid,
}

impl HistoricalSnapshot {
    pub fn get_timestamp(&self) -> DateTime<Utc> {
        self.timestamp
    }

    pub fn get_order_id(&self) -> &str {
        &self.order_id
    }

    pub fn get_event_type(&self) -> &str {
        &self.event_type
    }

    pub fn get_side(&self) -> &str {
        &self.side
    }

    pub fn get_symbol(&self) -> &str {
        &self.symbol
    }

    pub fn get_exchange(&self) -> &str {
        &self.exchange
    }

    pub fn get_security_id(&self) -> Uuid {
        self.security_id
    }

    pub fn get_exchange_id(&self) -> Uuid {
        self.exchange_id
    }

    pub fn get_price_level(&self) -> &BigDecimal {
        &self.price_level
    }

    pub fn get_quantity(&self) -> &BigDecimal {
        &self.quantity
    }
}