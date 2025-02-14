use crate::{get_timescale_connection, models::historical_order::{HistoricalOrder, NewHistoricalOrder}, CustomAsyncPgConnectionManager};
use deadpool::managed::Pool;
use diesel::{prelude::*, result::Error};
use diesel_async::RunQueryDsl;
use tokio_retry::{strategy::{jitter, ExponentialBackoff}, Retry};
use std::sync::Arc;

pub async fn create_historical_order(pool: Arc<Pool<CustomAsyncPgConnectionManager>>, historical_order: NewHistoricalOrder) -> Result<HistoricalOrder, Error> {
    println!("Creating historical order: {:?}", historical_order);
    use crate::schema::historical_orders::dsl::*;

    let retry_strategy = ExponentialBackoff::from_millis(10).map(jitter).take(3);

    Retry::spawn(retry_strategy, || async {
        let mut connection = get_timescale_connection(pool.clone())
            .await
            .expect("Error connecting to database");
        diesel::insert_into(historical_orders)
            .values(&historical_order)
            .on_conflict(event_id)
            .do_update()
            .set(&historical_order)
            .execute(&mut connection)
            .await?;

        historical_orders
            .filter(order_id.eq(&historical_order.order_id))
            .first(&mut connection)
            .await
            .map_err(|e| {
                eprintln!("Error fetching new historical order: {}", e);
                e
            })
    }).await
}

pub async fn get_historical_orders(pool: Arc<Pool<CustomAsyncPgConnectionManager>>) -> Result<Vec<HistoricalOrder>, Error> {
    use crate::schema::historical_orders::dsl::*;

    let retry_strategy = ExponentialBackoff::from_millis(10).map(jitter).take(3);

    Retry::spawn(retry_strategy, || async {
        let mut connection = get_timescale_connection(pool.clone())
            .await
            .expect("Error connecting to database");
        historical_orders
            .order(timestamp.asc())
            .load(&mut connection)
            .await
            .map_err(|e| {
                eprintln!("Error fetching historical orders: {}", e);
                e
            })
    }).await
}