use std::sync::Arc;
use std::time::Instant;

use crate::{get_timescale_connection, models::security::{NewSecurity, Security}};
use diesel_async::pooled_connection::deadpool;
use diesel_async::AsyncPgConnection;
use diesel::{prelude::*, result::Error};
use diesel_async::RunQueryDsl;
use tokio_retry::{strategy::{jitter, ExponentialBackoff}, Retry};
use ultra_logger::UltraLogger;

pub async fn create_security(pool: Arc<deadpool::Pool<AsyncPgConnection>>, new_security: NewSecurity) -> Result<Security, Error> {
    let start_time = Instant::now();
    let logger = UltraLogger::new("databaseschema".to_string());
    let _ = logger.info(format!("Creating security: {:?}", new_security)).await;
    use crate::schema::securities::dsl::*;

    let retry_strategy = ExponentialBackoff::from_millis(10).map(jitter).take(3);

    Retry::spawn(retry_strategy, || async {
        let mut connection = get_timescale_connection(pool.clone())
        .await
        .map_err(|e| {
            let logger = UltraLogger::new("databaseschema".to_string());
            let _ = logger.error(format!("Failed to get database connection: {}", e));
            Error::DatabaseError(
                diesel::result::DatabaseErrorKind::UnableToSendCommand,
                Box::new(e.to_string())
            )
        })?;
        
        diesel::insert_into(securities)
        .values(&new_security)
        .on_conflict(security_id)
        .do_update()
        .set(&new_security)
        .execute(&mut connection)
        .await?;

        let result = securities
        .filter(symbol.eq(&new_security.symbol))
        .first(&mut connection)
        .await
        .map_err(|e| {
            let logger = UltraLogger::new("databaseschema".to_string());
            let _ = logger.error(format!("Error fetching new security: {}", e));
            e
        })?;
        
        let logger = UltraLogger::new("databaseschema".to_string());
        let _ = logger.debug(format!("Created security in {}ms", start_time.elapsed().as_millis())).await;
        Ok(result)
    }).await
}

pub async fn get_securities(pool: Arc<deadpool::Pool<AsyncPgConnection>>) -> Result<Vec<Security>, Error> {
    let start_time = Instant::now();
    let logger = UltraLogger::new("databaseschema".to_string());
    let _ = logger.info(format!("Getting all securities")).await;
    use crate::schema::securities::dsl::*;

    let retry_strategy = ExponentialBackoff::from_millis(10).map(jitter).take(3);

    Retry::spawn(retry_strategy, || async {
        let mut connection = get_timescale_connection(pool.clone())
        .await
        .map_err(|e| {
            let logger = UltraLogger::new("databaseschema".to_string());
            let _ = logger.error(format!("Failed to get database connection: {}", e));
            Error::DatabaseError(
                diesel::result::DatabaseErrorKind::UnableToSendCommand,
                Box::new(e.to_string())
            )
        })?;
        
    let result = securities
        .load::<Security>(&mut connection)
        .await
        .map_err(|e| {
            let logger = UltraLogger::new("databaseschema".to_string());
            let _ = logger.error(format!("Error loading securities: {}", e));
            e
        })?;
        
        let logger = UltraLogger::new("databaseschema".to_string());
        let _ = logger.debug(format!("Fetched {} securities in {}ms", result.len(), start_time.elapsed().as_millis())).await;
        Ok(result)
    }).await
}

