use bigdecimal::BigDecimal;
use chrono::{DateTime, Utc};
use diesel::pg::{sql_types::Timestamptz, Pg};
use diesel::prelude::*;
use diesel::sql_types::{Numeric, VarChar};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::schema::sim_modified_buy_orders;

#[derive(Debug, Insertable, AsChangeset)]
#[diesel(table_name = sim_modified_buy_orders)]
pub struct NewSimModifiedBuyOrder {
    pub backtest_id: Uuid,
    pub symbol: String,
    pub exchange: String,
    pub security_id: Uuid,
    pub exchange_id: Uuid,
    pub buy_order_book_id: Uuid,
    pub unique_id: String,
    pub price_level: BigDecimal,
    pub new_buy_quantity: BigDecimal,
}

impl NewSimModifiedBuyOrder {
    pub fn new(
        backtest_id: Uuid,
        symbol: &str,
        exchange: &str,
        security_id: Uuid,
        exchange_id: Uuid,
        buy_order_book_id: Uuid,
        unique_id: &str,
        price_level: &BigDecimal,
        new_buy_quantity: &BigDecimal,
    ) -> NewSimModifiedBuyOrder {
        NewSimModifiedBuyOrder {
            backtest_id,
            symbol: symbol.to_string(),
            exchange: exchange.to_string(),
            security_id,
            exchange_id,
            buy_order_book_id,
            unique_id: unique_id.to_string(),
            price_level: price_level.clone(),
            new_buy_quantity: new_buy_quantity.clone(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Queryable, Clone, Selectable, QueryableByName, AsChangeset)]
#[diesel(table_name = sim_modified_buy_orders)]
#[diesel(check_for_backend(Pg))]
pub struct SimModifiedBuyOrder {
    #[diesel(sql_type = diesel::sql_types::Uuid)]
    pub backtest_id: Uuid,
    #[diesel(sql_type = Timestamptz)]
    pub created_at: DateTime<Utc>,
    #[diesel(sql_type = VarChar)]
    pub symbol: String,
    #[diesel(sql_type = VarChar)]
    pub exchange: String,
    #[diesel(sql_type = diesel::sql_types::Uuid)]
    pub security_id: Uuid,
    #[diesel(sql_type = diesel::sql_types::Uuid)]
    pub exchange_id: Uuid,
    #[diesel(sql_type = diesel::sql_types::Uuid)]
    pub buy_order_book_id: Uuid,
    #[diesel(sql_type = VarChar)]
    pub unique_id: String,
    #[diesel(sql_type = Numeric)]
    pub price_level: BigDecimal,
    #[diesel(sql_type = Numeric)]
    pub new_buy_quantity: BigDecimal,
}