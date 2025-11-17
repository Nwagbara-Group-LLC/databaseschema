use crate::{get_timescale_connection, models::open_buy_order::{NewOpenBuyOrder, OpenBuyOrder}};
use bigdecimal::BigDecimal;
use diesel_async::pooled_connection::deadpool;
use diesel_async::AsyncPgConnection;
use diesel::{prelude::*, result::Error};
use diesel::QueryDsl;
use diesel_async::{AsyncConnection, RunQueryDsl};
use tokio_retry::{strategy::{jitter, ExponentialBackoff}, Retry};
use tracing::{info, error, debug, warn};
use std::{cmp::Reverse, collections::BTreeMap, sync::Arc};

pub async fn create_open_buy_order(pool: Arc<deadpool::Pool<AsyncPgConnection>>, order: NewOpenBuyOrder) -> Result<OpenBuyOrder, Error> {
    if order.unique_id.is_empty() || order.unique_id.len() > 255 {
        warn!("Invalid unique_id length: {} characters", order.unique_id.len());
        return Err(Error::DatabaseError(
            diesel::result::DatabaseErrorKind::Unknown,
            Box::new("Unique ID must be between 1 and 255 characters".to_string())
        ));
    }
    
    debug!("Creating open buy order: unique_id={}", order.unique_id);
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
            Ok(_) => {
                info!("Successfully created open buy order: unique_id={}", order.unique_id);
            },
            Err(e) => {
                error!("Failed to save open buy order {}: {}", order.unique_id, e);
            }
        }

        open_buy_orders
            .filter(unique_id.eq(&order.unique_id))
            .first(&mut connection)
            .await
            .map_err(|e| {
                error!("Failed to fetch open buy order {}: {}", order.unique_id, e);
                e
            })
    }).await
}

pub async fn create_open_buy_orders(pool: Arc<deadpool::Pool<AsyncPgConnection>>, orders: Vec<NewOpenBuyOrder>) -> Result<Vec<OpenBuyOrder>, Error> {
    if orders.is_empty() {
        warn!("Attempted to create open buy orders with empty input");
        return Err(Error::DatabaseError(
            diesel::result::DatabaseErrorKind::Unknown,
            Box::new("Cannot create orders with empty input".to_string())
        ));
    }
    
    if orders.len() > 1000 {
        warn!("Large batch size for open buy orders: {}", orders.len());
        return Err(Error::DatabaseError(
            diesel::result::DatabaseErrorKind::Unknown,
            Box::new("Batch size too large (max 1000)".to_string())
        ));
    }
    
    info!("Creating {} open buy orders", orders.len());
    use crate::schema::open_buy_orders::dsl::*;

    let retry_strategy = ExponentialBackoff::from_millis(10).map(jitter).take(3);

    Retry::spawn(retry_strategy, || async {
        let mut connection = get_timescale_connection(pool.clone())
        .await
        .expect("Error connecting to database");

        // Process in smaller batches to reduce deadlock probability
        const BATCH_SIZE: usize = 25;
        let mut all_results = Vec::with_capacity(orders.len());

        for chunk in orders.chunks(BATCH_SIZE) {
            let batch_results = connection.transaction::<_, Error, _>(|conn| Box::pin(async {
                // Use DO NOTHING to avoid deadlocks on concurrent inserts
                diesel::insert_into(open_buy_orders)
                    .values(chunk)
                    .on_conflict((created_at, unique_id))
                    .do_nothing()
                    .execute(conn)
                    .await?;

                // Fetch the inserted/updated records
                let unique_ids: Vec<String> = chunk.iter()
                    .map(|order| order.unique_id.clone())
                    .collect();
                    
                open_buy_orders
                    .filter(unique_id.eq_any(&unique_ids))
                    .load::<OpenBuyOrder>(conn)
                    .await
                    .map_err(|e| {
                        error!("Failed to fetch created buy orders: {}", e);
                        e
                    })
            })).await?;

            all_results.extend(batch_results);
        }

        Ok(all_results)
    }).await
}

pub async fn modify_open_buy_order(pool: Arc<deadpool::Pool<AsyncPgConnection>>, id: &str, new_price_level: &BigDecimal, new_buy_quantity: &BigDecimal) -> Result<OpenBuyOrder, Error> {
    if id.is_empty() || id.len() > 255 {
        warn!("Invalid order ID length: {} characters", id.len());
        return Err(Error::DatabaseError(
            diesel::result::DatabaseErrorKind::Unknown,
            Box::new("Order ID must be between 1 and 255 characters".to_string())
        ));
    }
    
    debug!("Modifying open buy order: id={}", id);
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
            error!("Failed to modify open buy order {}: {}", id, e);
            e
        })
    }).await
}

pub async fn modify_open_buy_orders(
    pool: Arc<deadpool::Pool<AsyncPgConnection>>,
    updates: Vec<(&String, &BigDecimal, &BigDecimal)>,
) -> Result<Vec<OpenBuyOrder>, Error> {
    if updates.is_empty() {
        return Ok(vec![]);
    }
    
    if updates.len() > 1000 {
        warn!("Large batch size for buy order updates: {}", updates.len());
        return Err(Error::DatabaseError(
            diesel::result::DatabaseErrorKind::Unknown,
            Box::new("Batch size too large (max 1000)".to_string())
        ));
    }
    
    info!("Modifying {} open buy orders", updates.len());

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
                            error!("Database error modifying buy order {}: {}", id, e);
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

pub async fn delete_open_buy_order(pool: Arc<deadpool::Pool<AsyncPgConnection>>, id: &str) -> Result<usize, Error> {
    info!("Deleting open buy order");
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
            error!("Database error deleting open buy order: {}", e);
            e
        })
    }).await
}

pub async fn delete_open_buy_orders(pool: Arc<deadpool::Pool<AsyncPgConnection>>, ids: &[String]) -> Result<usize, Error> {
    info!("Deleting {} open buy orders", ids.len());
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
                    error!("Database error deleting buy orders batch: {}", e);
                    e
                })?;
            total_deleted += deleted;
        }
        
        Ok(total_deleted)
    }).await
}

pub async fn get_open_buy_orders(pool: Arc<deadpool::Pool<AsyncPgConnection>>) -> Result<BTreeMap<Reverse<BigDecimal>, Vec<OpenBuyOrder>>, Error> {
    info!("Getting open buy orders");
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
            error!("Database error loading open buy orders: {}", e);
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

pub async fn get_open_buy_orders_by_symbol(pool: Arc<deadpool::Pool<AsyncPgConnection>>, sym: &str) -> Result<BTreeMap<Reverse<BigDecimal>, Vec<OpenBuyOrder>>, Error> {
    info!("Getting open buy orders for symbol: {}", sym);
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
            error!("Database error loading open buy orders for symbol {}: {}", sym, e);
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
