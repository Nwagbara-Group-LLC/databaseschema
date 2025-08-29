use crate::{get_timescale_connection, models::trade::{NewTrade, Trade}, CustomAsyncPgConnectionManager};
use deadpool::managed::Pool;
use diesel::{prelude::*, result::Error, upsert::excluded};
use diesel_async::{AsyncConnection, RunQueryDsl};
use tokio_retry::{strategy::{jitter, ExponentialBackoff}, Retry};
use std::sync::Arc;

pub async fn create_trades(pool: Arc<Pool<CustomAsyncPgConnectionManager>>, new_trades: Vec<NewTrade>) -> Result<(), Error> {
    println!("Creating {} trades", new_trades.len());
    use crate::schema::trades::dsl::*;

    let retry_strategy = ExponentialBackoff::from_millis(10).map(jitter).take(3);

    Retry::spawn(retry_strategy, || async {
        let mut connection = get_timescale_connection(pool.clone())
        .await
        .expect("Error connecting to database");

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
                        eprintln!("Error saving trades batch: {}", e);
                        e
                    })
            })).await?;
        }
        
        println!("Successfully saved {} trades", new_trades.len());
        Ok(())
    }).await
}

pub async fn get_trades_by_symbol(pool: Arc<Pool<CustomAsyncPgConnectionManager>>, sym: &str, xchange: &str) -> Result<Vec<Trade>, Error> {
    println!("Getting trades by symbol");
    use crate::schema::trades::dsl::*;

    let retry_strategy = ExponentialBackoff::from_millis(10).map(jitter).take(3);

    Retry::spawn(retry_strategy, || async {
        let mut connection = get_timescale_connection(pool.clone())
            .await
            .expect("Error connecting to database");
        trades
            .filter(symbol.eq(sym).and(exchange.eq(xchange)))
            .order(created_at.asc())
            .select(Trade::as_select()) // Ensure the fields match
            .load::<Trade>(&mut connection)
            .await
            .map_err(|e| {
                eprintln!("Error loading trades: {}", e);
                e
            })
    }).await
}