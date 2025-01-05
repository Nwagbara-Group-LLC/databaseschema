use crate::{get_connection, models::modified_buy_order::{ModifiedBuyOrder, NewModifiedBuyOrder}, CustomAsyncPgConnectionManager};
use deadpool::managed::Pool;
use diesel::prelude::*;
use diesel::QueryDsl;
use diesel_async::RunQueryDsl;
use tokio_retry::{strategy::FixedInterval, Retry};
use std::sync::Arc;

pub async fn create_modified_buy_order(pool: Arc<Pool<CustomAsyncPgConnectionManager>>, order: NewModifiedBuyOrder) -> ModifiedBuyOrder {
    println!("Creating modified buy order: {:?}", order);
    use crate::schema::modified_buy_orders::dsl::*;

    let retry_strategy = FixedInterval::from_millis(1).take(15);

    Retry::spawn(retry_strategy, || async {
        let mut connection = get_connection(pool.clone())
        .await
        .expect("Error connecting to database");
    diesel::insert_into(modified_buy_orders)
        .values(&order)
        .returning(ModifiedBuyOrder::as_returning())
        .get_result(&mut connection)
        .await
        .map_err(|e| {
            eprintln!("Error saving new modified buy order: {}", e);
            e
        })
    }).await.expect("Error getting open buy candlesticks")
}

pub async fn create_modified_buy_orders(pool: Arc<Pool<CustomAsyncPgConnectionManager>>, orders: Vec<NewModifiedBuyOrder>) -> Vec<ModifiedBuyOrder> {
    println!("Creating modified buy orders: {:?}", orders);
    use crate::schema::modified_buy_orders::dsl::*;

    let retry_strategy = FixedInterval::from_millis(1).take(15);

    Retry::spawn(retry_strategy, || async {
        let mut connection = get_connection(pool.clone())
        .await
        .expect("Error connecting to database");
    diesel::insert_into(modified_buy_orders)
        .values(&orders)
        .returning(ModifiedBuyOrder::as_returning())
        .get_results(&mut connection)
        .await
        .map_err(|e| {
            eprintln!("Error saving new modified buy orders: {}", e);
            e
        })
    }).await.expect("Error getting open buy candlesticks")
}

pub async fn delete_modified_buy_order(pool: Arc<Pool<CustomAsyncPgConnectionManager>>, id: &str) {
    println!("Deleting modified buy order");
    use crate::schema::modified_buy_orders::dsl::*;

    let retry_strategy = FixedInterval::from_millis(1).take(15);

    Retry::spawn(retry_strategy, || async {
        let mut connection = get_connection(pool.clone())
        .await
        .expect("Error connecting to database");
    diesel::delete(modified_buy_orders.filter(unique_id.eq(id)))
        .execute(&mut connection)
        .await
        .map_err(|e| {
            eprintln!("Error deleting modified buy order: {}", e);
            e
        })
    }).await.expect("Error getting open buy candlesticks");
}

pub async fn delete_modified_buy_orders(pool: Arc<Pool<CustomAsyncPgConnectionManager>>, ids: &Vec<&String>) {
    println!("Deleting modified buy orders: {:?}", ids);
    use crate::schema::modified_buy_orders::dsl::*;

    let retry_strategy = FixedInterval::from_millis(1).take(15);

    Retry::spawn(retry_strategy, || async {
        let mut connection = get_connection(pool.clone())
        .await
        .expect("Error connecting to database");
    diesel::delete(modified_buy_orders.filter(unique_id.eq_any(ids)))
        .execute(&mut connection)
        .await
        .map_err(|e| {
            eprintln!("Error deleting modified buy orders: {}", e);
            e
        })
    }).await.expect("Error getting open buy candlesticks");
}

pub async fn get_modified_buy_orders(pool: Arc<Pool<CustomAsyncPgConnectionManager>>) -> Vec<ModifiedBuyOrder> {
    println!("Getting modified buy orders");
    use crate::schema::modified_buy_orders::dsl::*;

    let retry_strategy = FixedInterval::from_millis(1).take(15);

    Retry::spawn(retry_strategy, || async {
        let mut connection = get_connection(pool.clone())
        .await
        .expect("Error connecting to database");
    modified_buy_orders
        .load::<ModifiedBuyOrder>(&mut connection)
        .await
        .map_err(|e| {
            eprintln!("Error loading modified buy orders: {}", e);
            e
        })
    }).await.expect("Error getting modified buy orders")
}

pub async fn get_modified_buy_orders_by_symbol(pool: Arc<Pool<CustomAsyncPgConnectionManager>>, sym: &str) -> Vec<ModifiedBuyOrder> {
    println!("Getting modified buy orders");
    use crate::schema::modified_buy_orders::dsl::*;

    let retry_strategy = FixedInterval::from_millis(1).take(15);

    Retry::spawn(retry_strategy, || async {
        let mut connection = get_connection(pool.clone())
        .await
        .expect("Error connecting to database");
    modified_buy_orders
    .filter(symbol.eq(sym))
    .load::<ModifiedBuyOrder>(&mut connection)
        .await
        .map_err(|e| {
            eprintln!("Error loading modified buy orders: {}", e);
            e
        })
    }).await.expect("Error getting modified buy orders")
}