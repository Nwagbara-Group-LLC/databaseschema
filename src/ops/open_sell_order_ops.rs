use crate::{get_timescale_connection, models::open_sell_order::{NewOpenSellOrder, OpenSellOrder}, CustomAsyncPgConnectionManager};
use deadpool::managed::Pool;
use diesel::{prelude::*, result::Error, upsert::excluded};
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

    let result = diesel::insert_into(open_sell_orders)
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

        open_sell_orders
            .filter(unique_id.eq(&order.unique_id))
            .first(&mut connection)
            .await
            .map_err(|e| {
                eprintln!("Error fetching new open sell order: {}", e);
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

        let result = diesel::insert_into(open_sell_orders)
        .values(&orders)
        .on_conflict(unique_id)
        .do_update()
        .set((
            // Specify the columns you want to update here
            price_level.eq(excluded(price_level)),
            sell_quantity.eq(excluded(sell_quantity)),
            // Add more columns as needed
        ))
        .execute(&mut connection)
        .await;

        match result {
            Ok(_) => {},
            Err(e) => {
                eprintln!("Error saving new open sell orders: {}", e);
            }
        }

    open_sell_orders
        .filter(unique_id.eq_any(orders.iter().map(|order| order.unique_id.clone())))
        .load::<OpenSellOrder>(&mut connection)
        .await
        .map_err(|e| {
            eprintln!("Error fetching new open buy orders: {}", e);
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
