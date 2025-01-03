use std::sync::Arc;

use crate::{get_connection, models::security::{NewSecurity, Security}, CustomAsyncPgConnectionManager};
use deadpool::managed::Pool;
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use tokio_retry::{strategy::FixedInterval, Retry};

pub async fn create_security(pool: Arc<Pool<CustomAsyncPgConnectionManager>>, new_security: NewSecurity) -> Security {
    println!("Creating security: {:?}", new_security);
    use crate::schema::securities::dsl::*;

    let retry_strategy = FixedInterval::from_millis(1).take(15);

    Retry::spawn(retry_strategy, || async {
        let mut connection = get_connection(pool.clone()).await.expect("Error connecting to database");
        diesel::insert_into(securities)
        .values(&new_security)
        .returning(Security::as_returning())
        .get_result(&mut connection)
        .await
        .map_err(|e| {
            eprintln!("Error saving new security: {}", e);
            e
        })
    }).await.expect("Error creating security")
}

pub async fn get_securities(pool: Arc<Pool<CustomAsyncPgConnectionManager>>) -> Vec<Security> {
    println!("Getting securities");
    use crate::schema::securities::dsl::*;

    let retry_strategy = FixedInterval::from_millis(1).take(15);

    Retry::spawn(retry_strategy, || async {
        let mut connection = get_connection(pool.clone()).await.expect("Error connecting to database");
    securities
        .load::<Security>(&mut connection)
        .await
        .map_err(|e| {
            eprintln!("Error loading securities: {}", e);
            e
        })
    }).await.expect("Error getting securities")
}

pub async fn get_securities_by_id(pool: Arc<Pool<CustomAsyncPgConnectionManager>>, get_security: Security) -> Security {
    println!("Getting security");
    use crate::schema::securities::dsl::*;

    let retry_strategy = FixedInterval::from_millis(1).take(15);

    Retry::spawn(retry_strategy, || async {
        let mut connection = get_connection(pool.clone()).await.expect("Error connecting to database");
    securities
        .find(get_security.security_id)
        .first::<Security>(&mut connection)
        .await
        .map_err(|e| {
            eprintln!("Error loading security: {}", e);
            e
        })
    }).await.expect("Error getting security")
}

pub async fn get_security_by_symbol(pool: Arc<Pool<CustomAsyncPgConnectionManager>>, sym: &String) -> Security {
    println!("Getting security");
    use crate::schema::securities::dsl::*;

    let retry_strategy = FixedInterval::from_millis(1).take(15);

    Retry::spawn(retry_strategy, || async {
        let mut connection = get_connection(pool.clone()).await.expect("Error connecting to database");
    securities
        .filter(symbol.eq(sym))
        .first::<Security>(&mut connection)
        .await
        .map_err(|e| {
            eprintln!("Error loading security: {}", e);
            e
        })
    }).await.expect("Error getting security")

    
}

pub async fn security_exists(pool: Arc<Pool<CustomAsyncPgConnectionManager>>, sym: &String) -> bool {
    println!("Checking if security exists");
    use crate::schema::securities::dsl::*;

    let retry_strategy = FixedInterval::from_millis(1).take(15);

    Retry::spawn(retry_strategy, || async {
        let mut connection = get_connection(pool.clone()).await.expect("Error connecting to database");
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