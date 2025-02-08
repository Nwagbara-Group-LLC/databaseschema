use crate::{get_timescale_connection, models::sim_trade::{NewSimTrade, SimTrade}, CustomAsyncPgConnectionManager};
use deadpool::managed::Pool;
use diesel::{prelude::*, result::Error, upsert::excluded};
use diesel_async::RunQueryDsl;
use tokio_retry::{strategy::{jitter, ExponentialBackoff}, Retry};
use std::sync::Arc;

pub async fn create_sim_trades(pool: Arc<Pool<CustomAsyncPgConnectionManager>>, orders: Vec<NewSimTrade>) -> Result<(), Error> {
    println!("Creating Simtrades: {:?}", orders);
    use crate::schema::sim_trades::dsl::*;

    let retry_strategy = ExponentialBackoff::from_millis(10).map(jitter).take(3);

    Retry::spawn(retry_strategy, || async {
        let mut connection = get_timescale_connection(pool.clone())
        .await
        .expect("Error connecting to database");
    let result = diesel::insert_into(sim_trades)
        .values(&orders)
        .on_conflict((created_at, trade_id))
        .do_update()
        .set((
            side.eq(excluded(side)),
            price.eq(excluded(price)),
            quantity.eq(excluded(quantity)),
        ))
        .execute(&mut connection)
        .await;

        match result {
            Ok(_) => {
                println!("Successfully saved new Simtrades");
            },
            Err(e) => {
                eprintln!("Error saving new Simtrades: {}", e);
            }
        }
        Ok(())
    }).await
}

pub async fn get_sim_trades_by_symbol(pool: Arc<Pool<CustomAsyncPgConnectionManager>>, sym: &str) -> Result<Vec<SimTrade>, Error> {
    println!("Getting Simtrades by symbol");
    use crate::schema::sim_trades::dsl::*;

    let retry_strategy = ExponentialBackoff::from_millis(10).map(jitter).take(3);

    Retry::spawn(retry_strategy, || async {
        let mut connection = get_timescale_connection(pool.clone())
            .await
            .expect("Error connecting to database");
        sim_trades
            .filter(symbol.eq(sym))
            .select(SimTrade::as_select()) // Ensure the fields match
            .load::<SimTrade>(&mut connection)
            .await
            .map_err(|e| {
                eprintln!("Error loading Simtrades: {}", e);
                e
            })
    }).await
}