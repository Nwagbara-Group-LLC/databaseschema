use std::sync::Arc;
use crate::{get_timescale_connection, models::order_book::{NewOrderBook, OrderBook}};
use bigdecimal::BigDecimal;
use diesel_async::pooled_connection::deadpool;
use diesel_async::AsyncPgConnection;
use diesel::{prelude::*, result::Error};
use diesel_async::RunQueryDsl;
use tokio_retry::{strategy::{jitter, ExponentialBackoff}, Retry};
use uuid::Uuid;
use tracing::{info, error, debug};
use std::time::Instant;

pub async fn create_orderbook(pool: Arc<deadpool::Pool<AsyncPgConnection>>, orderbook: NewOrderBook) -> Result<OrderBook, Error> {
    let start_time = Instant::now();
    debug!("Creating orderbook: {:?}", orderbook);
    use crate::schema::order_books::dsl::*;

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
        
    diesel::insert_into(order_books)
        .values(&orderbook)
        .on_conflict(order_book_id)
        .do_update()
        .set(&orderbook)
        .execute(&mut connection)
        .await?;

    let result = order_books
        .filter(security_id.eq(&orderbook.security_id))
        .first(&mut connection)
        .await
        .map_err(|e| {
            error!("Error fetching new orderbook: {}", e);
            e
        })?;
        
    debug!("Orderbook created in {}ms", start_time.elapsed().as_millis());
    Ok(result)
    })
        .await
}

pub async fn get_orderbooks(pool: Arc<deadpool::Pool<AsyncPgConnection>>) -> Result<Vec<OrderBook>, Error> {
    let start_time = Instant::now();
    info!("Getting all orderbooks");
    use crate::schema::order_books::dsl::*;

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
        
    let result = order_books
        .limit(10000) // Prevent memory exhaustion
        .load::<OrderBook>(&mut connection)
        .await
        .map_err(|e| {
            error!("Error loading orderbooks: {}", e);
            e
        })?;
        
    info!("Fetched {} orderbooks in {}ms", result.len(), start_time.elapsed().as_millis());
    Ok(result)
    }).await
}

pub async fn update_orderbook(pool: Arc<deadpool::Pool<AsyncPgConnection>>, orderbook: OrderBook, volume: BigDecimal) -> Result<OrderBook, Error> {
    let start_time = Instant::now();
    debug!("Updating orderbook: {:?}", orderbook);
    use crate::schema::order_books::dsl::*;

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
            
        let result = diesel::update(order_books.find(orderbook.order_book_id))
            .set(total_volume.eq(volume.clone()))
            .get_result(&mut connection)
            .await
            .map_err(|e| {
                error!("Error updating orderbook: {}", e);
                e
            })?;
            
        debug!("Orderbook updated in {}ms", start_time.elapsed().as_millis());
        Ok(result)
    })
    .await
}

pub async fn get_orderbook_by_orderbook_id(pool: Arc<deadpool::Pool<AsyncPgConnection>>, o_id: Uuid) -> Result<OrderBook, Error> {
    let start_time = Instant::now();
    info!("Getting orderbook by id: {}", o_id);
    use crate::schema::order_books::dsl::*;

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
        
    let result = order_books
        .filter(order_book_id.eq(o_id))
        .first::<OrderBook>(&mut connection)
        .await
        .map_err(|e| {
            error!("Error loading orderbook: {}", e);
            e
        })?;
        
    debug!("Fetched orderbook in {}ms", start_time.elapsed().as_millis());
    Ok(result)
    }).await
}

pub async fn get_orderbook_by_exchange_id_and_security_id(pool: Arc<deadpool::Pool<AsyncPgConnection>>, e_id: Uuid, s_id: Uuid) -> Result<OrderBook, Error> {
    let start_time = Instant::now();
    info!("Getting orderbook by exchange_id: {} and security_id: {}", e_id, s_id);
    use crate::schema::order_books::dsl::*;

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
        
    let result = order_books
        .filter(exchange_id.eq(e_id).and(security_id.eq(s_id)))
        .first::<OrderBook>(&mut connection)
        .await
        .map_err(|e| {
            error!("Error loading orderbook: {}", e);
            e
        })?;
        
    debug!("Fetched orderbook in {}ms", start_time.elapsed().as_millis());
    Ok(result)
    }).await
}

pub async fn orderbook_exists(pool: Arc<deadpool::Pool<AsyncPgConnection>>, s_id: Uuid) -> bool {
    let start_time = Instant::now();
    info!("Checking if orderbook exists for security_id: {}", s_id);
    use crate::schema::order_books::dsl::*;

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
        
    order_books
        .filter(security_id.eq(s_id))
        .first::<OrderBook>(&mut connection)
        .await
        .map_err(|e| {
            debug!("Orderbook not found or error loading: {}", e);
            e
        })
    }).await.is_ok();
    
    debug!("Orderbook existence check completed in {}ms: {}", start_time.elapsed().as_millis(), result);
    result
}
