use crate::{get_timescale_connection, models::open_buy_order::{NewOpenBuyOrder, OpenBuyOrder}, CustomAsyncPgConnectionManager};
use deadpool::managed::Pool;
use diesel::prelude::*;
use diesel::QueryDsl;
use diesel_async::RunQueryDsl;
use tokio_retry::{strategy::FixedInterval, Retry};
use std::sync::Arc;

pub async fn create_open_buy_order(pool: Arc<Pool<CustomAsyncPgConnectionManager>>, order: NewOpenBuyOrder) -> OpenBuyOrder {
    println!("Creating open buy order: {:?}", order);
    use crate::schema::open_buy_orders::dsl::*;

    let retry_strategy = FixedInterval::from_millis(1).take(15);

    Retry::spawn(retry_strategy, || async {
        let mut connection = get_timescale_connection(pool.clone())
        .await
        .expect("Error connecting to database");
    diesel::insert_into(open_buy_orders)
        .values(&order)
        .returning(OpenBuyOrder::as_returning())
        .get_result(&mut connection)
        .await
        .map_err(|e| {
            eprintln!("Error saving new open buy order: {}", e);
            e
        })
    }).await.expect("Error creating new open buy order")
}

pub async fn create_open_buy_orders(pool: Arc<Pool<CustomAsyncPgConnectionManager>>, orders: Vec<NewOpenBuyOrder>) -> Vec<OpenBuyOrder> {
    println!("Creating open buy orders: {:?}", orders);
    use crate::schema::open_buy_orders::dsl::*;

    let retry_strategy = FixedInterval::from_millis(1).take(15);

    Retry::spawn(retry_strategy, || async {
        let mut connection = get_timescale_connection(pool.clone())
        .await
        .expect("Error connecting to database");
    diesel::insert_into(open_buy_orders)
        .values(&orders)
        .returning(OpenBuyOrder::as_returning())
        .get_results(&mut connection)
        .await
        .map_err(|e| {
            eprintln!("Error saving new open buy orders: {}", e);
            e
        })
    }).await.expect("Error creating new open buy orders")
}

pub async fn delete_open_buy_order(pool: Arc<Pool<CustomAsyncPgConnectionManager>>, id: &str) {
    println!("Deleting open buy order");
    use crate::schema::open_buy_orders::dsl::*;

    let retry_strategy = FixedInterval::from_millis(1).take(15);

    Retry::spawn(retry_strategy, || async {
        let mut connection = get_timescale_connection(pool.clone())
        .await
        .expect("Error connecting to database");
    diesel::delete(open_buy_orders.filter(unique_id.eq(id)))
        .execute(&mut connection)
        .await
        .map_err(|e| {
            eprintln!("Error deleting open buy order: {}", e);
            e
        })
    }).await.expect("Error deleting open buy order");
}

pub async fn delete_open_buy_orders(pool: Arc<Pool<CustomAsyncPgConnectionManager>>, ids: &Vec<&String>) {
    println!("Deleting open buy orders: {:?}", ids);
    use crate::schema::open_buy_orders::dsl::*;

    let retry_strategy = FixedInterval::from_millis(1).take(15);

    Retry::spawn(retry_strategy, || async {
        let mut connection = get_timescale_connection(pool.clone())
        .await
        .expect("Error connecting to database");
    diesel::delete(open_buy_orders.filter(unique_id.eq_any(ids)))
        .execute(&mut connection)
        .await
        .map_err(|e| {
            eprintln!("Error deleting open buy orders: {}", e);
            e
        })
    }).await.expect("Error deleting open buy orders");
}

pub async fn get_open_buy_orders(pool: Arc<Pool<CustomAsyncPgConnectionManager>>) -> Vec<OpenBuyOrder> {
    println!("Getting open buy orders");
    use crate::schema::open_buy_orders::dsl::*;

    let retry_strategy = FixedInterval::from_millis(1).take(15);

    Retry::spawn(retry_strategy, || async {
        let mut connection = get_timescale_connection(pool.clone())
        .await
        .expect("Error connecting to database");
    open_buy_orders
        .load::<OpenBuyOrder>(&mut connection)
        .await
        .map_err(|e| {
            eprintln!("Error loading open buy orders: {}", e);
            e
        })
    }).await.expect("Error getting open buy orders")
}

pub async fn get_open_buy_orders_by_symbol(pool: Arc<Pool<CustomAsyncPgConnectionManager>>, sym: &str) -> Vec<OpenBuyOrder> {
    println!("Getting open buy orders");
    use crate::schema::open_buy_orders::dsl::*;

    let retry_strategy = FixedInterval::from_millis(1).take(15);

    Retry::spawn(retry_strategy, || async {
        let mut connection = get_timescale_connection(pool.clone())
        .await
        .expect("Error connecting to database");
    open_buy_orders
    .filter(symbol.eq(sym))
    .load::<OpenBuyOrder>(&mut connection)
        .await
        .map_err(|e| {
            eprintln!("Error loading open buy orders: {}", e);
            e
        })
    }).await.expect("Error getting open buy orders")
}
