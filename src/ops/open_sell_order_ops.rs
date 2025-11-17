use crate::{get_timescale_connection, models::open_sell_order::{NewOpenSellOrder, OpenSellOrder}};
use bigdecimal::BigDecimal;
use diesel_async::pooled_connection::deadpool;
use diesel_async::AsyncPgConnection;
use diesel::{prelude::*, result::Error, upsert::excluded};
use diesel::QueryDsl;
use diesel_async::{AsyncConnection, RunQueryDsl};
use tokio_retry::{strategy::{jitter, ExponentialBackoff}, Retry};
use tracing::{debug, warn, info, error};
use std::{collections::BTreeMap, sync::Arc};

pub async fn create_open_sell_order(pool: Arc<deadpool::Pool<AsyncPgConnection>>, order: NewOpenSellOrder) -> Result<OpenSellOrder, Error> {
    if order.unique_id.is_empty() || order.unique_id.len() > 255 {
        warn!("Invalid unique_id length: {} characters", order.unique_id.len());
        return Err(Error::DatabaseError(
            diesel::result::DatabaseErrorKind::Unknown,
            Box::new("Unique ID must be between 1 and 255 characters".to_string())
        ));
    }
    
    debug!("Creating open sell order: unique_id={}", order.unique_id);
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
                error!("Database error saving new open sell order: {}", e);
            }
        }

        open_sell_orders
            .filter(unique_id.eq(&order.unique_id))
            .first(&mut connection)
            .await
            .map_err(|e| {
                error!("Database error fetching new open sell order: {}", e);
                e
            })
    }).await
}

pub async fn create_open_sell_orders(pool: Arc<deadpool::Pool<AsyncPgConnection>>, orders: Vec<NewOpenSellOrder>) -> Result<Vec<OpenSellOrder>, Error> {
    info!("Creating {} open sell orders", orders.len());
    use crate::schema::open_sell_orders::dsl::*;

    let retry_strategy = ExponentialBackoff::from_millis(10).map(jitter).take(3);

    Retry::spawn(retry_strategy, || async {
        let mut connection = get_timescale_connection(pool.clone())
        .await
        .expect("Error connecting to database");

        // Process in smaller batches to reduce deadlock probability
        const BATCH_SIZE: usize = 25;
        let mut all_results = Vec::with_capacity(orders.len());

        for chunk in orders.chunks(BATCH_SIZE) {
            // Sort by unique_id to ensure consistent lock ordering across pods
            let mut sorted_chunk = chunk.to_vec();
            sorted_chunk.sort_by(|a, b| a.unique_id.cmp(&b.unique_id));

            let batch_results = connection.transaction::<_, Error, _>(|conn| Box::pin(async {
                // Use DO NOTHING to avoid deadlocks on concurrent inserts
                diesel::insert_into(open_sell_orders)
                    .values(&sorted_chunk)
                    .on_conflict((created_at, unique_id))
                    .do_nothing()
                    .execute(conn)
                    .await?;

                // Fetch the inserted/updated records
                let unique_ids: Vec<String> = sorted_chunk.iter()
                    .map(|order| order.unique_id.clone())
                    .collect();
                    
                open_sell_orders
                    .filter(unique_id.eq_any(&unique_ids))
                    .load::<OpenSellOrder>(conn)
                    .await
                    .map_err(|e| {
                        error!("Database error fetching created sell orders: {}", e);
                        e
                    })
            })).await?;

            all_results.extend(batch_results);
        }

        Ok(all_results)
    }).await
}

pub async fn modify_open_sell_order(pool: Arc<deadpool::Pool<AsyncPgConnection>>, id: &str, new_price_level: &BigDecimal, new_sell_quantity: &BigDecimal) -> Result<OpenSellOrder, Error> {
    info!("Modifying open sell order: {}", id);
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
            error!("Database error modifying open sell order {}: {}", id, e);
            e
        })
    }).await
}

pub async fn modify_open_sell_orders(
    pool: Arc<deadpool::Pool<AsyncPgConnection>>,
    updates: Vec<(&String, &BigDecimal, &BigDecimal)>,
) -> Result<Vec<OpenSellOrder>, Error> {
    info!("Modifying {} open sell orders", updates.len());
    if updates.is_empty() {
        return Ok(vec![]);
    }

    let retry_strategy = ExponentialBackoff::from_millis(10).take(3);

    Retry::spawn(retry_strategy, || async {
        let mut connection = get_timescale_connection(pool.clone())
            .await
            .expect("Error connecting to database");

        use crate::schema::open_sell_orders::dsl::*;
        
        let mut all_results = Vec::with_capacity(updates.len());
        
        // Process in batches to avoid overwhelming database and ensure atomicity  
        const BATCH_SIZE: usize = 50;
        
        for chunk in updates.chunks(BATCH_SIZE) {
            // Use transaction to ensure atomicity within each batch
            let batch_results = connection.transaction::<_, Error, _>(|conn| Box::pin(async {
                let mut chunk_results = Vec::with_capacity(chunk.len());
                
                // Use parameterized queries - NO SQL INJECTION RISK
                for (id, new_price, new_quantity) in chunk {
                    let result = diesel::update(open_sell_orders.filter(unique_id.eq(*id)))
                        .set((
                            price_level.eq(*new_price), 
                            sell_quantity.eq(*new_quantity)
                        ))
                        .get_result::<OpenSellOrder>(conn)
                        .await
                        .map_err(|e| {
                            error!("Database error modifying sell order {}: {}", id, e);
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

pub async fn delete_open_sell_order(pool: Arc<deadpool::Pool<AsyncPgConnection>>, id: &str) -> Result<usize, Error> {
    info!("Deleting open sell order");
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
            error!("Database error deleting open sell order: {}", e);
            e
        })
    })
        .await
}

pub async fn delete_open_sell_orders(pool: Arc<deadpool::Pool<AsyncPgConnection>>, ids: &[String]) -> Result<usize, Error> {
    info!("Deleting {} open sell orders", ids.len());
    use crate::schema::open_sell_orders::dsl::*;

    let retry_strategy = ExponentialBackoff::from_millis(10).map(jitter).take(3);

    Retry::spawn(retry_strategy, || async {
        let mut connection = get_timescale_connection(pool.clone())
        .await
        .expect("Error connecting to database");

        // Process in batches for large deletions
        const BATCH_SIZE: usize = 100;
        let mut total_deleted = 0;
        
        for chunk in ids.chunks(BATCH_SIZE) {
            let deleted = diesel::delete(open_sell_orders.filter(unique_id.eq_any(chunk)))
                .execute(&mut connection)
                .await
                .map_err(|e| {
                    error!("Database error deleting sell orders batch: {}", e);
                    e
                })?;
            total_deleted += deleted;
        }
        
        Ok(total_deleted)
    }).await
}

pub async fn get_open_sell_orders(pool: Arc<deadpool::Pool<AsyncPgConnection>>) -> Result<BTreeMap<BigDecimal, Vec<OpenSellOrder>>, Error> {
    info!("Getting open sell orders");
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
            error!("Database error loading open sell orders: {}", e);
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

pub async fn get_open_sell_orders_by_symbol(pool: Arc<deadpool::Pool<AsyncPgConnection>>, sym: &str) -> Result<BTreeMap<BigDecimal, Vec<OpenSellOrder>>, Error> {
    info!("Getting open sell orders for symbol: {}", sym);
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
            error!("Database error loading open sell orders for symbol {}: {}", sym, e);
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
