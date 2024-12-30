use std::sync::Arc;

use crate::{get_connection, models::orderbook::{NewOrderBook, OrderBook}, CustomAsyncPgConnectionManager};
use bigdecimal::BigDecimal;
use deadpool::managed::Pool;
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use tokio_retry::{strategy::FixedInterval, Retry};
use uuid::Uuid;

pub async fn create_orderbook(pool: Arc<Pool<CustomAsyncPgConnectionManager>>, orderbook: NewOrderBook) -> OrderBook {
    println!("Creating orderbook: {:?}", orderbook);
    use crate::schema::order_books::dsl::*;

    let retry_strategy = FixedInterval::from_millis(1).take(15);

    Retry::spawn(retry_strategy, || async {
        let mut connection = get_connection(pool.clone())
        .await
        .expect("Error connecting to database");
    diesel::insert_into(order_books)
        .values(&orderbook)
        .returning(OrderBook::as_returning())
        .get_result(&mut connection)
        .await
        .map_err(|e| {
            eprintln!("Error saving new orderbook: {}", e);
            e
        })
    })
        .await
        .expect("Error creating orderbook")
}

pub async fn get_orderbooks(pool: Arc<Pool<CustomAsyncPgConnectionManager>>) -> Vec<OrderBook> {
    println!("Getting orderbooks");
    use crate::schema::order_books::dsl::*;

    let retry_strategy = FixedInterval::from_millis(1).take(15);

    Retry::spawn(retry_strategy, || async {
        let mut connection = get_connection(pool.clone())
        .await
        .expect("Error connecting to database");
    order_books
        .load::<OrderBook>(&mut connection)
        .await
        .map_err(|e| {
            eprintln!("Error loading orderbooks: {}", e);
            e
        })
    }).await.expect("Error getting orderbooks")
}

pub async fn update_orderbook(pool: Arc<Pool<CustomAsyncPgConnectionManager>>, orderbook: OrderBook, volume: BigDecimal) -> OrderBook {
    println!("Updating orderbook: {:?}", orderbook);
    use crate::schema::order_books::dsl::*;

    let retry_strategy = FixedInterval::from_millis(1).take(15);

    Retry::spawn(retry_strategy, || async {
        let mut connection = get_connection(pool.clone())
            .await
            .expect("Error connecting to database");
        diesel::update(order_books.find(orderbook.order_book_id))
            .set(total_volume.eq(volume.clone()))
            .get_result(&mut connection)
            .await
            .map_err(|e| {
                eprintln!("Error updating orderbook: {}", e);
                e
            })
    })
    .await.expect("Error updating orderbook")
}

pub async fn get_orderbook_by_exchange_id(pool: Arc<Pool<CustomAsyncPgConnectionManager>>, xchange_id: &Uuid) -> OrderBook {
    println!("Getting orderbook");
    use crate::schema::order_books::dsl::*;

    let retry_strategy = FixedInterval::from_millis(1).take(15);

    Retry::spawn(retry_strategy, || async {
        let mut connection = get_connection(pool.clone())
        .await
        .expect("Error connecting to database");
    order_books
        .filter(exchange_id.eq(xchange_id))
        .first::<OrderBook>(&mut connection)
        .await
        .map_err(|e| {
            eprintln!("Error loading orderbook: {}", e);
            e
        })
    }).await.expect("Error getting orderbook")
}

pub async fn orderbook_exists(pool: Arc<Pool<CustomAsyncPgConnectionManager>>, xchange_id: &Uuid) -> bool {
    println!("Checking if orderbook exists");
    use crate::schema::order_books::dsl::*;

    let retry_strategy = FixedInterval::from_millis(1).take(15);

    Retry::spawn(retry_strategy, || async {
        let mut connection = get_connection(pool.clone())
        .await
        .expect("Error connecting to database");
    order_books
        .filter(exchange_id.eq(xchange_id))
        .first::<OrderBook>(&mut connection)
        .await
        .map_err(|e| {
            eprintln!("Error loading orderbook: {}", e);
            e
        })
    }).await.is_ok()
}
