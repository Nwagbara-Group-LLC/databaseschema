use crate::{get_timescale_connection, models::open_buy_order::{NewOpenBuyOrder, OpenBuyOrder}, CustomAsyncPgConnectionManager};
use bigdecimal::BigDecimal;
use deadpool::managed::Pool;
use diesel::{prelude::*, result::Error, upsert::excluded};
use diesel::QueryDsl;
use diesel_async::{AsyncConnection, RunQueryDsl};
use tokio_retry::{strategy::{jitter, ExponentialBackoff}, Retry};
use std::{cmp::Reverse, collections::BTreeMap, sync::Arc};

pub async fn create_open_buy_order(pool: Arc<Pool<CustomAsyncPgConnectionManager>>, order: NewOpenBuyOrder) -> Result<OpenBuyOrder, Error> {
    println!("Creating open buy order: {:?}", order);
    use crate::schema::open_buy_orders::dsl::*;

    let retry_strategy = ExponentialBackoff::from_millis(10).map(jitter).take(3);

    Retry::spawn(retry_strategy, || async {
        let mut connection = get_timescale_connection(pool.clone())
        .await
        .expect("Error connecting to database");
    let result = diesel::insert_into(open_buy_orders)
            .values(&order)
            .on_conflict((created_at, unique_id))
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
    println!("Creating {} open buy orders", orders.len());
    use crate::schema::open_buy_orders::dsl::*;

    let retry_strategy = ExponentialBackoff::from_millis(10).map(jitter).take(3);

    Retry::spawn(retry_strategy, || async {
        let mut connection = get_timescale_connection(pool.clone())
        .await
        .expect("Error connecting to database");

        // Use transaction for atomicity
        connection.transaction::<_, Error, _>(|conn| Box::pin(async {
            // Batch insert with conflict resolution
            diesel::insert_into(open_buy_orders)
                .values(&orders)
                .on_conflict((created_at, unique_id))
                .do_update()
                .set((
                    price_level.eq(excluded(price_level)),
                    buy_quantity.eq(excluded(buy_quantity)),
                ))
                .execute(conn)
                .await?;

            // Fetch the inserted/updated records
            let unique_ids: Vec<String> = orders.iter()
                .map(|order| order.unique_id.clone())
                .collect();
                
            open_buy_orders
                .filter(unique_id.eq_any(&unique_ids))
                .load::<OpenBuyOrder>(conn)
                .await
                .map_err(|e| {
                    eprintln!("Error fetching created buy orders: {}", e);
                    e
                })
        })).await
    }).await
}

pub async fn modify_open_buy_order(pool: Arc<Pool<CustomAsyncPgConnectionManager>>, id: &str, new_price_level: &BigDecimal, new_buy_quantity: &BigDecimal) -> Result<OpenBuyOrder, Error> {
    println!("Modifying open buy order: {:?}", id);
    use crate::schema::open_buy_orders::dsl::*;

    let retry_strategy = ExponentialBackoff::from_millis(10).map(jitter).take(3);

    Retry::spawn(retry_strategy, || async {
        let mut connection = get_timescale_connection(pool.clone())
        .await
        .expect("Error connecting to database");
    diesel::update(open_buy_orders.filter(unique_id.eq(id)))
        .set((price_level.eq(new_price_level), buy_quantity.eq(new_buy_quantity)))
        .get_result(&mut connection)
        .await
        .map_err(|e| {
            eprintln!("Error modifying open buy order: {}", e);
            e
        })
    }).await
}

pub async fn modify_open_buy_orders(
    pool: Arc<Pool<CustomAsyncPgConnectionManager>>,
    updates: Vec<(&String, &BigDecimal, &BigDecimal)>,
) -> Result<Vec<OpenBuyOrder>, Error> {
    println!("Modifying {} open buy orders", updates.len());
    if updates.is_empty() {
        return Ok(vec![]);
    }

    let retry_strategy = ExponentialBackoff::from_millis(10).take(3);

    Retry::spawn(retry_strategy, || async {
        let mut connection = get_timescale_connection(pool.clone())
            .await
            .expect("Error connecting to database");

        use crate::schema::open_buy_orders::dsl::*;
        
        let mut all_results = Vec::with_capacity(updates.len());
        
        // Process in batches to avoid overwhelming database and ensure atomicity
        const BATCH_SIZE: usize = 50;
        
        for chunk in updates.chunks(BATCH_SIZE) {
            // Use transaction to ensure atomicity within each batch
            let batch_results = connection.transaction::<_, Error, _>(|conn| Box::pin(async {
                let mut chunk_results = Vec::with_capacity(chunk.len());
                
                // Use parameterized queries - NO SQL INJECTION RISK
                for (id, new_price, new_quantity) in chunk {
                    let result = diesel::update(open_buy_orders.filter(unique_id.eq(*id)))
                        .set((
                            price_level.eq(*new_price), 
                            buy_quantity.eq(*new_quantity)
                        ))
                        .get_result::<OpenBuyOrder>(conn)
                        .await
                        .map_err(|e| {
                            eprintln!("Error modifying buy order {}: {}", id, e);
                            e
                        })?;
                    chunk_results.push(result);
                }
                Ok(chunk_results)
            })).await?;
            
            all_results.extend(batch_results);
        }
        
        Ok(all_results)
    }).await
}

pub async fn delete_open_buy_order(pool: Arc<Pool<CustomAsyncPgConnectionManager>>, id: &str) -> Result<usize, Error> {
    println!("Deleting open buy order");
    use crate::schema::open_buy_orders::dsl::*;

    let retry_strategy = ExponentialBackoff::from_millis(10).map(jitter).take(3);

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

pub async fn delete_open_buy_orders(pool: Arc<Pool<CustomAsyncPgConnectionManager>>, ids: &[String]) -> Result<usize, Error> {
    println!("Deleting {} open buy orders", ids.len());
    use crate::schema::open_buy_orders::dsl::*;

    let retry_strategy = ExponentialBackoff::from_millis(10).map(jitter).take(3);

    Retry::spawn(retry_strategy, || async {
        let mut connection = get_timescale_connection(pool.clone())
        .await
        .expect("Error connecting to database");

        // Process in batches for large deletions
        const BATCH_SIZE: usize = 100;
        let mut total_deleted = 0;
        
        for chunk in ids.chunks(BATCH_SIZE) {
            let deleted = diesel::delete(open_buy_orders.filter(unique_id.eq_any(chunk)))
                .execute(&mut connection)
                .await
                .map_err(|e| {
                    eprintln!("Error deleting buy orders batch: {}", e);
                    e
                })?;
            total_deleted += deleted;
        }
        
        Ok(total_deleted)
    }).await
}

pub async fn get_open_buy_orders(pool: Arc<Pool<CustomAsyncPgConnectionManager>>) -> Result<BTreeMap<Reverse<BigDecimal>, Vec<OpenBuyOrder>>, Error> {
    println!("Getting open buy orders");
    use crate::schema::open_buy_orders::dsl::*;

    let retry_strategy = ExponentialBackoff::from_millis(10).map(jitter).take(3);

    let orders = Retry::spawn(retry_strategy, || async {
        let mut connection = get_timescale_connection(pool.clone())
        .await
        .expect("Error connecting to database");
    open_buy_orders
        .order(price_level.desc())
        .load::<OpenBuyOrder>(&mut connection)
        .await
        .map_err(|e| {
            eprintln!("Error loading open buy orders: {}", e);
            e
        })
    }).await?;

    let mut buy_orderbook = BTreeMap::new();
    for order in orders {
        buy_orderbook.entry(Reverse(order.price_level.clone()))
            .or_insert_with(Vec::new)
            .push(order);
    }

    for orders_at_price in buy_orderbook.values_mut() {
        orders_at_price.sort_by(|a: &OpenBuyOrder, b| a.created_at.cmp(&b.created_at));
    }

    Ok(buy_orderbook)
}

pub async fn get_open_buy_orders_by_symbol(pool: Arc<Pool<CustomAsyncPgConnectionManager>>, sym: &str) -> Result<BTreeMap<Reverse<BigDecimal>, Vec<OpenBuyOrder>>, Error> {
    println!("Getting open buy orders");
    use crate::schema::open_buy_orders::dsl::*;

    let retry_strategy = ExponentialBackoff::from_millis(10).map(jitter).take(3);

    let orders = Retry::spawn(retry_strategy, || async {
        let mut connection = get_timescale_connection(pool.clone())
        .await
        .expect("Error connecting to database");
    open_buy_orders
    .filter(symbol.eq(sym))
    .order(price_level.desc())
    .load::<OpenBuyOrder>(&mut connection)
        .await
        .map_err(|e| {
            eprintln!("Error loading open buy orders: {}", e);
            e
        })
    }).await?;

    let mut buy_orderbook = BTreeMap::new();
    for order in orders {
        buy_orderbook.entry(Reverse(order.price_level.clone()))
            .or_insert_with(Vec::new)
            .push(order);
    }

    for orders_at_price in buy_orderbook.values_mut() {
        orders_at_price.sort_by(|a: &OpenBuyOrder, b| a.created_at.cmp(&b.created_at));
    }

    Ok(buy_orderbook)
}