use crate::{get_connection, models::modified_buy_candlestick::ModifiedBuyCandlestick, CustomAsyncPgConnectionManager};
use deadpool::managed::Pool;
use diesel_async::RunQueryDsl;
use tokio_retry::{strategy::FixedInterval, Retry};
use std::sync::Arc;

pub async fn get_modified_buy_candlesticks(pool: Arc<Pool<CustomAsyncPgConnectionManager>>) -> Vec<ModifiedBuyCandlestick> {
    println!("Getting modified buy candlesticks");
    use crate::schema::modified_buy_candlestick_agg::dsl::*;

    let retry_strategy = FixedInterval::from_millis(1).take(15);

    Retry::spawn(retry_strategy, || async {
        let mut connection = get_connection(pool.clone())
        .await
        .expect("Error connecting to database");
    modified_buy_candlestick_agg
        .load::<ModifiedBuyCandlestick>(&mut connection)
        .await
        .map_err(|e| {
            eprintln!("Error loading modified buy candlesticks: {}", e);
            e
        })
    }).await.expect("Error getting modified buy candlesticks")
}