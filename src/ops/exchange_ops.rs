use crate::{get_timescale_connection, models::exchange::{Exchange, NewExchange}, CustomAsyncPgConnectionManager};
use deadpool::managed::Pool;
use diesel::{prelude::*, result::Error};
use diesel_async::RunQueryDsl;
use tokio_retry::{strategy::{jitter, ExponentialBackoff}, Retry};
use std::sync::Arc;

pub async fn create_exchange(pool: Arc<Pool<CustomAsyncPgConnectionManager>>, new_exchange: NewExchange) -> Result<Exchange, Error> {
    println!("Creating exchange: {:?}", new_exchange);
    use crate::schema::exchanges::dsl::*;

    let retry_strategy = ExponentialBackoff::from_millis(10).map(jitter).take(3);

    Retry::spawn(retry_strategy, || async {
        let mut connection = get_timescale_connection(pool.clone())
            .await
            .expect("Error connecting to database");
        diesel::insert_into(exchanges)
            .values(&new_exchange)
            .on_conflict(exchange_id)
            .do_update()
            .set(&new_exchange)
            .execute(&mut connection)
            .await?;

        exchanges
            .filter(exchange.eq(&new_exchange.exchange))
            .first(&mut connection)
            .await
            .map_err(|e| {
                eprintln!("Error fetching new exchange: {}", e);
                e
            })
    }).await
}

pub async fn get_exchanges(pool: Arc<Pool<CustomAsyncPgConnectionManager>>) -> Result<Vec<Exchange>, Error> {
    println!("Getting exchanges");
    use crate::schema::exchanges::dsl::*;

    let retry_strategy = ExponentialBackoff::from_millis(10).map(jitter).take(3);

    Retry::spawn(retry_strategy, || async {
        let mut connection = get_timescale_connection(pool.clone())
            .await
            .expect("Error connecting to database");
        exchanges
            .load::<Exchange>(&mut connection)
            .await
            .map_err(|e| {
                eprintln!("Error loading exchanges: {}", e);
                e
            })
    }).await
}

pub async fn get_exchanges_by_name(pool: Arc<Pool<CustomAsyncPgConnectionManager>>, name: &String) -> Result<Exchange, Error> {
    println!("Getting exchange");
    use crate::schema::exchanges::dsl::*;

    let retry_strategy = ExponentialBackoff::from_millis(10).map(jitter).take(3);

    Retry::spawn(retry_strategy, || async {
        let mut connection = get_timescale_connection(pool.clone())
            .await
            .expect("Error connecting to database");
        exchanges
            .filter(exchange.eq(name))
            .first::<Exchange>(&mut connection)
            .await
            .map_err(|e| {
                eprintln!("Error loading exchange: {}", e);
                e
            })
    }).await
}

pub async fn exchange_exists(pool: Arc<Pool<CustomAsyncPgConnectionManager>>, name: &String) -> bool {
    println!("Checking if exchange exists");
    use crate::schema::exchanges::dsl::*;

    let retry_strategy = ExponentialBackoff::from_millis(10).map(jitter).take(3);

    Retry::spawn(retry_strategy, || async {
        let mut connection = get_timescale_connection(pool.clone())
            .await
            .expect("Error connecting to database");
        exchanges
            .filter(exchange.eq(name))
            .first::<Exchange>(&mut connection)
            .await
            .map_err(|e| {
                eprintln!("Error loading exchange: {}", e);
                e
            })
    }).await.is_ok()
}