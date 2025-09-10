use crate::{get_timescale_connection, models::exchange::{Exchange, NewExchange}};
use diesel_async::pooled_connection::deadpool;
use diesel_async::AsyncPgConnection;
use diesel::{prelude::*, result::Error};
use diesel_async::RunQueryDsl;
use tokio_retry::{strategy::{jitter, ExponentialBackoff}, Retry};
use std::sync::Arc;
use std::time::Instant;
use tracing::{info, error, debug};

pub async fn create_exchange(pool: Arc<deadpool::Pool<AsyncPgConnection>>, new_exchange: NewExchange) -> Result<Exchange, Error> {
    let start_time = Instant::now();
    info!("Creating exchange: {:?}", new_exchange);
    use crate::schema::exchanges::dsl::*;

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
            
        diesel::insert_into(exchanges)
            .values(&new_exchange)
            .on_conflict(exchange_id)
            .do_update()
            .set(&new_exchange)
            .execute(&mut connection)
            .await?;

        let result = exchanges
            .filter(exchange.eq(&new_exchange.exchange))
            .first(&mut connection)
            .await
            .map_err(|e| {
                error!("Error fetching new exchange: {}", e);
                e
            })?;
            
        debug!("Created exchange in {}ms", start_time.elapsed().as_millis());
        Ok(result)
    }).await
}

pub async fn get_exchanges(pool: Arc<deadpool::Pool<AsyncPgConnection>>) -> Result<Vec<Exchange>, Error> {
    let start_time = Instant::now();
    info!("Getting all exchanges");
    use crate::schema::exchanges::dsl::*;

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
            
        let result = exchanges
            .load::<Exchange>(&mut connection)
            .await
            .map_err(|e| {
                error!("Error loading exchanges: {}", e);
                e
            })?;
            
        debug!("Fetched {} exchanges in {}ms", result.len(), start_time.elapsed().as_millis());
        Ok(result)
    }).await
}

pub async fn get_exchanges_by_name(pool: Arc<deadpool::Pool<AsyncPgConnection>>, name: &String) -> Result<Exchange, Error> {
    let start_time = Instant::now();
    info!("Getting exchange by name: {}", name);
    
    // Input validation
    if name.is_empty() || name.len() > 50 {
        error!("Invalid exchange name: empty or too long (max 50 chars)");
        return Err(Error::NotFound);
    }
    
    use crate::schema::exchanges::dsl::*;

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
            
        let result = exchanges
            .filter(exchange.eq(name))
            .first::<Exchange>(&mut connection)
            .await
            .map_err(|e| {
                error!("Error loading exchange: {}", e);
                e
            })?;
            
        debug!("Fetched exchange by name in {}ms", start_time.elapsed().as_millis());
        Ok(result)
    }).await
}

pub async fn exchange_exists(pool: Arc<deadpool::Pool<AsyncPgConnection>>, name: &String) -> bool {
    let start_time = Instant::now();
    info!("Checking if exchange exists: {}", name);
    
    // Input validation
    if name.is_empty() || name.len() > 50 {
        error!("Invalid exchange name for existence check: empty or too long (max 50 chars)");
        return false;
    }
    
    use crate::schema::exchanges::dsl::*;

    let retry_strategy = ExponentialBackoff::from_millis(10).map(jitter).take(3);

    let result = Retry::spawn(retry_strategy, || async {
        let mut connection = get_timescale_connection(pool.clone())
            .await
            .map_err(|e| {
                error!("Failed to get database connection: {}", e);
                Error::DatabaseError(
                    diesel::result::DatabaseErrorKind::UnableToSendCommand,
                    Box::new(e.to_string())
                )
            })?;
            
        exchanges
            .filter(exchange.eq(name))
            .first::<Exchange>(&mut connection)
            .await
            .map_err(|e| {
                debug!("Exchange not found or error loading: {}", e);
                e
            })
    }).await.is_ok();
    
    debug!("Exchange existence check completed in {}ms: {}", start_time.elapsed().as_millis(), result);
    result
}
