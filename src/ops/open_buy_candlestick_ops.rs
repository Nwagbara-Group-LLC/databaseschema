use crate::{get_connection, models::OpenBuyCandlestick, CustomAsyncPgConnectionManager};
use deadpool::managed::Pool;
use diesel_async::RunQueryDsl;
use std::sync::Arc;

pub async fn get_open_buy_candlesticks(pool: Arc<Pool<CustomAsyncPgConnectionManager>>) -> Vec<OpenBuyCandlestick> {
    println!("Getting open buy candlesticks");
    use crate::schema::open_buy_candlestick_agg::dsl::*;

    let mut connection = get_connection(pool)
        .await
        .expect("Error connecting to database");
    open_buy_candlestick_agg
        .load::<OpenBuyCandlestick>(&mut connection)
        .await
        .expect("Error loading open buy candlesticks")
}