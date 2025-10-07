use bigdecimal::BigDecimal;
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use diesel::pg::Pg;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::schema::order_books;

#[derive(Debug, Insertable, AsChangeset)]
#[diesel(table_name = order_books)]
pub struct NewOrderBook {
    pub symbol: String,
    pub exchange: String,
    pub security_id: Uuid,
    pub exchange_id: Uuid,
}

impl NewOrderBook {
    pub fn new(symbol: &String, exchange: &String, security_id: Uuid, exchange_id: Uuid) -> NewOrderBook {
        NewOrderBook {
            symbol: symbol.to_string(),
            exchange: exchange.to_string(),
            security_id,
            exchange_id,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Queryable, Clone, Selectable, AsChangeset)]
#[diesel(table_name = order_books)]
#[diesel(check_for_backend(Pg))]
pub struct OrderBook {
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
    pub symbol: String,
    pub exchange: String,
    pub security_id: Uuid,
    pub exchange_id: Uuid,
    pub order_book_id: Uuid,
    pub buy_order_book_id: Uuid,
    pub sell_order_book_id: Uuid,
    pub total_volume: BigDecimal,
}