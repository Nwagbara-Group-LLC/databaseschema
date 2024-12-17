use bigdecimal::BigDecimal;
use chrono::{DateTime, Utc};
use diesel::pg::{sql_types::Timestamptz, Pg};
use diesel::prelude::*;
use diesel::sql_types::{Nullable, Numeric, VarChar};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::schema::trades;
use crate::schema::securities;
use crate::schema::exchanges;
use crate::schema::order_books;
use crate::schema::open_buy_orders;
use crate::schema::open_buy_candlestick_agg;
use crate::schema::open_sell_orders;
use crate::schema::open_sell_candlestick_agg;
use crate::schema::modified_buy_orders;
use crate::schema::modified_buy_candlestick_agg;
use crate::schema::modified_sell_orders;
use crate::schema::modified_sell_candlestick_agg;

#[derive(Serialize, Deserialize, Debug, Queryable, Selectable, QueryableByName, AsChangeset)]
#[diesel(table_name = open_buy_candlestick_agg)]
#[diesel(check_for_backend(Pg))]
pub struct OpenBuyCandlestick {
    #[diesel(sql_type = Timestamptz)]
    pub bucket: DateTime<Utc>,
    #[diesel(sql_type = VarChar)]
    pub symbol: String,
    #[diesel(sql_type = VarChar)]
    pub exchange: String,
    #[diesel(sql_type = Numeric)]
    pub low_buy_price: BigDecimal,
    #[diesel(sql_type = Numeric)]
    pub high_buy_price: BigDecimal,
    #[diesel(sql_type = Numeric)]
    pub open_buy_price: BigDecimal,
    #[diesel(sql_type = Numeric)]
    pub close_buy_price: BigDecimal,
    #[diesel(sql_type = Numeric)]
    pub total_buy_volume: BigDecimal,
}

//--------------------------------------------------------------------------------------------

#[derive(Serialize, Deserialize, Debug, Queryable, Selectable, QueryableByName, AsChangeset)]
#[diesel(table_name = modified_buy_candlestick_agg)]
#[diesel(check_for_backend(Pg))]
pub struct ModifiedBuyCandlestick {
    #[diesel(sql_type = Timestamptz)]
    pub bucket: DateTime<Utc>,
    #[diesel(sql_type = VarChar)]
    pub symbol: String,
    #[diesel(sql_type = VarChar)]
    pub exchange: String,
    #[diesel(sql_type = Numeric)]
    pub low_buy_price: BigDecimal,
    #[diesel(sql_type = Numeric)]
    pub high_buy_price: BigDecimal,
    #[diesel(sql_type = Numeric)]
    pub open_buy_price: BigDecimal,
    #[diesel(sql_type = Numeric)]
    pub close_buy_price: BigDecimal,
    #[diesel(sql_type = Numeric)]
    pub total_buy_volume: BigDecimal,
}

//--------------------------------------------------------------------------------------------

#[derive(Serialize, Deserialize, Debug, Queryable, Selectable, QueryableByName, AsChangeset)]
#[diesel(table_name = open_sell_candlestick_agg)]
#[diesel(check_for_backend(Pg))]
pub struct OpenSellCandlestick {
    #[diesel(sql_type = Timestamptz)]
    pub bucket: DateTime<Utc>,
    #[diesel(sql_type = VarChar)]
    pub symbol: String,
    #[diesel(sql_type = VarChar)]
    pub exchange: String,
    #[diesel(sql_type = Numeric)]
    pub low_sell_price: BigDecimal,
    #[diesel(sql_type = Numeric)]
    pub high_sell_price: BigDecimal,
    #[diesel(sql_type = Numeric)]
    pub open_sell_price: BigDecimal,
    #[diesel(sql_type = Numeric)]
    pub close_sell_price: BigDecimal,
    #[diesel(sql_type = Numeric)]
    pub total_sell_volume: BigDecimal,
}

//--------------------------------------------------------------------------------------------

#[derive(Serialize, Deserialize, Debug, Queryable, Selectable, QueryableByName, AsChangeset)]
#[diesel(table_name = modified_sell_candlestick_agg)]
#[diesel(check_for_backend(Pg))]
pub struct ModifiedSellCandlestick {
    #[diesel(sql_type = Timestamptz)]
    pub bucket: DateTime<Utc>,
    #[diesel(sql_type = VarChar)]
    pub symbol: String,
    #[diesel(sql_type = VarChar)]
    pub exchange: String,
    #[diesel(sql_type = Numeric)]
    pub low_sell_price: BigDecimal,
    #[diesel(sql_type = Numeric)]
    pub high_sell_price: BigDecimal,
    #[diesel(sql_type = Numeric)]
    pub open_sell_price: BigDecimal,
    #[diesel(sql_type = Numeric)]
    pub close_sell_price: BigDecimal,
    #[diesel(sql_type = Numeric)]
    pub total_sell_volume: BigDecimal,
}

