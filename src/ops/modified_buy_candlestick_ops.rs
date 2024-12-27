use crate::{get_connection, models::modified_buy_candlestick::ModifiedBuyCandlestick, CustomAsyncPgConnectionManager};
use deadpool::managed::Pool;
use diesel_async::RunQueryDsl;
use std::sync::Arc;

pub async fn get_modified_buy_candlesticks(pool: Arc<Pool<CustomAsyncPgConnectionManager>>) -> Vec<ModifiedBuyCandlestick> {
    println!("Getting modified buy candlesticks");
    use crate::schema::modified_buy_candlestick_agg::dsl::*;

    let mut connection = get_connection(pool)
        .await
        .expect("Error connecting to database");
    modified_buy_candlestick_agg
        .load::<ModifiedBuyCandlestick>(&mut connection)
        .await
        .expect("Error loading modified buy candlesticks")
}