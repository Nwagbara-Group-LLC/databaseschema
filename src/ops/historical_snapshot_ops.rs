use crate::{get_timescale_connection, models::historical_snapshot::{HistoricalSnapshot, NewHistoricalSnapshot}};
use diesel_async::pooled_connection::deadpool;
use diesel_async::AsyncPgConnection;
use diesel::{prelude::*, result::Error, upsert::excluded};
use diesel_async::{AsyncConnection, RunQueryDsl};
use tokio_retry::{strategy::{jitter, ExponentialBackoff}, Retry};
use tracing::{info, error, warn};
use std::sync::Arc;

pub async fn create_historical_snapshot(pool: Arc<deadpool::Pool<AsyncPgConnection>>, snapshots: Vec<NewHistoricalSnapshot>) -> Result<Vec<HistoricalSnapshot>, Error> {
    if snapshots.is_empty() {
        warn!("Attempted to create historical snapshots with empty input");
        return Err(Error::DatabaseError(
            diesel::result::DatabaseErrorKind::Unknown,
            Box::new("Cannot create snapshots with empty input".to_string())
        ));
    }
    
    if snapshots.len() > 10000 {
        warn!("Large batch size for historical snapshots: {}", snapshots.len());
        return Err(Error::DatabaseError(
            diesel::result::DatabaseErrorKind::Unknown,
            Box::new("Batch size too large (max 10000)".to_string())
        ));
    }
    
    info!("Creating {} historical snapshots", snapshots.len());
    use crate::schema::historical_snapshot::dsl::*;

    let retry_strategy = ExponentialBackoff::from_millis(10).map(jitter).take(3);

    Retry::spawn(retry_strategy, || async {
        let mut connection = get_timescale_connection(pool.clone())
            .await
            .expect("Error connecting to database");

        // Process in smaller batches to reduce deadlock probability
        const BATCH_SIZE: usize = 100;
        let mut all_results = Vec::with_capacity(snapshots.len());

        for chunk in snapshots.chunks(BATCH_SIZE) {
            // Sort by event_id to ensure consistent lock ordering across pods
            let mut sorted_chunk = chunk.to_vec();
            sorted_chunk.sort_by(|a, b| a.event_id.cmp(&b.event_id));

            let batch_results = connection.transaction::<_, Error, _>(|conn| Box::pin(async {
                // Use DO NOTHING to avoid deadlocks on concurrent inserts
                diesel::insert_into(historical_snapshot)
                    .values(&sorted_chunk)
                    .on_conflict(event_id)
                    .do_nothing()
                    .execute(conn)
                    .await?;

                // Fetch the inserted/updated records
                let order_ids: Vec<String> = sorted_chunk.iter()
                    .map(|snapshot| snapshot.order_id.clone())
                    .collect();
                    
                historical_snapshot
                    .filter(order_id.eq_any(&order_ids))
                    .load::<HistoricalSnapshot>(conn)
                    .await
                    .map_err(|e| {
                        error!("Failed to fetch historical snapshots after insert: {}", e);
                        e
                    })
            })).await?;

            all_results.extend(batch_results);
        }

        Ok(all_results)
    }).await
}

pub async fn get_historical_snapshot(pool: Arc<deadpool::Pool<AsyncPgConnection>>, sym: &str, xchange: &str) -> Result<Vec<HistoricalSnapshot>, Error> {
    if sym.is_empty() || sym.len() > 50 {
        warn!("Invalid symbol length: {} characters", sym.len());
        return Err(Error::DatabaseError(
            diesel::result::DatabaseErrorKind::Unknown,
            Box::new("Symbol must be between 1 and 50 characters".to_string())
        ));
    }
    
    if xchange.is_empty() || xchange.len() > 50 {
        warn!("Invalid exchange length: {} characters", xchange.len());
        return Err(Error::DatabaseError(
            diesel::result::DatabaseErrorKind::Unknown,
            Box::new("Exchange must be between 1 and 50 characters".to_string())
        ));
    }
    
    use crate::schema::historical_snapshot::dsl::*;

    let retry_strategy = ExponentialBackoff::from_millis(10).map(jitter).take(3);

    Retry::spawn(retry_strategy, || async {
        let mut connection = get_timescale_connection(pool.clone())
            .await
            .expect("Error connecting to database");
        historical_snapshot
            .filter((symbol.eq(sym)).and(exchange.eq(xchange)))
            .order(timestamp.asc())
            .load(&mut connection)
            .await
            .map_err(|e| {
                error!("Failed to fetch historical snapshots for symbol {} on exchange {}: {}", sym, xchange, e);
                e
            })
    }).await
}

