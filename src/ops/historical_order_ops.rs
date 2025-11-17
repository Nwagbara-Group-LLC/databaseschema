use crate::{get_timescale_connection, models::historical_order::{HistoricalOrder, NewHistoricalOrder}};
use diesel_async::pooled_connection::deadpool;
use diesel_async::AsyncPgConnection;
use diesel::{prelude::*, result::Error};
use diesel_async::AsyncConnection;
use diesel_async::RunQueryDsl;
use tokio_retry::{strategy::{jitter, ExponentialBackoff}, Retry};
use std::sync::Arc;
use tracing::{info, error, warn, debug};
use std::time::Instant;

pub async fn create_historical_order(pool: Arc<deadpool::Pool<AsyncPgConnection>>, historical_order: NewHistoricalOrder) -> Result<HistoricalOrder, Error> {
    let start_time = Instant::now();
    debug!("Creating historical order: {:?}", historical_order);
    use crate::schema::historical_orders::dsl::*;

    let retry_strategy = ExponentialBackoff::from_millis(10).map(jitter).take(3);

    Retry::spawn(retry_strategy, || async {
        let mut connection = get_timescale_connection(pool.clone())
            .await
            .map_err(|e| {
                error!("Failed to get database connection: {}", e);
                Error::DatabaseError(
                    diesel::result::DatabaseErrorKind::UnableToSendCommand,
                    Box::new(e.to_string())
                )
            })?;
            
        diesel::insert_into(historical_orders)
            .values(&historical_order)
            .on_conflict((timestamp, order_id, event_type))
            .do_update()
            .set(&historical_order)
            .execute(&mut connection)
            .await?;

        let result = historical_orders
            .filter(order_id.eq(&historical_order.get_order_id()))
            .first(&mut connection)
            .await
            .map_err(|e| {
                error!("Error fetching new historical order: {}", e);
                e
            })?;
            
        debug!("Historical order created in {}ms", start_time.elapsed().as_millis());
        Ok(result)
    }).await
}

pub async fn create_historical_orders(pool: Arc<deadpool::Pool<AsyncPgConnection>>, orders: Vec<NewHistoricalOrder>) -> Result<Vec<HistoricalOrder>, Error> {
    let start_time = Instant::now();
    info!("Creating {} historical orders", orders.len());
    use crate::schema::historical_orders::dsl::*;

    // Security: Validate batch size to prevent resource exhaustion
    const MAX_BATCH_SIZE: usize = 10000;
    if orders.len() > MAX_BATCH_SIZE {
        error!("Batch size {} exceeds maximum {}", orders.len(), MAX_BATCH_SIZE);
        return Err(Error::RollbackTransaction);
    }

    if orders.is_empty() {
        warn!("create_historical_orders called with empty orders vector");
        return Ok(Vec::new());
    }

    let retry_strategy = ExponentialBackoff::from_millis(10).map(jitter).take(3);

    Retry::spawn(retry_strategy, || async {
        let mut connection = get_timescale_connection(pool.clone())
            .await
            .map_err(|e| {
                error!("Failed to get database connection: {}", e);
                Error::DatabaseError(
                    diesel::result::DatabaseErrorKind::UnableToSendCommand,
                    Box::new(e.to_string())
                )
            })?;

        // Process in smaller chunks to reduce deadlock probability
        const CHUNK_SIZE: usize = 100;
        let mut all_results = Vec::new();
        
        for chunk in orders.chunks(CHUNK_SIZE) {
            let chunk_results = connection.transaction::<_, Error, _>(|conn| Box::pin(async {
                // Use DO NOTHING to avoid deadlocks on concurrent inserts
                diesel::insert_into(historical_orders)
                    .values(chunk)
                    .on_conflict((timestamp, order_id, event_type))
                    .do_nothing()
                    .execute(conn)
                    .await?;

                // Fetch the inserted/updated records for this chunk
                let chunk_order_ids: Vec<String> = chunk.iter()
                    .map(|order| order.get_order_id())
                    .collect();
                    
                historical_orders
                    .filter(order_id.eq_any(&chunk_order_ids))
                    .load::<HistoricalOrder>(conn)
                    .await
                    .map_err(|e| {
                        error!("Error fetching historical orders chunk: {}", e);
                        e
                    })
            })).await?;
                
            all_results.extend(chunk_results);
        }
        
        info!("Created {} historical orders in {}ms", all_results.len(), start_time.elapsed().as_millis());
        Ok(all_results)
    }).await
}

