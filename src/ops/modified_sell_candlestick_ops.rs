use crate::{get_connection, models::modified_sell_candlestick::ModifiedSellCandlestick, CustomAsyncPgConnectionManager};
use deadpool::managed::Pool;
use diesel::{ExpressionMethods, QueryDsl};
use diesel_async::RunQueryDsl;
use tokio_retry::{strategy::FixedInterval, Retry};
use std::sync::Arc;

pub async fn get_modified_sell_candlesticks_by_symbol(pool: Arc<Pool<CustomAsyncPgConnectionManager>>, sym: &str) -> Vec<ModifiedSellCandlestick> {
    println!("Getting modified sell candlesticks");
    use crate::schema::modified_sell_candlestick_agg::dsl::*;

    let retry_strategy = FixedInterval::from_millis(1).take(15);

    Retry::spawn(retry_strategy, || async {
        let mut connection = get_connection(pool.clone())
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
    }).await.expect("Error getting modified sell candlesticks")
}