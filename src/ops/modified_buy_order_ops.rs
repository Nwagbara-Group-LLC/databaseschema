use crate::{get_timescale_connection, models::modified_buy_order::{ModifiedBuyOrder, NewModifiedBuyOrder}, CustomAsyncPgConnectionManager};
use deadpool::managed::Pool;
use diesel::{prelude::*, result::Error, upsert::excluded};
use diesel::QueryDsl;
use diesel_async::RunQueryDsl;
use tokio_retry::{strategy::{jitter, ExponentialBackoff}, Retry};
use std::sync::Arc;

pub async fn create_modified_buy_order(pool: Arc<Pool<CustomAsyncPgConnectionManager>>, order: NewModifiedBuyOrder) -> Result<ModifiedBuyOrder, Error> {
    println!("Creating modified buy order: {:?}", order);
    use crate::schema::modified_buy_orders::dsl::*;

    let retry_strategy = ExponentialBackoff::from_millis(10).map(jitter).take(3);

    Retry::spawn(retry_strategy, || async {
        let mut connection = get_timescale_connection(pool.clone())
        .await
        .expect("Error connecting to database");
    let result = diesel::insert_into(modified_buy_orders)
            .values(&order)
            .on_conflict((created_at, unique_id))
            .do_update()
            .set(&order)
            .execute(&mut connection)
            .await;

        match result {
            Ok(_) => {},
            Err(e) => {
                eprintln!("Error saving new modified buy order: {}", e);
            }
        }

        modified_buy_orders
            .filter(unique_id.eq(&order.unique_id))
            .first(&mut connection)
            .await
            .map_err(|e| {
                eprintln!("Error fetching new modified buy order: {}", e);
                e
            })
    }).await
}

pub async fn create_modified_buy_orders(pool: Arc<Pool<CustomAsyncPgConnectionManager>>, orders: Vec<NewModifiedBuyOrder>) -> Result<Vec<ModifiedBuyOrder>, Error> {
    println!("Creating modified buy orders: {:?}", orders);
    use crate::schema::modified_buy_orders::dsl::*;

    let retry_strategy = ExponentialBackoff::from_millis(10)
    .map(jitter)
    .take(3);

    Retry::spawn(retry_strategy, || async {
        let mut connection = get_timescale_connection(pool.clone())
        .await
        .expect("Error connecting to database");
    let result = diesel::insert_into(modified_buy_orders)
            .values(&orders)
            .on_conflict((created_at, unique_id))
            .do_update()
            .set((
                // Specify the columns you want to update here
                price_level.eq(excluded(price_level)),
                new_buy_quantity.eq(excluded(new_buy_quantity)),
                // Add more columns as needed
            ))
            .execute(&mut connection)
            .await;

        match result {
            Ok(_) => {},
            Err(e) => {
                eprintln!("Error saving new modified buy orders: {}", e);
            }
        }

        modified_buy_orders
            .filter(unique_id.eq_any(orders.iter().map(|order| order.unique_id.clone())))
            .load::<ModifiedBuyOrder>(&mut connection)
            .await
            .map_err(|e| {
                eprintln!("Error fetching new modified buy orders: {}", e);
                e
            })
    }).await
}

pub async fn delete_modified_buy_order(pool: Arc<Pool<CustomAsyncPgConnectionManager>>, id: &str) -> Result<usize, Error> {
    println!("Deleting modified buy order");
    use crate::schema::modified_buy_orders::dsl::*;

    let retry_strategy = ExponentialBackoff::from_millis(10)
    .map(jitter)
    .take(3);

    Retry::spawn(retry_strategy, || async {
        let mut connection = get_timescale_connection(pool.clone())
        .await
        .expect("Error connecting to database");
    diesel::delete(modified_buy_orders.filter(unique_id.eq(id)))
        .execute(&mut connection)
        .await
        .map_err(|e| {
            eprintln!("Error deleting modified buy order: {}", e);
            e
        })
    }).await
}

pub async fn delete_modified_buy_orders(pool: Arc<Pool<CustomAsyncPgConnectionManager>>, ids: &Vec<&String>) -> Result<usize, Error> {
    println!("Deleting modified buy orders: {:?}", ids);
    use crate::schema::modified_buy_orders::dsl::*;

    let retry_strategy = ExponentialBackoff::from_millis(10)
    .map(jitter)
    .take(3);

    Retry::spawn(retry_strategy, || async {
        let mut connection = get_timescale_connection(pool.clone())
        .await
        .expect("Error connecting to database");
    diesel::delete(modified_buy_orders.filter(unique_id.eq_any(ids)))
        .execute(&mut connection)
        .await
        .map_err(|e| {
            eprintln!("Error deleting modified buy orders: {}", e);
            e
        })
    }).await
}

pub async fn get_modified_buy_orders(pool: Arc<Pool<CustomAsyncPgConnectionManager>>) -> Result<Vec<ModifiedBuyOrder>, Error> {
    println!("Getting modified buy orders");
    use crate::schema::modified_buy_orders::dsl::*;

    let retry_strategy = ExponentialBackoff::from_millis(10)
    .map(jitter)
    .take(3);

    Retry::spawn(retry_strategy, || async {
        let mut connection = get_timescale_connection(pool.clone())
        .await
        .expect("Error connecting to database");
    modified_buy_orders
        .load::<ModifiedBuyOrder>(&mut connection)
        .await
        .map_err(|e| {
            eprintln!("Error loading modified buy orders: {}", e);
            e
        })
    }).await
}

pub async fn get_modified_buy_orders_by_symbol(pool: Arc<Pool<CustomAsyncPgConnectionManager>>, sym: &str) -> Result<Vec<ModifiedBuyOrder>, Error> {
    println!("Getting modified buy orders");
    use crate::schema::modified_buy_orders::dsl::*;

    let retry_strategy = ExponentialBackoff::from_millis(10)
    .map(jitter)
    .take(3);

    Retry::spawn(retry_strategy, || async {
        let mut connection = get_timescale_connection(pool.clone())
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
    }).await
}