pub async fn get_securities_by_id(pool: Arc<deadpool::Pool<AsyncPgConnection>>, get_security: Security) -> Result<Security, Error> {
    let start_time = Instant::now();
    let logger = UltraLogger::new("databaseschema".to_string());
    let _ = logger.info(format!("Getting security by id: {}", get_security.security_id)).await;
    use crate::schema::securities::dsl::*;

    let retry_strategy = ExponentialBackoff::from_millis(10).map(jitter).take(3);

    Retry::spawn(retry_strategy, || async {
        let mut connection = get_timescale_connection(pool.clone())
        .await
        .map_err(|e| {
            let logger = UltraLogger::new("databaseschema".to_string());
            let _ = logger.error(format!("Failed to get database connection: {}", e));
            Error::DatabaseError(
                diesel::result::DatabaseErrorKind::UnableToSendCommand,
                Box::new(e.to_string())
            )
        })?;
        
    let result = securities
        .find(get_security.security_id)
        .first::<Security>(&mut connection)
        .await
        .map_err(|e| {
            let logger = UltraLogger::new("databaseschema".to_string());
            let _ = logger.error(format!("Error loading security: {}", e));
            e
        })?;
        
        let logger = UltraLogger::new("databaseschema".to_string());
        let _ = logger.debug(format!("Fetched security in {}ms", start_time.elapsed().as_millis())).await;
        Ok(result)
    }).await
}

pub async fn get_security_by_symbol(pool: Arc<deadpool::Pool<AsyncPgConnection>>, sym: &String) -> Result<Security, Error> {
    let start_time = Instant::now();
    let logger = UltraLogger::new("databaseschema".to_string());
    let _ = logger.info(format!("Getting security by symbol: {}", sym)).await;
    
    // Input validation
    if sym.is_empty() || sym.len() > 20 {
        let logger = UltraLogger::new("databaseschema".to_string());
        let _ = logger.error(format!("Invalid symbol: empty or too long (max 20 chars)")).await;
        return Err(Error::NotFound);
    }
    
    use crate::schema::securities::dsl::*;

    let retry_strategy = ExponentialBackoff::from_millis(10).map(jitter).take(3);

    Retry::spawn(retry_strategy, || async {
        let mut connection = get_timescale_connection(pool.clone())
        .await
        .map_err(|e| {
            let logger = UltraLogger::new("databaseschema".to_string());
            let _ = logger.error(format!("Failed to get database connection: {}", e));
            Error::DatabaseError(
                diesel::result::DatabaseErrorKind::UnableToSendCommand,
                Box::new(e.to_string())
            )
        })?;
        
    let result = securities
        .filter(symbol.eq(sym))
        .first::<Security>(&mut connection)
        .await
        .map_err(|e| {
            let logger = UltraLogger::new("databaseschema".to_string());
            let _ = logger.error(format!("Error loading security: {}", e));
            e
        })?;
        
        let logger = UltraLogger::new("databaseschema".to_string());
        let _ = logger.debug(format!("Fetched security by symbol in {}ms", start_time.elapsed().as_millis())).await;
        Ok(result)
    }).await
}

pub async fn security_exists(pool: Arc<deadpool::Pool<AsyncPgConnection>>, sym: &String) -> bool {
    let start_time = Instant::now();
    let logger = UltraLogger::new("databaseschema".to_string());
    let _ = logger.info(format!("Checking if security exists: {}", sym)).await;
    
    // Input validation
    if sym.is_empty() || sym.len() > 20 {
        let logger = UltraLogger::new("databaseschema".to_string());
        let _ = logger.error(format!("Invalid symbol for existence check: empty or too long (max 20 chars)")).await;
        return false;
    }
    
    use crate::schema::securities::dsl::*;

    let retry_strategy = ExponentialBackoff::from_millis(10).map(jitter).take(3);

    let result = Retry::spawn(retry_strategy, || async {
        let mut connection = get_timescale_connection(pool.clone())
        .await
        .map_err(|e| {
            let logger = UltraLogger::new("databaseschema".to_string());
            let _ = logger.error(format!("Failed to get database connection: {}", e));
            Error::DatabaseError(
                diesel::result::DatabaseErrorKind::UnableToSendCommand,
                Box::new(e.to_string())
            )
        })?;
        
    securities
        .filter(symbol.eq(sym))
        .first::<Security>(&mut connection)
        .await
        .map_err(|e| {
            let logger = UltraLogger::new("databaseschema".to_string());
            let _ = logger.debug(format!("Security not found or error loading: {}", e));
            e
        })
    }).await.is_ok();
    
    let logger = UltraLogger::new("databaseschema".to_string());
    let _ = logger.debug(format!("Security existence check completed in {}ms: {}", start_time.elapsed().as_millis(), result)).await;
    result
}
