use crate::{get_timescale_connection, models::open_sell_order::{NewOpenSellOrder, OpenSellOrder}, CustomAsyncPgConnectionManager};
use bigdecimal::BigDecimal;
use deadpool::managed::Pool;
use diesel::{prelude::*, result::Error, sql_query, upsert::excluded};
use diesel::QueryDsl;
use diesel_async::RunQueryDsl;
use tokio_retry::{strategy::{jitter, ExponentialBackoff}, Retry};
use std::{collections::BTreeMap, sync::Arc};

pub async fn create_open_sell_order(pool: Arc<Pool<CustomAsyncPgConnectionManager>>, order: NewOpenSellOrder) -> Result<OpenSellOrder, Error> {
    println!("Creating open sell order: {:?}", order);
    use crate::schema::open_sell_orders::dsl::*;
    
    let retry_strategy = ExponentialBackoff::from_millis(10).map(jitter).take(3);

    Retry::spawn(retry_strategy, || async {
        let mut connection = get_timescale_connection(pool.clone())
        .await
        .expect("Error connecting to database");

    let result = diesel::insert_into(open_sell_orders)
        .values(&order)
        .on_conflict((created_at, unique_id))
        .do_update()
        .set(&order)
        .execute(&mut connection)
            .await;

        match result {
            Ok(_) => {},
            Err(e) => {
                eprintln!("Error saving new open sell order: {}", e);
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

    let retry_strategy = ExponentialBackoff::from_millis(10).map(jitter).take(3);

    Retry::spawn(retry_strategy, || async {
        let mut connection = get_timescale_connection(pool.clone())
        .await
        .expect("Error connecting to database");

        let result = diesel::insert_into(open_sell_orders)
        .values(&orders)
        .on_conflict((created_at, unique_id))
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
            eprintln!("Error fetching new open sell orders: {}", e);
            e
        })
    }).await
}

pub async fn modify_open_sell_order(pool: Arc<Pool<CustomAsyncPgConnectionManager>>, id: &str, new_price_level: &BigDecimal, new_sell_quantity: &BigDecimal) -> Result<OpenSellOrder, Error> {
    println!("Modifying open sell order: {:?}", id);
    use crate::schema::open_sell_orders::dsl::*;

    let retry_strategy = ExponentialBackoff::from_millis(10).map(jitter).take(3);

    Retry::spawn(retry_strategy, || async {
        let mut connection = get_timescale_connection(pool.clone())
        .await
        .expect("Error connecting to database");
    diesel::update(open_sell_orders.filter(unique_id.eq(id)))
        .set((price_level.eq(new_price_level), sell_quantity.eq(new_sell_quantity)))
        .get_result(&mut connection)
        .await
        .map_err(|e| {
            eprintln!("Error modifying open sell order: {}", e);
            e
        })
    }).await
}

pub async fn modify_open_sell_orders(
    pool: Arc<Pool<CustomAsyncPgConnectionManager>>,
    updates: Vec<(&String, &BigDecimal, &BigDecimal)>,
) -> Result<Vec<OpenSellOrder>, Error> {
    println!("Modifying open sell orders: {:?}", updates);
    if updates.is_empty() {
        return Ok(vec![]);
    }

    let retry_strategy = ExponentialBackoff::from_millis(10).take(3);

    Retry::spawn(retry_strategy, || async {
        let mut connection = get_timescale_connection(pool.clone())
            .await
            .expect("Error connecting to database");

        let mut unique_ids = Vec::with_capacity(updates.len());
        let mut price_level_cases = String::new();
        let mut sell_quantity_cases = String::new();

        for (id, new_price, new_quantity) in &updates {
            unique_ids.push(format!("'{}'", id));
            price_level_cases.push_str(&format!("WHEN unique_id = '{}' THEN '{}' ", id, new_price));
            sell_quantity_cases.push_str(&format!("WHEN unique_id = '{}' THEN '{}' ", id, new_quantity));
        }

        let unique_ids_sql = unique_ids.join(", ");

        let update_query = format!(
            "UPDATE open_sell_orders SET 
                price_level = CASE {} ELSE price_level END, 
                sell_quantity = CASE {} ELSE sell_quantity END 
            WHERE unique_id IN ({}) 
            RETURNING *;",
            price_level_cases, sell_quantity_cases, unique_ids_sql
        );

        sql_query(update_query)
            .load::<OpenSellOrder>(&mut connection)
            .await
            .map_err(|e| {
                eprintln!("Error modifying open sell orders: {}", e);
                e
            })
    })
    .await
}

pub async fn delete_open_sell_order(pool: Arc<Pool<CustomAsyncPgConnectionManager>>, id: &str) -> Result<usize, Error> {
    println!("Deleting open sell order");
    use crate::schema::open_sell_orders::dsl::*;

    let retry_strategy = ExponentialBackoff::from_millis(10).map(jitter).take(3);

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

    let retry_strategy = ExponentialBackoff::from_millis(10).map(jitter).take(3);

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

pub async fn get_open_sell_orders(pool: Arc<Pool<CustomAsyncPgConnectionManager>>) -> Result<BTreeMap<BigDecimal, Vec<OpenSellOrder>>, Error> {
    println!("Getting open sell orders");
    use crate::schema::open_sell_orders::dsl::*;

    let retry_strategy = ExponentialBackoff::from_millis(10).map(jitter).take(3);

    let orders = Retry::spawn(retry_strategy, || async {
        let mut connection = get_timescale_connection(pool.clone())
        .await
        .expect("Error connecting to database");
    open_sell_orders
        .order(price_level.asc())
        .load::<OpenSellOrder>(&mut connection)
        .await
        .map_err(|e| {
            eprintln!("Error loading open sell orders: {}", e);
            e
        })
    }).await?;

    let mut sell_orderbook = BTreeMap::new();
    for order in orders {
        sell_orderbook.entry(order.price_level.clone())
            .or_insert_with(Vec::new)
            .push(order);
    }

    for orders_at_price in sell_orderbook.values_mut() {
        orders_at_price.sort_by(|a: &OpenSellOrder, b| a.created_at.cmp(&b.created_at));
    }

    Ok(sell_orderbook)
}

pub async fn get_open_sell_orders_by_symbol(pool: Arc<Pool<CustomAsyncPgConnectionManager>>, sym: &str) -> Result<BTreeMap<BigDecimal, Vec<OpenSellOrder>>, Error> {
    println!("Getting open sell orders");
    use crate::schema::open_sell_orders::dsl::*;

    let retry_strategy = ExponentialBackoff::from_millis(10).map(jitter).take(3);

    let orders = Retry::spawn(retry_strategy, || async {
        let mut connection = get_timescale_connection(pool.clone())
        .await
        .expect("Error connecting to database");
    open_sell_orders
    .filter(symbol.eq(sym))
    .order(price_level.asc())
    .load::<OpenSellOrder>(&mut connection)
        .await
        .map_err(|e| {
            eprintln!("Error loading open sell orders: {}", e);
            e
        })
    }).await?;

    let mut sell_orderbook = BTreeMap::new();
    for order in orders {
        sell_orderbook.entry(order.price_level.clone())
            .or_insert_with(Vec::new)
            .push(order);
    }

    for orders_at_price in sell_orderbook.values_mut() {
        orders_at_price.sort_by(|a: &OpenSellOrder, b| a.created_at.cmp(&b.created_at));
    }

    Ok(sell_orderbook)
}