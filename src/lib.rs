pub mod ops;
pub mod schema;
pub mod models;
pub mod errors;

use anyhow::Result;
use diesel_async::AsyncPgConnection;
use diesel_async::pooled_connection::{AsyncDieselConnectionManager, deadpool};
use dotenv::dotenv;
use tokio_retry::{strategy::{jitter, ExponentialBackoff}, Retry};
use std::{env, sync::Arc};

/// Function to create a connection pool using diesel-async's built-in manager
pub fn create_timescale_connection_pool() -> deadpool::Pool<AsyncPgConnection> {
    dotenv().ok();
    println!("Creating database connection pool");

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let config = AsyncDieselConnectionManager::<AsyncPgConnection>::new(database_url);

    deadpool::Pool::builder(config)
        .max_size(10) // Limit to 10 connections per pod (3 pods × 10 = 30 total)
        .build()
        .expect("Failed to create database pool")
}

/// Function to get a connection from the pool
pub async fn get_timescale_connection(
    pool: Arc<deadpool::Pool<AsyncPgConnection>>,
) -> Result<deadpool::Object<AsyncPgConnection>> {
    println!("Getting database connection from pool");
    let retry_strategy = ExponentialBackoff::from_millis(10).map(jitter).take(3);

    Retry::spawn(retry_strategy, || async {
        Ok(pool.get().await.map_err(|e| {
            eprintln!("Failed to get database connection from pool: {}", e);
            e
        }).expect("Failed to get database connection from pool"))
    })
    .await
}