//--------------------------------------------------------------------------------------------

#[derive(Debug, Insertable)]
#[diesel(table_name = trades)]
pub struct NewTrade {
    pub symbol: String,
    pub exchange: String,
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
        security_id: Uuid,
        exchange_id: Uuid,
        side: &str,
        price: &BigDecimal,
        quantity: &BigDecimal,
    ) -> NewTrade {
        NewTrade {
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

#[derive(Serialize, Deserialize, Debug, Queryable, Selectable, QueryableByName, AsChangeset)]
#[diesel(table_name = trades)]
#[diesel(check_for_backend(Pg))]
pub struct Trade {
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

//--------------------------------------------------------------------------------------------

#[derive(Debug, Insertable)]
#[diesel(table_name = securities)]
pub struct NewSecurity {
    pub symbol: String,
}

impl NewSecurity {
    pub fn new(symbol: &String) -> NewSecurity {
        NewSecurity { symbol: symbol.to_string() }
    }
}

#[derive(Serialize, Deserialize, Debug, Queryable, Selectable, QueryableByName, AsChangeset)]
#[diesel(table_name = securities)]
#[diesel(check_for_backend(Pg))]
pub struct Security {
    #[diesel(sql_type = Timestamptz)]
    pub created_at: DateTime<Utc>,
    #[diesel(sql_type = diesel::sql_types::Uuid)]
    pub security_id: Uuid,
    #[diesel(sql_type = VarChar)]
    pub symbol: String,
}

//--------------------------------------------------------------------------------------------
#[derive(Debug, Insertable)]
#[diesel(table_name = exchanges)]
pub struct NewExchange {
    pub exchange: String,
}

impl NewExchange {
    pub fn new(exchange: &String) -> NewExchange {
        NewExchange { exchange: exchange.to_string() }
    }
}

#[derive(Serialize, Deserialize, Debug, Queryable, Selectable, QueryableByName, AsChangeset)]
#[diesel(table_name = exchanges)]
#[diesel(check_for_backend(Pg))]
pub struct Exchange {
    #[diesel(sql_type = Timestamptz)]
    pub created_at: DateTime<Utc>,
    #[diesel(sql_type = diesel::sql_types::Uuid)]
    pub exchange_id: Uuid,
    #[diesel(sql_type = VarChar)]
    pub exchange: String,
}
//--------------------------------------------------------------------------------------------
#[derive(Debug, Insertable)]
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
//--------------------------------------------------------------------------------------------
#[derive(Debug, Insertable)]
#[diesel(table_name = open_buy_orders)]
pub struct NewOpenBuyOrder {
    pub symbol: String,
    pub exchange: String,
    pub security_id: Uuid,
    pub exchange_id: Uuid,
    pub buy_order_book_id: Uuid,
    pub unique_id: String,
    pub price_level: BigDecimal,
    pub buy_quantity: BigDecimal,
}

impl NewOpenBuyOrder {
    pub fn new(
        symbol: &str,
        exchange: &str,
        security_id: Uuid,
        exchange_id: Uuid,
        buy_order_book_id: Uuid,
        unique_id: &str,
        price_level: &BigDecimal,
        buy_quantity: &BigDecimal,
    ) -> NewOpenBuyOrder {
        NewOpenBuyOrder {
            symbol: symbol.to_string(),
            exchange: exchange.to_string(),
            security_id,
            exchange_id,
            buy_order_book_id,
            unique_id: unique_id.to_string(),
            price_level: price_level.clone(),
            buy_quantity: buy_quantity.clone(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Queryable, Clone, Selectable, QueryableByName, AsChangeset)]
#[diesel(table_name = open_buy_orders)]
#[diesel(check_for_backend(Pg))]
pub struct OpenBuyOrder {
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
    pub buy_quantity: BigDecimal,
}
//--------------------------------------------------------------------------------------------
#[derive(Debug, Insertable)]
#[diesel(table_name = modified_buy_orders)]
pub struct NewModifiedBuyOrder {
    pub symbol: String,
    pub exchange: String,
    pub security_id: Uuid,
    pub exchange_id: Uuid,
    pub buy_order_book_id: Uuid,
    pub unique_id: String,
    pub price_level: BigDecimal,
    pub new_buy_quantity: BigDecimal,
}

impl NewModifiedBuyOrder {
    pub fn new(
        symbol: &str,
        exchange: &str,
        security_id: Uuid,
        exchange_id: Uuid,
        buy_order_book_id: Uuid,
        unique_id: &str,
        price_level: &BigDecimal,
        new_buy_quantity: &BigDecimal,
    ) -> NewModifiedBuyOrder {
        NewModifiedBuyOrder {
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
#[diesel(table_name = modified_buy_orders)]
#[diesel(check_for_backend(Pg))]
pub struct ModifiedBuyOrder {
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
//--------------------------------------------------------------------------------------------
#[derive(Debug, Insertable)]
#[diesel(table_name = open_sell_orders)]
pub struct NewOpenSellOrder {
    pub symbol: String,
    pub exchange: String,
    pub security_id: Uuid,
    pub exchange_id: Uuid,
    pub sell_order_book_id: Uuid,
    pub unique_id: String,
    pub price_level: BigDecimal,
    pub sell_quantity: BigDecimal,
}

impl NewOpenSellOrder {
    pub fn new(
        symbol: &str,
        exchange: &str,
        security_id: Uuid,
        exchange_id: Uuid,
        sell_order_book_id: Uuid,
        unique_id: &str,
        price_level: &BigDecimal,
        sell_quantity: &BigDecimal,
    ) -> NewOpenSellOrder {
        NewOpenSellOrder {
            symbol: symbol.to_string(),
            exchange: exchange.to_string(),
            security_id,
            exchange_id,
            sell_order_book_id,
            unique_id: unique_id.to_string(),
            price_level: price_level.clone(),
            sell_quantity: sell_quantity.clone(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Queryable, Clone, Selectable, QueryableByName, AsChangeset)]
#[diesel(table_name = open_sell_orders)]
#[diesel(check_for_backend(Pg))]
pub struct OpenSellOrder {
    #[diesel(sql_type = Timestamptz)]
    pub created_at: DateTime<Utc>,
    #[diesel(sql_type = VarChar)]
    pub symbol: String,
    #[diesel(sql_type = VarChar)]
    pub exchange: String,
    #[diesel(sql_type = diesel::sql_types::Uuid)]
    pub security_id: Uuid,
    #[diesel(sql_type = diesel::sql_types::Uuid)]
    pub sell_order_book_id: Uuid,
    #[diesel(sql_type = diesel::sql_types::Uuid)]
    pub exchange_id: Uuid,
    #[diesel(sql_type = VarChar)]
    pub unique_id: String,
    #[diesel(sql_type = Numeric)]
    pub price_level: BigDecimal,
    #[diesel(sql_type = Numeric)]
    pub sell_quantity: BigDecimal,
}
//--------------------------------------------------------------------------------------------
#[derive(Debug, Insertable)]
#[diesel(table_name = modified_sell_orders)]
pub struct NewModifiedSellOrder {
    pub symbol: String,
    pub exchange: String,
    pub security_id: Uuid,
    pub exchange_id: Uuid,
    pub sell_order_book_id: Uuid,
    pub unique_id: String,
    pub price_level: BigDecimal,
    pub new_sell_quantity: BigDecimal,
}

impl NewModifiedSellOrder {
    pub fn new(
        symbol: &str,
        exchange: &str,
        security_id: Uuid,
        exchange_id: Uuid,
        sell_order_book_id: Uuid,
        unique_id: &str,
        price_level: &BigDecimal,
        new_sell_quantity: &BigDecimal,
    ) -> NewModifiedSellOrder {
        NewModifiedSellOrder {
            symbol: symbol.to_string(),
            exchange: exchange.to_string(),
            security_id,
            exchange_id,
            sell_order_book_id,
            unique_id: unique_id.to_string(),
            price_level: price_level.clone(),
            new_sell_quantity: new_sell_quantity.clone(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Queryable, Clone, Selectable, QueryableByName, AsChangeset)]
#[diesel(table_name = modified_sell_orders)]
#[diesel(check_for_backend(Pg))]
pub struct ModifiedSellOrder {
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
    pub sell_order_book_id: Uuid,
    #[diesel(sql_type = VarChar)]
    pub unique_id: String,
    #[diesel(sql_type = Numeric)]
    pub price_level: BigDecimal,
    #[diesel(sql_type = Numeric)]
    pub new_sell_quantity: BigDecimal,
}