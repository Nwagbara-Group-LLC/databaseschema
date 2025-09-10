use std::sync::Arc;
use crate::{get_timescale_connection, models::order_book::{NewOrderBook, OrderBook}};
use bigdecimal::BigDecimal;
use diesel_async::pooled_connection::deadpool;
use diesel_async::AsyncPgConnection;
use diesel::{prelude::*, result::Error};
use diesel_async::RunQueryDsl;
use tokio_retry::{strategy::{jitter, ExponentialBackoff}, Retry};
use uuid::Uuid;
use ultra_logger::UltraLogger;
use std::time::Instant;

pub async fn create_orderbook(pool: Arc<deadpool::Pool<AsyncPgConnection>>, orderbook: NewOrderBook) -> Result<OrderBook, Error> {
    let start_time = Instant::now();
    let logger = UltraLogger::new("databaseschema".to_string());
    let _ = logger.debug(format!("Creating orderbook: {:?}", orderbook)).await;
    use crate::schema::order_books::dsl::*;

    let retry_strategy = ExponentialBackoff::from_millis(10).map(jitter).take(3);

    let result = Retry::spawn(retry_strategy, || async {
        let mut connection = get_timescale_connection(pool.clone())
        .await
        .map_err(|e| {
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
            e
        })?;
        
    Ok(result)
    })
        .await;
    
    match result {
        Ok(orderbook) => {
            let logger = UltraLogger::new("databaseschema".to_string());
            let _ = logger.debug(format!("Orderbook created in {}ms", start_time.elapsed().as_millis())).await;
            Ok(orderbook)
        }
        Err(e) => Err(e)
    }
}

pub async fn get_orderbooks(pool: Arc<deadpool::Pool<AsyncPgConnection>>) -> Result<Vec<OrderBook>, Error> {
    let start_time = Instant::now();
    let logger = UltraLogger::new("databaseschema".to_string());
    let _ = logger.info(format!("Getting all orderbooks")).await;
    use crate::schema::order_books::dsl::*;

    let retry_strategy = ExponentialBackoff::from_millis(10).map(jitter).take(3);

    let result = Retry::spawn(retry_strategy, || async {
        let mut connection = get_timescale_connection(pool.clone())
        .await
        .map_err(|e| {
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
            e
        })?;
        
    Ok(result)
    }).await;
    
    match result {
        Ok(orderbooks) => {
            let logger = UltraLogger::new("databaseschema".to_string());
            let _ = logger.info(format!("Fetched {} orderbooks in {}ms", orderbooks.len(), start_time.elapsed().as_millis())).await;
            Ok(orderbooks)
        }
        Err(e) => Err(e)
    }
}

pub async fn update_orderbook(pool: Arc<deadpool::Pool<AsyncPgConnection>>, orderbook: OrderBook, volume: BigDecimal) -> Result<OrderBook, Error> {
    let start_time = Instant::now();
    let logger = UltraLogger::new("databaseschema".to_string()); let _ = logger.debug(format!("Updating orderbook: {:?}", orderbook)).await;
    use crate::schema::order_books::dsl::*;

    let retry_strategy = ExponentialBackoff::from_millis(10).map(jitter).take(3);

    let result = Retry::spawn(retry_strategy, || async {
        let mut connection = get_timescale_connection(pool.clone())
            .await
            .map_err(|e| {
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
                e
            })?;
            
        Ok(result)
    })
    .await;
    
    match result {
        Ok(orderbook) => {
            let logger = UltraLogger::new("databaseschema".to_string()); 
            let _ = logger.debug(format!("Orderbook updated in {}ms", start_time.elapsed().as_millis())).await;
            Ok(orderbook)
        }
        Err(e) => Err(e)
    }
}

pub async fn get_orderbook_by_orderbook_id(pool: Arc<deadpool::Pool<AsyncPgConnection>>, o_id: Uuid) -> Result<OrderBook, Error> {
    let start_time = Instant::now();
    let logger = UltraLogger::new("databaseschema".to_string()); let _ = logger.info(format!("Getting orderbook by id: {}", o_id)).await;
    use crate::schema::order_books::dsl::*;

    let retry_strategy = ExponentialBackoff::from_millis(10).map(jitter).take(3);

    let result = Retry::spawn(retry_strategy, || async {
        let mut connection = get_timescale_connection(pool.clone())
        .await
        .map_err(|e| {
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
            e
        })?;
        
    Ok(result)
    }).await;
    
    match result {
        Ok(orderbook) => {
            let logger = UltraLogger::new("databaseschema".to_string()); 
            let _ = logger.debug(format!("Fetched orderbook in {}ms", start_time.elapsed().as_millis())).await;
            Ok(orderbook)
        }
        Err(e) => Err(e)
    }
}

pub async fn get_orderbook_by_exchange_id_and_security_id(pool: Arc<deadpool::Pool<AsyncPgConnection>>, e_id: Uuid, s_id: Uuid) -> Result<OrderBook, Error> {
    let start_time = Instant::now();
    let logger = UltraLogger::new("databaseschema".to_string()); let _ = logger.info(format!("Getting orderbook by exchange_id: {} and security_id: {}", e_id, s_id)).await;
    use crate::schema::order_books::dsl::*;

    let retry_strategy = ExponentialBackoff::from_millis(10).map(jitter).take(3);

    let result = Retry::spawn(retry_strategy, || async {
        let mut connection = get_timescale_connection(pool.clone())
        .await
        .map_err(|e| {
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
            e
        })?;
        
    Ok(result)
    }).await;
    
    match result {
        Ok(orderbook) => {
            let logger = UltraLogger::new("databaseschema".to_string()); 
            let _ = logger.debug(format!("Fetched orderbook in {}ms", start_time.elapsed().as_millis())).await;
            Ok(orderbook)
        }
        Err(e) => Err(e)
    }
}

pub async fn orderbook_exists(pool: Arc<deadpool::Pool<AsyncPgConnection>>, s_id: Uuid) -> bool {
    let start_time = Instant::now();
    let logger = UltraLogger::new("databaseschema".to_string()); let _ = logger.info(format!("Checking if orderbook exists for security_id: {}", s_id)).await;
    use crate::schema::order_books::dsl::*;

    let retry_strategy = ExponentialBackoff::from_millis(10).map(jitter).take(3);

    let result = Retry::spawn(retry_strategy, || async {
        let mut connection = get_timescale_connection(pool.clone())
        .await
        .map_err(|e| {
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
            e
        })
    }).await.is_ok();
    
    let logger = UltraLogger::new("databaseschema".to_string()); 
    let _ = logger.debug(format!("Orderbook existence check completed in {}ms: {}", start_time.elapsed().as_millis(), result)).await;
    result
}