/// Get historical snapshots with randomized sequence for Monte Carlo simulation
/// This function shuffles snapshots within time windows while preserving market structure
pub async fn get_randomized_historical_snapshots(
    pool: Arc<deadpool::Pool<AsyncPgConnection>>, 
    sym: &str, 
    xchange: &str,
    window_minutes: i32,  // Time window for shuffling (e.g., 30 minutes)
    seed: Option<u64>     // Random seed for reproducibility
) -> Result<Vec<HistoricalSnapshot>, Error> {
    use chrono::Duration;
    use rand::SeedableRng;
    use rand::seq::SliceRandom;
    
    let snapshots = get_historical_snapshot(pool, sym, xchange).await?;
    
    if snapshots.is_empty() {
        return Ok(snapshots);
    }

    // Initialize RNG with seed for reproducibility
    let mut rng = if let Some(s) = seed {
        rand::rngs::StdRng::seed_from_u64(s)
    } else {
        rand::rngs::StdRng::from_entropy()
    };

    // Group snapshots into time windows and shuffle within each window
    let window_duration = Duration::minutes(window_minutes as i64);
    let start_time = snapshots[0].timestamp;
    let end_time = snapshots[snapshots.len() - 1].timestamp;
    
    let mut randomized_snapshots = Vec::new();
    let mut current_window_start = start_time;
    
    while current_window_start < end_time {
        let current_window_end = current_window_start + window_duration;
        
        // Extract snapshots in current window
        let mut window_snapshots: Vec<HistoricalSnapshot> = snapshots
            .iter()
            .filter(|snapshot| {
                let snapshot_time = snapshot.timestamp;
                snapshot_time >= current_window_start && snapshot_time < current_window_end
            })
            .cloned()
            .collect();
            
        // Shuffle snapshots within this window
        window_snapshots.shuffle(&mut rng);
        randomized_snapshots.extend(window_snapshots);
        
        current_window_start = current_window_end;
    }
    
    Ok(randomized_snapshots)
}

/// Bootstrap sample historical snapshots for Monte Carlo simulation
/// Creates a new sequence by sampling with replacement from historical data
pub async fn get_bootstrap_historical_snapshots(
    pool: Arc<deadpool::Pool<AsyncPgConnection>>,
    sym: &str,
    xchange: &str,
    sample_size: usize,  // Number of snapshots to sample
    block_size: Option<usize>, // Block bootstrap size (None for simple bootstrap)
    seed: Option<u64>
) -> Result<Vec<HistoricalSnapshot>, Error> {
    use rand::{Rng, SeedableRng};
    use rand::seq::SliceRandom;
    
    let original_snapshots = get_historical_snapshot(pool, sym, xchange).await?;
    
    if original_snapshots.is_empty() {
        return Ok(vec![]);
    }

    let mut rng = if let Some(s) = seed {
        rand::rngs::StdRng::seed_from_u64(s)
    } else {
        rand::rngs::StdRng::from_entropy()
    };

    let mut bootstrap_snapshots = Vec::with_capacity(sample_size);
    
    match block_size {
        Some(block_sz) => {
            // Block bootstrap to preserve autocorrelation
            let num_blocks = (sample_size + block_sz - 1) / block_sz;
            
            for _ in 0..num_blocks {
                let start_idx = rng.gen_range(0..original_snapshots.len().saturating_sub(block_sz));
                let end_idx = (start_idx + block_sz).min(original_snapshots.len());
                
                for idx in start_idx..end_idx {
                    if bootstrap_snapshots.len() < sample_size {
                        bootstrap_snapshots.push(original_snapshots[idx].clone());
                    }
                }
            }
        },
        None => {
            // Simple bootstrap - random sampling with replacement
            for _ in 0..sample_size {
                if let Some(snapshot) = original_snapshots.choose(&mut rng) {
                    bootstrap_snapshots.push(snapshot.clone());
                }
            }
        }
    }
    
    Ok(bootstrap_snapshots)
}
