use crate::{get_timescale_connection, models::trade::{NewTrade, Trade}};
use diesel_async::pooled_connection::deadpool;
use diesel_async::AsyncPgConnection;
use diesel::{prelude::*, result::Error, upsert::excluded};
use diesel_async::{AsyncConnection, RunQueryDsl};
use tokio_retry::{strategy::{jitter, ExponentialBackoff}, Retry};
use std::sync::Arc;

pub async fn create_trades(pool: Arc<deadpool::Pool<AsyncPgConnection>>, new_trades: Vec<NewTrade>) -> Result<(), Error> {
    use crate::schema::trades::dsl::*;

    // Security: Validate batch size to prevent resource exhaustion
    const MAX_BATCH_SIZE: usize = 50000;
    if new_trades.len() > MAX_BATCH_SIZE {
        return Err(Error::RollbackTransaction);
    }

    if new_trades.is_empty() {
        return Ok(());
    }

    let retry_strategy = ExponentialBackoff::from_millis(10).map(jitter).take(3);

    Retry::spawn(retry_strategy, || async {
        let mut connection = get_timescale_connection(pool.clone())
        .await
        .map_err(|_e| {
            Error::DatabaseError(
                diesel::result::DatabaseErrorKind::UnableToSendCommand,
                Box::new("Failed to get database connection".to_string())
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
                        e
                    })
            })).await?;
        }
        
        Ok(())
    }).await
}

pub async fn get_trades_by_symbol(pool: Arc<deadpool::Pool<AsyncPgConnection>>, sym: &str, xchange: &str) -> Result<Vec<Trade>, Error> {
    use crate::schema::trades::dsl::*;

    // Security: Input validation
    if sym.is_empty() || sym.len() > 20 {
        return Err(Error::RollbackTransaction);
    }
    
    if xchange.is_empty() || xchange.len() > 50 {
        return Err(Error::RollbackTransaction);
    }

    let retry_strategy = ExponentialBackoff::from_millis(10).map(jitter).take(3);

    Retry::spawn(retry_strategy, || async {
        let mut connection = get_timescale_connection(pool.clone())
            .await
            .map_err(|_e| {
                Error::DatabaseError(
                    diesel::result::DatabaseErrorKind::UnableToSendCommand,
                    Box::new("Failed to get database connection".to_string())
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
                e
            })?;
            
        Ok(result)
    }).await
}
