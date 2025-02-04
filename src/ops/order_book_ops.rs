use std::sync::Arc;
use crate::{get_timescale_connection, models::orderbook::{NewOrderBook, OrderBook}, CustomAsyncPgConnectionManager};
use bigdecimal::BigDecimal;
use deadpool::managed::Pool;
use diesel::{prelude::*, result::Error};
use diesel_async::RunQueryDsl;
use tokio_retry::{strategy::{jitter, ExponentialBackoff}, Retry};
use uuid::Uuid;

pub async fn create_orderbook(pool: Arc<Pool<CustomAsyncPgConnectionManager>>, orderbook: NewOrderBook) -> Result<OrderBook, Error> {
    println!("Creating orderbook: {:?}", orderbook);
    use crate::schema::order_books::dsl::*;

    let retry_strategy = ExponentialBackoff::from_millis(10).map(jitter).take(3);

    Retry::spawn(retry_strategy, || async {
        let mut connection = get_timescale_connection(pool.clone())
        .await
        .expect("Error connecting to database");
    diesel::insert_into(order_books)
        .values(&orderbook)
        .on_conflict(order_book_id)
        .do_update()
        .set(&orderbook)
        .execute(&mut connection)
        .await?;

    order_books
        .filter(security_id.eq(&orderbook.security_id))
        .first(&mut connection)
        .await
        .map_err(|e| {
            eprintln!("Error fetching new orderbook: {}", e);
            e
        })
    })
        .await
}

pub async fn get_orderbooks(pool: Arc<Pool<CustomAsyncPgConnectionManager>>) -> Result<Vec<OrderBook>, Error> {
    println!("Getting orderbooks");
    use crate::schema::order_books::dsl::*;

    let retry_strategy = ExponentialBackoff::from_millis(10).map(jitter).take(3);

    Retry::spawn(retry_strategy, || async {
        let mut connection = get_timescale_connection(pool.clone())
        .await
        .expect("Error connecting to database");
    order_books
        .load::<OrderBook>(&mut connection)
        .await
        .map_err(|e| {
            eprintln!("Error loading orderbooks: {}", e);
            e
        })
    }).await
}

pub async fn update_orderbook(pool: Arc<Pool<CustomAsyncPgConnectionManager>>, orderbook: OrderBook, volume: BigDecimal) -> Result<OrderBook, Error> {
    println!("Updating orderbook: {:?}", orderbook);
    use crate::schema::order_books::dsl::*;

    let retry_strategy = ExponentialBackoff::from_millis(10).map(jitter).take(3);

    Retry::spawn(retry_strategy, || async {
        let mut connection = get_timescale_connection(pool.clone())
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
    .await
}

pub async fn get_orderbook_by_orderbook_id(pool: Arc<Pool<CustomAsyncPgConnectionManager>>, o_id: Uuid) -> Result<OrderBook, Error> {
    println!("Getting orderbook by symbol");
    use crate::schema::order_books::dsl::*;

    let retry_strategy = ExponentialBackoff::from_millis(10).map(jitter).take(3);

    Retry::spawn(retry_strategy, || async {
        let mut connection = get_timescale_connection(pool.clone())
        .await
        .expect("Error connecting to database");
    order_books
        .filter(order_book_id.eq(o_id))
        .first::<OrderBook>(&mut connection)
        .await
        .map_err(|e| {
            eprintln!("Error loading orderbook: {}", e);
            e
        })
    }).await
}

pub async fn orderbook_exists(pool: Arc<Pool<CustomAsyncPgConnectionManager>>, s_id: Uuid) -> bool {
    println!("Checking if orderbook exists: {}", s_id);
    use crate::schema::order_books::dsl::*;

    let retry_strategy = ExponentialBackoff::from_millis(10).map(jitter).take(3);

    Retry::spawn(retry_strategy, || async {
        let mut connection = get_timescale_connection(pool.clone())
        .await
        .expect("Error connecting to database");
    order_books
        .filter(security_id.eq(s_id))
        .first::<OrderBook>(&mut connection)
        .await
        .map_err(|e| {
            eprintln!("Error loading orderbook: {}", e);
            e
        })
    }).await.is_ok()
}
