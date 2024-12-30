use crate::{get_connection, models::open_sell_candlestick::OpenSellCandlestick, CustomAsyncPgConnectionManager};
use deadpool::managed::Pool;
use diesel_async::RunQueryDsl;
use tokio_retry::{strategy::FixedInterval, Retry};
use std::sync::Arc;

pub async fn get_open_sell_candlesticks(pool: Arc<Pool<CustomAsyncPgConnectionManager>>) -> Vec<OpenSellCandlestick> {
    println!("Getting open sell candlesticks");
    use crate::schema::open_sell_candlestick_agg::dsl::*;

    let retry_strategy = FixedInterval::from_millis(1).take(15);

    Retry::spawn(retry_strategy, || async {
        let mut connection = get_connection(pool.clone())
        .await
        .expect("Error connecting to database");
    open_sell_candlestick_agg
        .load::<OpenSellCandlestick>(&mut connection)
        .await
        .map_err(|e| {
            eprintln!("Error loading open sell candlesticks: {}", e);
            e
        })
    }).await.expect("Error getting open sell candlesticks")
}