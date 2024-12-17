use crate::{get_connection, models::OpenSellCandlestick, CustomAsyncPgConnectionManager};
use deadpool::managed::Pool;
use diesel_async::RunQueryDsl;
use std::sync::Arc;

pub async fn get_open_sell_candlesticks(pool: Arc<Pool<CustomAsyncPgConnectionManager>>) -> Vec<OpenSellCandlestick> {
    println!("Getting open sell candlesticks");
    use crate::schema::open_sell_candlestick_agg::dsl::*;

    let mut connection = get_connection(pool)
        .await
        .expect("Error connecting to database");
    open_sell_candlestick_agg
        .load::<OpenSellCandlestick>(&mut connection)
        .await
        .expect("Error loading open sell candlesticks")
}