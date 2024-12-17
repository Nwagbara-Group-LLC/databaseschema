use std::sync::Arc;

use crate::{get_connection, models::{NewOrderBook, OrderBook}, CustomAsyncPgConnectionManager};
use bigdecimal::BigDecimal;
use deadpool::managed::Pool;
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use uuid::Uuid;

pub async fn create_orderbook(pool: Arc<Pool<CustomAsyncPgConnectionManager>>, orderbook: NewOrderBook) -> OrderBook {
    println!("Creating orderbook: {:?}", orderbook);
    use crate::schema::order_books::dsl::*;

    let mut connection = get_connection(pool)
        .await
        .expect("Error connecting to database");

    diesel::insert_into(order_books)
        .values(&orderbook)
        .returning(OrderBook::as_returning())
        .get_result(&mut connection)
        .await
        .expect("Error saving new orderbook")
}

pub async fn get_orderbooks(pool: Arc<Pool<CustomAsyncPgConnectionManager>>) -> Vec<OrderBook> {
    println!("Getting orderbooks");
    use crate::schema::order_books::dsl::*;

    let mut connection = get_connection(pool)
        .await
        .expect("Error connecting to database");
    order_books
        .load::<OrderBook>(&mut connection)
        .await
        .expect("Error loading orderbooks")
}

pub async fn update_orderbook(pool: Arc<Pool<CustomAsyncPgConnectionManager>>, orderbook: OrderBook, volume: BigDecimal) -> OrderBook {
    println!("Updating orderbook: {:?}", orderbook);
    use crate::schema::order_books::dsl::*;

    let mut connection = get_connection(pool)
        .await
        .expect("Error connecting to database");
    diesel::update(order_books.find(orderbook.order_book_id))
        .set(total_volume.eq(volume))
        .get_result(&mut connection)
        .await
        .expect("Error updating orderbook")
}

pub async fn get_orderbook_by_exchange_id(pool: Arc<Pool<CustomAsyncPgConnectionManager>>, xchange_id: &Uuid) -> OrderBook {
    println!("Getting orderbook");
    use crate::schema::order_books::dsl::*;

    let mut connection = get_connection(pool)
        .await
        .expect("Error connecting to database");
    order_books
        .filter(exchange_id.eq(xchange_id))
        .first::<OrderBook>(&mut connection)
        .await
        .expect("Error loading orderbook")
}

pub async fn orderbook_exists(pool: Arc<Pool<CustomAsyncPgConnectionManager>>, xchange_id: &Uuid) -> bool {
    println!("Checking if orderbook exists");
    use crate::schema::order_books::dsl::*;

    let mut connection = get_connection(pool)
        .await
        .expect("Error connecting to database");
    order_books
        .filter(exchange_id.eq(xchange_id))
        .first::<OrderBook>(&mut connection)
        .await
        .is_ok()
}
