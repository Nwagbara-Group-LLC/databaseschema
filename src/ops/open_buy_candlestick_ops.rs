use crate::{get_connection, models::open_buy_candlestick::OpenBuyCandlestick, CustomAsyncPgConnectionManager};
use deadpool::managed::Pool;
use diesel_async::RunQueryDsl;
use tokio_retry::{strategy::FixedInterval, Retry};
use std::sync::Arc;

pub async fn get_open_buy_candlesticks(pool: Arc<Pool<CustomAsyncPgConnectionManager>>) -> Vec<OpenBuyCandlestick> {
    println!("Getting open buy candlesticks");
    use crate::schema::open_buy_candlestick_agg::dsl::*;

    let retry_strategy = FixedInterval::from_millis(1).take(15);

    Retry::spawn(retry_strategy, || async {
        let mut connection = get_connection(pool.clone())
        .await
        .expect("Error connecting to database");
    open_buy_candlestick_agg
        .load::<OpenBuyCandlestick>(&mut connection)
        .await
        .map_err(|e| {
            eprintln!("Error loading open buy candlesticks: {}", e);
            e
        })
    }).await.expect("Error getting open buy candlesticks")
}