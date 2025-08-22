use crate::{get_timescale_connection, models::historical_order::{HistoricalOrder, NewHistoricalOrder}, CustomAsyncPgConnectionManager};
use deadpool::managed::Pool;
use diesel::{prelude::*, result::Error, upsert::excluded};
use diesel_async::RunQueryDsl;
use tokio_retry::{strategy::{jitter, ExponentialBackoff}, Retry};
use std::sync::Arc;

pub async fn create_historical_order(pool: Arc<Pool<CustomAsyncPgConnectionManager>>, historical_order: NewHistoricalOrder) -> Result<HistoricalOrder, Error> {
    println!("Creating historical order: {:?}", historical_order);
    use crate::schema::historical_orders::dsl::*;

    let retry_strategy = ExponentialBackoff::from_millis(10).map(jitter).take(3);

    Retry::spawn(retry_strategy, || async {
        let mut connection = get_timescale_connection(pool.clone())
            .await
            .expect("Error connecting to database");
        diesel::insert_into(historical_orders)
            .values(&historical_order)
            .on_conflict(event_id)
            .do_update()
            .set(&historical_order)
            .execute(&mut connection)
            .await?;

        historical_orders
            .filter(order_id.eq(&historical_order.get_order_id()))
            .first(&mut connection)
            .await
            .map_err(|e| {
                eprintln!("Error fetching new historical order: {}", e);
                e
            })
    }).await
}

pub async fn create_historical_orders(pool: Arc<Pool<CustomAsyncPgConnectionManager>>, orders: Vec<NewHistoricalOrder>) -> Result<Vec<HistoricalOrder>, Error> {
    println!("Creating historical orders: {:?}", orders);
    use crate::schema::historical_orders::dsl::*;

    let retry_strategy = ExponentialBackoff::from_millis(10).map(jitter).take(3);

    Retry::spawn(retry_strategy, || async {
        let mut connection = get_timescale_connection(pool.clone())
            .await
            .expect("Error connecting to database");
        diesel::insert_into(historical_orders)
            .values(&historical_orders)
            .on_conflict(event_id)
            .do_update()
            .set((
                event_id.eq(excluded(event_id)),
                timestamp.eq(excluded(timestamp)),
                order_id.eq(excluded(order_id)),
                event_type.eq(excluded(event_type)),
                side.eq(excluded(side)),
                price_level.eq(excluded(price_level)),
                quantity.eq(excluded(quantity)),
                prev_price.eq(excluded(prev_price)),
                prev_quantity.eq(excluded(prev_quantity)),
                status.eq(excluded(status)),
                exchange.eq(excluded(exchange)),
                symbol.eq(excluded(symbol)),
            ))
            .execute(&mut connection)
            .await?;

        historical_orders
            .filter(order_id.eq_any(orders.iter().map(|order| order.get_order_id())))
            .load(&mut connection)
            .await
            .map_err(|e| {
                eprintln!("Error fetching new historical orders: {}", e);
                e
            })
    }).await
}

pub async fn get_historical_orders(pool: Arc<Pool<CustomAsyncPgConnectionManager>>, sym: &str, xchange: &str) -> Result<Vec<HistoricalOrder>, Error> {
    use crate::schema::historical_orders::dsl::*;

    let retry_strategy = ExponentialBackoff::from_millis(10).map(jitter).take(3);

    Retry::spawn(retry_strategy, || async {
        let mut connection = get_timescale_connection(pool.clone())
            .await
            .expect("Error connecting to database");
        historical_orders
            .filter((symbol.eq(sym)).and(exchange.eq(xchange)))
            .order(timestamp.asc())
            .load(&mut connection)
            .await
            .map_err(|e| {
                eprintln!("Error fetching historical orders: {}", e);
                e
            })
    }).await
}

/// Get historical orders with randomized sequence for Monte Carlo simulation
/// This function shuffles orders within time windows while preserving market structure
pub async fn get_randomized_historical_orders(
    pool: Arc<Pool<CustomAsyncPgConnectionManager>>, 
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
    pool: Arc<Pool<CustomAsyncPgConnectionManager>>,
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