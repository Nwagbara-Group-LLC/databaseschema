use crate::{get_timescale_connection, models::open_buy_order::{NewOpenBuyOrder, OpenBuyOrder}, CustomAsyncPgConnectionManager};
use deadpool::managed::Pool;
use diesel::{prelude::*, result::Error, upsert::excluded};
use diesel::QueryDsl;
use diesel_async::RunQueryDsl;
use tokio_retry::{strategy::FixedInterval, Retry};
use std::sync::Arc;

pub async fn create_open_buy_order(pool: Arc<Pool<CustomAsyncPgConnectionManager>>, order: NewOpenBuyOrder) -> Result<OpenBuyOrder, Error> {
    println!("Creating open buy order: {:?}", order);
    use crate::schema::open_buy_orders::dsl::*;

    let retry_strategy = FixedInterval::from_millis(1).take(15);

    Retry::spawn(retry_strategy, || async {
        let mut connection = get_timescale_connection(pool.clone())
        .await
        .expect("Error connecting to database");
    let result = diesel::insert_into(open_buy_orders)
            .values(&order)
            .on_conflict(unique_id)
            .do_update()
            .set(&order)
            .execute(&mut connection)
            .await;

        match result {
            Ok(_) => {},
            Err(e) => {
                eprintln!("Error saving new open buy order: {}", e);
            }
        }

        open_buy_orders
            .filter(unique_id.eq(&order.unique_id))
            .first(&mut connection)
            .await
            .map_err(|e| {
                eprintln!("Error fetching new open buy order: {}", e);
                e
            })
    }).await
}

pub async fn create_open_buy_orders(pool: Arc<Pool<CustomAsyncPgConnectionManager>>, orders: Vec<NewOpenBuyOrder>) -> Result<Vec<OpenBuyOrder>, Error> {
    println!("Creating open buy orders: {:?}", orders);
    use crate::schema::open_buy_orders::dsl::*;

    let retry_strategy = FixedInterval::from_millis(1).take(15);

    Retry::spawn(retry_strategy, || async {
        let mut connection = get_timescale_connection(pool.clone())
        .await
        .expect("Error connecting to database");
    let result = diesel::insert_into(open_buy_orders)
            .values(&orders)
            .on_conflict(unique_id)
            .do_update()
            .set((
                // Specify the columns you want to update here
                price_level.eq(excluded(price_level)),
                buy_quantity.eq(excluded(buy_quantity)),
                // Add more columns as needed
            ))
            .execute(&mut connection)
            .await;

        match result {
            Ok(_) => {},
            Err(e) => {
                eprintln!("Error saving new open buy orders: {}", e);
            }
        }

        open_buy_orders
            .filter(unique_id.eq_any(orders.iter().map(|order| order.unique_id.clone())))
            .load::<OpenBuyOrder>(&mut connection)
            .await
            .map_err(|e| {
                eprintln!("Error fetching new open buy orders: {}", e);
                e
            })
    }).await
}

pub async fn delete_open_buy_order(pool: Arc<Pool<CustomAsyncPgConnectionManager>>, id: &str) -> Result<usize, Error> {
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
    }).await
}

pub async fn delete_open_buy_orders(pool: Arc<Pool<CustomAsyncPgConnectionManager>>, ids: &Vec<&String>) -> Result<usize, Error> {
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
    }).await
}

pub async fn get_open_buy_orders(pool: Arc<Pool<CustomAsyncPgConnectionManager>>) -> Result<Vec<OpenBuyOrder>, Error> {
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
    }).await
}

pub async fn get_open_buy_orders_by_symbol(pool: Arc<Pool<CustomAsyncPgConnectionManager>>, sym: &str) -> Result<Vec<OpenBuyOrder>, Error> {
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
    }).await
}
