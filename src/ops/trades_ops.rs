use crate::{get_timescale_connection, models::trade::{NewTrade, Trade}};
use diesel_async::pooled_connection::deadpool;
use diesel_async::AsyncPgConnection;
use diesel::{prelude::*, result::Error, upsert::excluded};
use diesel_async::{AsyncConnection, RunQueryDsl};
use tokio_retry::{strategy::{jitter, ExponentialBackoff}, Retry};
use std::sync::Arc;
use tracing::{info, error, warn};
use std::time::Instant;

pub async fn create_trades(pool: Arc<deadpool::Pool<AsyncPgConnection>>, new_trades: Vec<NewTrade>) -> Result<(), Error> {
    let start_time = Instant::now();
    info!("Creating {} trades", new_trades.len());
    use crate::schema::trades::dsl::*;

    // Security: Validate batch size to prevent resource exhaustion
    const MAX_BATCH_SIZE: usize = 50000;
    if new_trades.len() > MAX_BATCH_SIZE {
        error!("Batch size {} exceeds maximum {}", new_trades.len(), MAX_BATCH_SIZE);
        return Err(Error::RollbackTransaction);
    }

    if new_trades.is_empty() {
        warn!("create_trades called with empty trades vector");
        return Ok(());
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

        // Process in batches to handle large datasets
        const BATCH_SIZE: usize = 1000;
        
        for chunk in new_trades.chunks(BATCH_SIZE) {
            connection.transaction::<_, Error, _>(|conn| Box::pin(async {
                diesel::insert_into(trades)
                    .values(chunk)
                    .on_conflict((created_at, trade_id))
                    .do_update()
                    .set((
                        side.eq(excluded(side)),
                        price.eq(excluded(price)),
                        quantity.eq(excluded(quantity)),
                    ))
                    .execute(conn)
                    .await
                    .map_err(|e| {
                        error!("Error saving trades batch: {}", e);
                        e
                    })
            })).await?;
        }
        
        info!("Successfully saved {} trades in {}ms", new_trades.len(), start_time.elapsed().as_millis());
        Ok(())
    }).await
}

pub async fn get_trades_by_symbol(pool: Arc<deadpool::Pool<AsyncPgConnection>>, sym: &str, xchange: &str) -> Result<Vec<Trade>, Error> {
    let start_time = Instant::now();
    info!("Getting trades by symbol: {} on exchange: {}", sym, xchange);
    use crate::schema::trades::dsl::*;

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
            
        let result = trades
            .filter(symbol.eq(sym).and(exchange.eq(xchange)))
            .order(created_at.asc())
            .limit(100000) // Prevent memory exhaustion
            .select(Trade::as_select()) // Ensure the fields match
            .load::<Trade>(&mut connection)
            .await
            .map_err(|e| {
                error!("Error loading trades: {}", e);
                e
            })?;
            
        info!("Fetched {} trades in {}ms", result.len(), start_time.elapsed().as_millis());
        Ok(result)
    }).await
}
