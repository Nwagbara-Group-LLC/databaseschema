use crate::{
    get_connection,
    models::modified_sell_order::{ModifiedSellOrder, NewModifiedSellOrder},
    CustomAsyncPgConnectionManager,
};
use deadpool::managed::Pool;
use diesel::prelude::*;
use diesel::QueryDsl;
use diesel_async::RunQueryDsl;
use tokio_retry::{strategy::FixedInterval, Retry};
use std::sync::Arc;

pub async fn create_modified_sell_order(
    pool: Arc<Pool<CustomAsyncPgConnectionManager>>,
    order: NewModifiedSellOrder,
) -> ModifiedSellOrder {
    println!("Creating modified sell order: {:?}", order);
    use crate::schema::modified_sell_orders::dsl::*;

    let retry_strategy = FixedInterval::from_millis(1).take(15);

    Retry::spawn(retry_strategy, || async {
        let mut connection = get_connection(pool.clone())
        .await
        .expect("Error connecting to database");

    diesel::insert_into(modified_sell_orders)
        .values(&order)
        .returning(ModifiedSellOrder::as_returning())
        .get_result(&mut connection)
        .await
        .map_err(|e| {
            eprintln!("Error saving new modified sell order: {}", e);
            e
        })
    }).await.expect("Error creating new modified sell order")
}

pub async fn create_modified_sell_orders(
    pool: Arc<Pool<CustomAsyncPgConnectionManager>>,
    orders: Vec<NewModifiedSellOrder>,
) -> Vec<ModifiedSellOrder> {
    println!("Creating modified sell orders: {:?}", orders);
    use crate::schema::modified_sell_orders::dsl::*;

    let retry_strategy = FixedInterval::from_millis(1).take(15);

    Retry::spawn(retry_strategy, || async {
        let mut connection = get_connection(pool.clone())
        .await
        .expect("Error connecting to database");

    diesel::insert_into(modified_sell_orders)
        .values(&orders)
        .returning(ModifiedSellOrder::as_returning())
        .get_results(&mut connection)
        .await
        .map_err(|e| {
            eprintln!("Error saving new modified sell orders: {}", e);
            e
        })
    }).await.expect("Error creating new modified sell orders")
}

pub async fn delete_modified_sell_order(pool: Arc<Pool<CustomAsyncPgConnectionManager>>, id: &str) {
    println!("Deleting modified sell order");
    use crate::schema::modified_sell_orders::dsl::*;

    let retry_strategy = FixedInterval::from_millis(1).take(15);

    Retry::spawn(retry_strategy, || async {
        let mut connection = get_connection(pool.clone())
        .await
        .expect("Error connecting to database");
    diesel::delete(modified_sell_orders.filter(unique_id.eq(id)))
        .execute(&mut connection)
        .await
        .map_err(|e| {
            eprintln!("Error deleting modified sell order: {}", e);
            e
        })
    }).await.expect("Error deleting modified sell order");
}

pub async fn delete_modified_sell_orders(pool: Arc<Pool<CustomAsyncPgConnectionManager>>, ids: &Vec<&String>) {
    println!("Deleting modified sell orders: {:?}", ids);
    use crate::schema::modified_sell_orders::dsl::*;

    let retry_strategy = FixedInterval::from_millis(1).take(15);

    Retry::spawn(retry_strategy, || async {
        let mut connection = get_connection(pool.clone())
        .await
        .expect("Error connecting to database");
    diesel::delete(modified_sell_orders.filter(unique_id.eq_any(ids)))
        .execute(&mut connection)
        .await
        .map_err(|e| {
            eprintln!("Error deleting modified sell orders: {}", e);
            e
        })
    }).await.expect("Error deleting modified sell orders");
}

pub async fn get_modified_sell_orders(pool: Arc<Pool<CustomAsyncPgConnectionManager>>) -> Vec<ModifiedSellOrder> {
    println!("Getting modified sell orders");
    use crate::schema::modified_sell_orders::dsl::*;

    let retry_strategy = FixedInterval::from_millis(1).take(15);

    Retry::spawn(retry_strategy, || async {
        let mut connection = get_connection(pool.clone())
        .await
        .expect("Error connecting to database");
    modified_sell_orders
        .load::<ModifiedSellOrder>(&mut connection)
        .await
        .map_err(|e| {
            eprintln!("Error loading modified sell orders: {}", e);
            e
        })
    }).await.expect("Error getting modified sell orders")
}

pub async fn get_modified_sell_orders_by_symbol(pool: Arc<Pool<CustomAsyncPgConnectionManager>>, sym: &str) -> Vec<ModifiedSellOrder> {
    println!("Getting modified sell orders");
    use crate::schema::modified_sell_orders::dsl::*;

    let retry_strategy = FixedInterval::from_millis(1).take(15);

    Retry::spawn(retry_strategy, || async {
        let mut connection = get_connection(pool.clone())
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
    }).await.expect("Error getting modified sell orders")
}