use crate::{get_timescale_connection, models::open_buy_candlestick::OpenBuyCandlestick, CustomAsyncPgConnectionManager};
use deadpool::managed::Pool;
use diesel::{result::Error, ExpressionMethods, QueryDsl};
use diesel_async::RunQueryDsl;
use tokio_retry::{strategy::{jitter, ExponentialBackoff}, Retry};
use std::sync::Arc;

pub async fn get_open_buy_candlesticks_by_symbol(pool: Arc<Pool<CustomAsyncPgConnectionManager>>, sym: &str) -> Result<Vec<OpenBuyCandlestick>, Error> {
    println!("Getting open buy candlesticks");
    use crate::schema::open_buy_candlestick_agg::dsl::*;

    let retry_strategy = ExponentialBackoff::from_millis(10).map(jitter).take(3);

    Retry::spawn(retry_strategy, || async {
        let mut connection = get_timescale_connection(pool.clone())
        .await
        .expect("Error connecting to database");
    open_buy_candlestick_agg
        .filter(symbol.eq(sym))
        .load::<OpenBuyCandlestick>(&mut connection)
        .await
        .map_err(|e| {
            eprintln!("Error loading open buy candlesticks: {}", e);
            e
        })
    }).await
}