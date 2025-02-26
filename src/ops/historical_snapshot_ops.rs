use crate::{get_timescale_connection, models::historical_snapshot::{HistoricalSnapshot, NewHistoricalSnapshot}, CustomAsyncPgConnectionManager};
use deadpool::managed::Pool;
use diesel::{prelude::*, result::Error, upsert::excluded};
use diesel_async::RunQueryDsl;
use tokio_retry::{strategy::{jitter, ExponentialBackoff}, Retry};
use std::sync::Arc;

pub async fn create_historical_snapshot(pool: Arc<Pool<CustomAsyncPgConnectionManager>>, snapshots: Vec<NewHistoricalSnapshot>) -> Result<Vec<HistoricalSnapshot>, Error> {
    println!("Creating historical snapshots: {:?}", snapshots);
    use crate::schema::historical_snapshot::dsl::*;

    let retry_strategy = ExponentialBackoff::from_millis(10).map(jitter).take(3);

    Retry::spawn(retry_strategy, || async {
        let mut connection = get_timescale_connection(pool.clone())
            .await
            .expect("Error connecting to database");
        diesel::insert_into(historical_snapshot)
            .values(&historical_snapshot)
            .on_conflict(event_id)
            .do_update()
            .set((
                event_id.eq(excluded(event_id)),
                timestamp.eq(excluded(timestamp)),
                order_id.eq(excluded(order_id)),
                event_type.eq(excluded(event_type)),
                side.eq(excluded(side)),
                price_level.eq(excluded(price_level)),
                quantity.eq(excluded(quantity)),
                status.eq(excluded(status)),
                exchange.eq(excluded(exchange)),
                symbol.eq(excluded(symbol)),
            ))
            .execute(&mut connection)
            .await?;

        historical_snapshot
            .filter(order_id.eq_any(snapshots.iter().map(|order| order.order_id.clone())))
            .load(&mut connection)
            .await
            .map_err(|e| {
                eprintln!("Error fetching new historical snapshots: {}", e);
                e
            })
    }).await
}

pub async fn get_historical_snapshot(pool: Arc<Pool<CustomAsyncPgConnectionManager>>, sym: &str, xchange: &str) -> Result<Vec<HistoricalSnapshot>, Error> {
    use crate::schema::historical_snapshot::dsl::*;

    let retry_strategy = ExponentialBackoff::from_millis(10).map(jitter).take(3);

    Retry::spawn(retry_strategy, || async {
        let mut connection = get_timescale_connection(pool.clone())
            .await
            .expect("Error connecting to database");
        historical_snapshot
            .filter((symbol.eq(sym)).and(exchange.eq(xchange)))
            .order(timestamp.asc())
            .load(&mut connection)
            .await
            .map_err(|e| {
                eprintln!("Error fetching historical snapshots: {}", e);
                e
            })
    }).await
}