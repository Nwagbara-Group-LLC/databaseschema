use std::sync::Arc;

use crate::{get_timescale_connection, models::security::{NewSecurity, Security}, CustomAsyncPgConnectionManager};
use deadpool::managed::Pool;
use diesel::{prelude::*, result::Error};
use diesel_async::RunQueryDsl;
use tokio_retry::{strategy::{jitter, ExponentialBackoff}, Retry};

pub async fn create_security(pool: Arc<Pool<CustomAsyncPgConnectionManager>>, new_security: NewSecurity) -> Result<Security, Error> {
    println!("Creating security: {:?}", new_security);
    use crate::schema::securities::dsl::*;

    let retry_strategy = ExponentialBackoff::from_millis(10).map(jitter).take(3);

    Retry::spawn(retry_strategy, || async {
        let mut connection = get_timescale_connection(pool.clone()).await.expect("Error connecting to database");
        diesel::insert_into(securities)
        .values(&new_security)
        .on_conflict(security_id)
        .do_update()
        .set(&new_security)
        .execute(&mut connection)
        .await?;

        securities
        .filter(symbol.eq(&new_security.symbol))
        .first(&mut connection)
        .await
        .map_err(|e| {
            eprintln!("Error fetching new security: {}", e);
            e
        })
    }).await
}

pub async fn get_securities(pool: Arc<Pool<CustomAsyncPgConnectionManager>>) -> Result<Vec<Security>, Error> {
    println!("Getting securities");
    use crate::schema::securities::dsl::*;

    let retry_strategy = ExponentialBackoff::from_millis(10).map(jitter).take(3);

    Retry::spawn(retry_strategy, || async {
        let mut connection = get_timescale_connection(pool.clone()).await.expect("Error connecting to database");
    securities
        .load::<Security>(&mut connection)
        .await
        .map_err(|e| {
            eprintln!("Error loading securities: {}", e);
            e
        })
    }).await
}

pub async fn get_securities_by_id(pool: Arc<Pool<CustomAsyncPgConnectionManager>>, get_security: Security) -> Result<Security, Error> {
    println!("Getting security");
    use crate::schema::securities::dsl::*;

    let retry_strategy = ExponentialBackoff::from_millis(10).map(jitter).take(3);

    Retry::spawn(retry_strategy, || async {
        let mut connection = get_timescale_connection(pool.clone()).await.expect("Error connecting to database");
    securities
        .find(get_security.security_id)
        .first::<Security>(&mut connection)
        .await
        .map_err(|e| {
            eprintln!("Error loading security: {}", e);
            e
        })
    }).await
}

pub async fn get_security_by_symbol(pool: Arc<Pool<CustomAsyncPgConnectionManager>>, sym: &String) -> Result<Security, Error> {
    println!("Getting security");
    use crate::schema::securities::dsl::*;

    let retry_strategy = ExponentialBackoff::from_millis(10).map(jitter).take(3);

    Retry::spawn(retry_strategy, || async {
        let mut connection = get_timescale_connection(pool.clone()).await.expect("Error connecting to database");
    securities
        .filter(symbol.eq(sym))
        .first::<Security>(&mut connection)
        .await
        .map_err(|e| {
            eprintln!("Error loading security: {}", e);
            e
        })
    }).await

    
}

pub async fn security_exists(pool: Arc<Pool<CustomAsyncPgConnectionManager>>, sym: &String) -> bool {
    println!("Checking if security exists");
    use crate::schema::securities::dsl::*;

    let retry_strategy = ExponentialBackoff::from_millis(10).map(jitter).take(3);

    Retry::spawn(retry_strategy, || async {
        let mut connection = get_timescale_connection(pool.clone()).await.expect("Error connecting to database");
    securities
        .filter(symbol.eq(sym))
        .first::<Security>(&mut connection)
        .await
        .map_err(|e| {
            eprintln!("Error loading security: {}", e);
            e
        })
    }).await.is_ok()
}