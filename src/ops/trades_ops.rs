use crate::{get_connection, models::trade::{NewTrade, Trade}, CustomAsyncPgConnectionManager};
use deadpool::managed::Pool;
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use tokio_retry::{strategy::FixedInterval, Retry};
use std::sync::Arc;

pub async fn create_trades(pool: Arc<Pool<CustomAsyncPgConnectionManager>>, orders: Vec<NewTrade>) -> Vec<Trade> {
    println!("Creating trades: {:?}", orders);
    use crate::schema::trades::dsl::*;

    let retry_strategy = FixedInterval::from_millis(1).take(15);

    Retry::spawn(retry_strategy, || async {
        let mut connection = get_connection(pool.clone())
        .await
        .expect("Error connecting to database");
    diesel::insert_into(trades)
        .values(&orders)
        .returning(Trade::as_returning())
        .get_results(&mut connection)
        .await
        .map_err(|e| {
            eprintln!("Error saving new trades: {}", e);
            e
        })
    }).await.expect("Error creating trades")
}

pub async fn get_trades_by_symbol(pool: Arc<Pool<CustomAsyncPgConnectionManager>>, sym: &str) -> Vec<Trade> {
    println!("Getting trades by symbol");
    use crate::schema::trades::dsl::*;

    let retry_strategy = FixedInterval::from_millis(1).take(15);

    Retry::spawn(retry_strategy, || async {
        let mut connection = get_connection(pool.clone())
            .await
            .expect("Error connecting to database");
        trades
            .filter(symbol.eq(sym))
            .select(Trade::as_select()) // Ensure the fields match
            .load::<Trade>(&mut connection)
            .await
            .map_err(|e| {
                eprintln!("Error loading trades: {}", e);
                e
            })
    }).await.expect("Error getting trades")
}