pub async fn get_historical_orders(pool: Arc<deadpool::Pool<AsyncPgConnection>>, sym: &str, xchange: &str) -> Result<Vec<HistoricalOrder>, Error> {
    let start_time = Instant::now();
    info!("Getting historical orders for symbol: {} on exchange: {}", sym, xchange);
    use crate::schema::historical_orders::dsl::*;

    // Security: Input validation
    if sym.is_empty() || sym.len() > 20 {
        error!("Invalid symbol length: {}", sym.len());
        return Err(Error::RollbackTransaction);
    }
    
    if xchange.is_empty() || xchange.len() > 50 {
        error!("Invalid exchange length: {}", xchange.len());
        return Err(Error::RollbackTransaction);
    }

    let retry_strategy = ExponentialBackoff::from_millis(10).map(jitter).take(3);

    Retry::spawn(retry_strategy, || async {
        let mut connection = get_timescale_connection(pool.clone())
            .await
            .map_err(|e| {
                error!("Failed to get database connection: {}", e);
                Error::DatabaseError(
                    diesel::result::DatabaseErrorKind::UnableToSendCommand,
                    Box::new(e.to_string())
                )
            })?;
            
        let result = historical_orders
            .filter((symbol.eq(sym)).and(exchange.eq(xchange)))
            .order(timestamp.asc())
            .limit(100000) // Prevent memory exhaustion
            .load(&mut connection)
            .await
            .map_err(|e| {
                error!("Error fetching historical orders: {}", e);
                e
            })?;
            
        info!("Fetched {} historical orders in {}ms", result.len(), start_time.elapsed().as_millis());
        Ok(result)
    }).await
}

/// Get historical orders with randomized sequence for Monte Carlo simulation
/// This function shuffles orders within time windows while preserving market structure
pub async fn get_randomized_historical_orders(
    pool: Arc<deadpool::Pool<AsyncPgConnection>>, 
    sym: &str, 
    xchange: &str,
    window_minutes: i32,  // Time window for shuffling (e.g., 30 minutes)
    seed: Option<u64>     // Random seed for reproducibility
) -> Result<Vec<HistoricalOrder>, Error> {
    use chrono::Duration;
    use rand::SeedableRng;
    use rand::seq::SliceRandom;
    
    let orders = get_historical_orders(pool, sym, xchange).await?;
    
    if orders.is_empty() {
        return Ok(orders);
    }

    // Initialize RNG with seed for reproducibility
    let mut rng = if let Some(s) = seed {
        rand::rngs::StdRng::seed_from_u64(s)
    } else {
        rand::rngs::StdRng::from_entropy()
    };

    // Group orders into time windows and shuffle within each window
    let window_duration = Duration::minutes(window_minutes as i64);
    let start_time = orders[0].timestamp;
    let end_time = orders[orders.len() - 1].timestamp;
    
    let mut randomized_orders = Vec::new();
    let mut current_window_start = start_time;
    
    while current_window_start < end_time {
        let current_window_end = current_window_start + window_duration;
        
        // Extract orders in current window
        let mut window_orders: Vec<HistoricalOrder> = orders
            .iter()
            .filter(|order| {
                let order_time = order.timestamp;
                order_time >= current_window_start && order_time < current_window_end
            })
            .cloned()
            .collect();
            
        // Shuffle orders within this window
        window_orders.shuffle(&mut rng);
        randomized_orders.extend(window_orders);
        
        current_window_start = current_window_end;
    }
    
    Ok(randomized_orders)
}

/// Bootstrap sample historical orders for Monte Carlo simulation
/// Creates a new sequence by sampling with replacement from historical data
pub async fn get_bootstrap_historical_orders(
    pool: Arc<deadpool::Pool<AsyncPgConnection>>,
    sym: &str,
    xchange: &str,
    sample_size: usize,  // Number of orders to sample
    block_size: Option<usize>, // Block bootstrap size (None for simple bootstrap)
    seed: Option<u64>
) -> Result<Vec<HistoricalOrder>, Error> {
    use rand::{Rng, SeedableRng};
    use rand::seq::SliceRandom;
    
    let original_orders = get_historical_orders(pool, sym, xchange).await?;
    
    if original_orders.is_empty() {
        return Ok(vec![]);
    }

    let mut rng = if let Some(s) = seed {
        rand::rngs::StdRng::seed_from_u64(s)
    } else {
        rand::rngs::StdRng::from_entropy()
    };

    let mut bootstrap_orders = Vec::with_capacity(sample_size);
    
    match block_size {
        Some(block_sz) => {
            // Block bootstrap to preserve autocorrelation
            let num_blocks = (sample_size + block_sz - 1) / block_sz;
            
            for _ in 0..num_blocks {
                let start_idx = rng.gen_range(0..original_orders.len().saturating_sub(block_sz));
                let end_idx = (start_idx + block_sz).min(original_orders.len());
                
                for idx in start_idx..end_idx {
                    if bootstrap_orders.len() < sample_size {
                        bootstrap_orders.push(original_orders[idx].clone());
                    }
                }
            }
        },
        None => {
            // Simple bootstrap - random sampling with replacement
            for _ in 0..sample_size {
                if let Some(order) = original_orders.choose(&mut rng) {
                    bootstrap_orders.push(order.clone());
                }
            }
        }
    }
    
    Ok(bootstrap_orders)
}
