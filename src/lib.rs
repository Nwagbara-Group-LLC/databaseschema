pub mod models;
pub mod ops;
mod schema;

use anyhow::Result;
use deadpool::managed::{Manager, Metrics, Object, Pool, RecycleResult};
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use dotenv::dotenv;
use native_tls::TlsConnector;
use postgres_native_tls::MakeTlsConnector;
use tokio_retry::{strategy::FixedInterval, Retry};
use std::{env, sync::Arc};
use tokio_postgres::Config;

// Define a custom connection manager for `AsyncPgConnection`
pub struct CustomAsyncPgConnectionManager {
    database_url: String,
}

impl CustomAsyncPgConnectionManager {
    pub fn new(database_url: String) -> Self {
        Self { database_url }
    }

    async fn create_connection(&self) -> Result<AsyncPgConnection> {
        let config = self
            .database_url
            .parse::<Config>()
            .expect("Failed to parse database URL");

        let tls_connector = TlsConnector::new().expect("Failed to create TLS connector");
        let tls = MakeTlsConnector::new(tls_connector);

        // Connect using tokio-postgres
        let (client, connection) = config.connect(tls).await?;

        // Spawn a background task to manage the connection's lifecycle
        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("connection error: {}", e);
            }
        });

        // Convert the `tokio-postgres` client to `AsyncPgConnection`
        let async_pg_conn = AsyncPgConnection::try_from(client).await?;
        Ok(async_pg_conn)
    }
}

// Implement the Manager trait for deadpool
impl Manager for CustomAsyncPgConnectionManager {
    type Type = AsyncPgConnection; // Define the type for the connection
    type Error = anyhow::Error; // Define the error type

    async fn create(&self) -> Result<Self::Type> {
        self.create_connection().await.map_err(|e| {
            eprintln!("Failed to create connection: {}", e);
            e
        })
    }

    async fn recycle(
        &self,
        conn: &mut Self::Type,
        _metrics: &Metrics,
    ) -> RecycleResult<Self::Error> {
        let query_result = diesel::sql_query("SELECT 1")
            .execute(conn)
            .await;

        match query_result {
            Ok(_) => Ok(()),
            Err(e) => {
                eprintln!("Connection recycle failed: {}", e);
                Err(anyhow::Error::from(e).into())
            }
        }
    }
}

/// Function to create a connection pool
pub fn establish_connection_pool() -> Pool<CustomAsyncPgConnectionManager> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let manager = CustomAsyncPgConnectionManager::new(database_url);

    Pool::builder(manager)
        .build()
        .expect("Failed to create pool")
}

/// Function to get a connection from the pool
pub async fn get_connection(
    pool: Arc<Pool<CustomAsyncPgConnectionManager>>,
) -> Result<Object<CustomAsyncPgConnectionManager>> {
    let retry_strategy = FixedInterval::from_millis(1).take(15);

    Retry::spawn(retry_strategy, || async {
        Ok(pool.get().await.map_err(|e| {
            eprintln!("Failed to get connection from pool: {}", e);
            e
        }).expect("Failed to get connection from pool"))
    })
    .await
}