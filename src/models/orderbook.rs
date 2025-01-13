use bigdecimal::BigDecimal;
use chrono::{DateTime, Utc};
use diesel::pg::{sql_types::Timestamptz, Pg};
use diesel::prelude::*;
use diesel::sql_types::{Nullable, Numeric, VarChar};
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

#[derive(Serialize, Deserialize, Debug, Queryable, Clone, Selectable, QueryableByName, AsChangeset)]
#[diesel(table_name = order_books)]
#[diesel(check_for_backend(Pg))]
pub struct OrderBook {
    #[diesel(sql_type = Timestamptz)]
    pub created_at: DateTime<Utc>,
    #[diesel(sql_type = Nullable<Timestamptz>)]
    pub updated_at: Option<DateTime<Utc>>,
    #[diesel(sql_type = VarChar)]
    pub symbol: String,
    #[diesel(sql_type = VarChar)]
    pub exchange: String,
    #[diesel(sql_type = diesel::sql_types::Uuid)]
    pub security_id: Uuid,
    #[diesel(sql_type = diesel::sql_types::Uuid)]
    pub exchange_id: Uuid,
    #[diesel(sql_type = diesel::sql_types::Uuid)]
    pub order_book_id: Uuid,
    #[diesel(sql_type = diesel::sql_types::Uuid)]
    pub buy_order_book_id: Uuid,
    #[diesel(sql_type = diesel::sql_types::Uuid)]
    pub sell_order_book_id: Uuid,
    #[diesel(sql_type = Numeric)]
    pub total_volume: BigDecimal,
}