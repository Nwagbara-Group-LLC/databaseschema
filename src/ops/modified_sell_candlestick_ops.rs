use crate::{get_timescale_connection, models::modified_sell_candlestick::ModifiedSellCandlestick, CustomAsyncPgConnectionManager};
use deadpool::managed::Pool;
use diesel::{result::Error, ExpressionMethods, QueryDsl};
use diesel_async::RunQueryDsl;
use tokio_retry::{strategy::{jitter, ExponentialBackoff}, Retry};
use std::sync::Arc;

pub async fn get_modified_sell_candlesticks_by_symbol(pool: Arc<Pool<CustomAsyncPgConnectionManager>>, sym: &str) -> Result<Vec<ModifiedSellCandlestick>, Error> {
    println!("Getting modified sell candlesticks");
    use crate::schema::modified_sell_candlestick_agg::dsl::*;

    let retry_strategy = ExponentialBackoff::from_millis(10).map(jitter).take(3);

    Retry::spawn(retry_strategy, || async {
        let mut connection = get_timescale_connection(pool.clone())
        .await
        .expect("Error connecting to database");
    modified_sell_candlestick_agg
        .filter(symbol.eq(sym))
        .load::<ModifiedSellCandlestick>(&mut connection)
        .await
        .map_err(|e| {
            eprintln!("Error loading modified sell candlesticks: {}", e);
            e
        })
    }).await
}