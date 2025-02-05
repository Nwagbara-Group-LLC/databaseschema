use crate::{get_timescale_connection, models::modified_buy_candlestick::ModifiedBuyCandlestick, CustomAsyncPgConnectionManager};
use deadpool::managed::Pool;
use diesel::{result::Error, ExpressionMethods, QueryDsl};
use diesel_async::RunQueryDsl;
use tokio_retry::{strategy::{jitter, ExponentialBackoff}, Retry};
use std::sync::Arc;

pub async fn get_modified_buy_candlesticks_by_symbol(pool: Arc<Pool<CustomAsyncPgConnectionManager>>, sym: &str) -> Result<Vec<ModifiedBuyCandlestick>, Error> {
    println!("Getting modified buy candlesticks");
    use crate::schema::modified_buy_candlestick_agg::dsl::*;

    let retry_strategy = ExponentialBackoff::from_millis(10).map(jitter).take(3);

    Retry::spawn(retry_strategy, || async {
        let mut connection = get_timescale_connection(pool.clone())
        .await
        .expect("Error connecting to database");
    modified_buy_candlestick_agg
        .filter(symbol.eq(sym))
        .load::<ModifiedBuyCandlestick>(&mut connection)
        .await
        .map_err(|e| {
            eprintln!("Error loading modified buy candlesticks: {}", e);
            e
        })
    }).await
}