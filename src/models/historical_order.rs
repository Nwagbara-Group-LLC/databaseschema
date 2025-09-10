use bigdecimal::BigDecimal;
use chrono::{DateTime, Utc};
use diesel::pg::{sql_types::Timestamptz, Pg};
use diesel::prelude::*;
use diesel::sql_types::{Nullable, Numeric, VarChar};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::schema::historical_orders;

#[derive(Debug, Insertable, AsChangeset)]
#[diesel(table_name = historical_orders)]
pub struct NewHistoricalOrder {
    timestamp: DateTime<Utc>,
    order_id: String,
    event_type: String,
    side: String,
    price_level: BigDecimal,
    quantity: BigDecimal,
    prev_price: Option<BigDecimal>,
    prev_quantity: Option<BigDecimal>,
    status: String,
    exchange: String,
    symbol: String,
    exchange_id: Uuid,
    security_id: Uuid,
}

impl NewHistoricalOrder {
    pub fn new(
        timestamp: DateTime<Utc>,
        order_id: &str,
        event_type: &str,
        side: &str,
        price_level: &BigDecimal,
        quantity: &BigDecimal,
        prev_price: Option<&BigDecimal>,
        prev_quantity: Option<&BigDecimal>,
        status: &str,
        exchange: &str,
        symbol: &str,
        exchange_id: Uuid,
        security_id: Uuid,
    ) -> NewHistoricalOrder {
        NewHistoricalOrder {
            timestamp,
            order_id: order_id.to_string(),
            event_type: event_type.to_string(),
            side: side.to_string(),
            price_level: price_level.clone(),
            quantity: quantity.clone(),
            prev_price: prev_price.cloned(),
            prev_quantity: prev_quantity.cloned(),
            status: status.to_string(),
            exchange: exchange.to_string(),
            symbol: symbol.to_string(),
            exchange_id,
            security_id,
        }
    }

    pub fn get_order_id(&self) -> String {
        self.order_id.clone()
    }
}

#[derive(Clone, Serialize, Deserialize, Debug, Queryable, Selectable, QueryableByName, AsChangeset)]
#[diesel(table_name = historical_orders)]
#[diesel(check_for_backend(Pg))]
pub struct HistoricalOrder {
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
    #[diesel(sql_type = Nullable<Numeric>)]
    pub prev_price: Option<BigDecimal>,
    #[diesel(sql_type = Nullable<Numeric>)]
    pub prev_quantity: Option<BigDecimal>,
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

impl HistoricalOrder {
    pub fn get_timestamp(&self) -> &DateTime<Utc> {
        &self.timestamp
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