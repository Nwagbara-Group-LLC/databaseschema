use crate::{get_timescale_connection, models::open_sell_order::{NewOpenSellOrder, OpenSellOrder}, CustomAsyncPgConnectionManager};
use deadpool::managed::Pool;
use diesel::{prelude::*, result::Error};
use diesel::QueryDsl;
use diesel_async::RunQueryDsl;
use tokio_retry::{strategy::FixedInterval, Retry};
use std::sync::Arc;

pub async fn create_open_sell_order(pool: Arc<Pool<CustomAsyncPgConnectionManager>>, order: NewOpenSellOrder) -> Result<OpenSellOrder, Error> {
    println!("Creating open sell order: {:?}", order);
    use crate::schema::open_sell_orders::dsl::*;
    
    let retry_strategy = FixedInterval::from_millis(1).take(15);

    Retry::spawn(retry_strategy, || async {
        let mut connection = get_timescale_connection(pool.clone())
        .await
        .expect("Error connecting to database");

    diesel::insert_into(open_sell_orders)
        .values(&order)
        .returning(OpenSellOrder::as_returning())
        .get_result(&mut connection)
        .await
        .map_err(|e| {
            eprintln!("Error saving new open sell order: {}", e);
            e
        })
    }).await
}

pub async fn create_open_sell_orders(pool: Arc<Pool<CustomAsyncPgConnectionManager>>, orders: Vec<NewOpenSellOrder>) -> Result<Vec<OpenSellOrder>, Error> {
    println!("Creating open sell order: {:?}", orders);
    use crate::schema::open_sell_orders::dsl::*;

    let retry_strategy = FixedInterval::from_millis(1).take(15);

    Retry::spawn(retry_strategy, || async {
        let mut connection = get_timescale_connection(pool.clone())
        .await
        .expect("Error connecting to database");

    diesel::insert_into(open_sell_orders)
        .values(&orders)
        .returning(OpenSellOrder::as_returning())
        .get_results(&mut connection)
        .await
        .map_err(|e| {
            eprintln!("Error saving new open sell order: {}", e);
            e
        })
    }).await
}

pub async fn delete_open_sell_order(pool: Arc<Pool<CustomAsyncPgConnectionManager>>, id: &str) -> Result<usize, Error> {
    println!("Deleting open sell order");
    use crate::schema::open_sell_orders::dsl::*;

    let retry_strategy = FixedInterval::from_millis(1).take(15);

    Retry::spawn(retry_strategy, || async {
        let mut connection = get_timescale_connection(pool.clone())
        .await
        .expect("Error connecting to database");
    diesel::delete(open_sell_orders.filter(unique_id.eq(id)))
        .execute(&mut connection)
        .await
        .map_err(|e| {
            eprintln!("Error deleting open sell order: {}", e);
            e
        })
    })
        .await
}

pub async fn delete_open_sell_orders(pool: Arc<Pool<CustomAsyncPgConnectionManager>>, ids: &Vec<&String>) -> Result<usize, Error> {
    println!("Deleting open sell orders: {:?}", ids);
    use crate::schema::open_sell_orders::dsl::*;

    let retry_strategy = FixedInterval::from_millis(1).take(15);

    Retry::spawn(retry_strategy, || async {
        let mut connection = get_timescale_connection(pool.clone())
        .await
        .expect("Error connecting to database");
    diesel::delete(open_sell_orders.filter(unique_id.eq_any(ids)))
        .execute(&mut connection)
        .await
        .map_err(|e| {
            eprintln!("Error deleting open sell orders: {}", e);
            e
        })
    }).await
}

pub async fn get_open_sell_orders(pool: Arc<Pool<CustomAsyncPgConnectionManager>>) -> Result<Vec<OpenSellOrder>, Error> {
    println!("Getting open sell orders");
    use crate::schema::open_sell_orders::dsl::*;

    let retry_strategy = FixedInterval::from_millis(1).take(15);

    Retry::spawn(retry_strategy, || async {
        let mut connection = get_timescale_connection(pool.clone())
        .await
        .expect("Error connecting to database");
    open_sell_orders
        .load::<OpenSellOrder>(&mut connection)
        .await
        .map_err(|e| {
            eprintln!("Error loading open sell orders: {}", e);
            e
        })
    }).await
}

pub async fn get_open_sell_orders_by_symbol(pool: Arc<Pool<CustomAsyncPgConnectionManager>>, sym: &str) -> Result<Vec<OpenSellOrder>, Error> {
    println!("Getting open sell orders");
    use crate::schema::open_sell_orders::dsl::*;

    let retry_strategy = FixedInterval::from_millis(1).take(15);

    Retry::spawn(retry_strategy, || async {
        let mut connection = get_timescale_connection(pool.clone())
        .await
        .expect("Error connecting to database");
    open_sell_orders
    .filter(symbol.eq(sym))
    .load::<OpenSellOrder>(&mut connection)
        .await
        .map_err(|e| {
            eprintln!("Error loading open sell orders: {}", e);
            e
        })
    }).await
}
