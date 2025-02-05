use crate::{
    get_timescale_connection,
    models::modified_sell_order::{ModifiedSellOrder, NewModifiedSellOrder},
    CustomAsyncPgConnectionManager,
};
use deadpool::managed::Pool;
use diesel::{prelude::*, result::Error, upsert::excluded};
use diesel::QueryDsl;
use diesel_async::RunQueryDsl;
use tokio_retry::{strategy::{jitter, ExponentialBackoff}, Retry};
use std::sync::Arc;

pub async fn create_modified_sell_order(
    pool: Arc<Pool<CustomAsyncPgConnectionManager>>,
    order: NewModifiedSellOrder,
) -> Result<ModifiedSellOrder, Error> {
    println!("Creating modified sell order: {:?}", order);
    use crate::schema::modified_sell_orders::dsl::*;

    let retry_strategy = ExponentialBackoff::from_millis(10).map(jitter).take(3);

    Retry::spawn(retry_strategy, || async {
        let mut connection = get_timescale_connection(pool.clone())
        .await
        .expect("Error connecting to database");

        let result = diesel::insert_into(modified_sell_orders)
        .values(&order)
        .on_conflict((created_at, unique_id))
        .do_update()
        .set(&order)
        .execute(&mut connection)
        .await;

    match result {
        Ok(_) => {},
        Err(e) => {
            eprintln!("Error saving new modified sell order: {}", e);
        }
    }

    modified_sell_orders
        .filter(unique_id.eq(&order.unique_id))
        .first(&mut connection)
        .await
        .map_err(|e| {
            eprintln!("Error fetching new modified sell order: {}", e);
            e
        })
    }).await
}

pub async fn create_modified_sell_orders(
    pool: Arc<Pool<CustomAsyncPgConnectionManager>>,
    orders: Vec<NewModifiedSellOrder>,
) -> Result<Vec<ModifiedSellOrder>, Error> {
    println!("Creating modified sell orders: {:?}", orders);
    use crate::schema::modified_sell_orders::dsl::*;

    let retry_strategy = ExponentialBackoff::from_millis(10)
    .map(jitter)
    .take(3);

    Retry::spawn(retry_strategy, || async {
        let mut connection = get_timescale_connection(pool.clone())
        .await
        .expect("Error connecting to database");

        let result = diesel::insert_into(modified_sell_orders)
        .values(&orders)
        .on_conflict((created_at, unique_id))
        .do_update()
        .set((
            // Specify the columns you want to update here
            price_level.eq(excluded(price_level)),
            new_sell_quantity.eq(excluded(new_sell_quantity)),
            // Add more columns as needed
        ))
        .execute(&mut connection)
        .await;

    match result {
        Ok(_) => {},
        Err(e) => {
            eprintln!("Error saving new modified sell orders: {}", e);
        }
    }

    modified_sell_orders
        .filter(unique_id.eq_any(orders.iter().map(|order| order.unique_id.clone())))
        .load::<ModifiedSellOrder>(&mut connection)
        .await
        .map_err(|e| {
            eprintln!("Error fetching new modified sell orders: {}", e);
            e
        })
    }).await
}

pub async fn delete_modified_sell_order(pool: Arc<Pool<CustomAsyncPgConnectionManager>>, id: &str) -> Result<usize, Error> {
    println!("Deleting modified sell order");
    use crate::schema::modified_sell_orders::dsl::*;

    let retry_strategy = ExponentialBackoff::from_millis(10)
    .map(jitter)
    .take(3);

    Retry::spawn(retry_strategy, || async {
        let mut connection = get_timescale_connection(pool.clone())
        .await
        .expect("Error connecting to database");
    diesel::delete(modified_sell_orders.filter(unique_id.eq(id)))
        .execute(&mut connection)
        .await
        .map_err(|e| {
            eprintln!("Error deleting modified sell order: {}", e);
            e
        })
    }).await
}

pub async fn delete_modified_sell_orders(pool: Arc<Pool<CustomAsyncPgConnectionManager>>, ids: &Vec<&String>) -> Result<usize, Error> {
    println!("Deleting modified sell orders: {:?}", ids);
    use crate::schema::modified_sell_orders::dsl::*;

    let retry_strategy = ExponentialBackoff::from_millis(10)
    .map(jitter)
    .take(3);

    Retry::spawn(retry_strategy, || async {
        let mut connection = get_timescale_connection(pool.clone())
        .await
        .expect("Error connecting to database");
    diesel::delete(modified_sell_orders.filter(unique_id.eq_any(ids)))
        .execute(&mut connection)
        .await
        .map_err(|e| {
            eprintln!("Error deleting modified sell orders: {}", e);
            e
        })
    }).await
}

pub async fn get_modified_sell_orders(pool: Arc<Pool<CustomAsyncPgConnectionManager>>) -> Result<Vec<ModifiedSellOrder>, Error> {
    println!("Getting modified sell orders");
    use crate::schema::modified_sell_orders::dsl::*;

    let retry_strategy = ExponentialBackoff::from_millis(10)
    .map(jitter)
    .take(3);

    Retry::spawn(retry_strategy, || async {
        let mut connection = get_timescale_connection(pool.clone())
        .await
        .expect("Error connecting to database");
    modified_sell_orders
        .load::<ModifiedSellOrder>(&mut connection)
        .await
        .map_err(|e| {
            eprintln!("Error loading modified sell orders: {}", e);
            e
        })
    }).await
}

pub async fn get_modified_sell_orders_by_symbol(pool: Arc<Pool<CustomAsyncPgConnectionManager>>, sym: &str) -> Result<Vec<ModifiedSellOrder>, Error> {
    println!("Getting modified sell orders");
    use crate::schema::modified_sell_orders::dsl::*;

    let retry_strategy = ExponentialBackoff::from_millis(10)
    .map(jitter)
    .take(3);

    Retry::spawn(retry_strategy, || async {
        let mut connection = get_timescale_connection(pool.clone())
        .await
        .expect("Error connecting to database");
    modified_sell_orders
    .filter(symbol.eq(sym))
    .load::<ModifiedSellOrder>(&mut connection)
        .await
        .map_err(|e| {
            eprintln!("Error loading modified sell orders: {}", e);
            e
        })
    }).await
}