use crate::{get_connection, models::ModifiedSellCandlestick, CustomAsyncPgConnectionManager};
use deadpool::managed::Pool;
use diesel_async::RunQueryDsl;
use std::sync::Arc;

pub async fn get_modified_sell_candlesticks(pool: Arc<Pool<CustomAsyncPgConnectionManager>>) -> Vec<ModifiedSellCandlestick> {
    println!("Getting modified sell candlesticks");
    use crate::schema::modified_sell_candlestick_agg::dsl::*;

    let mut connection = get_connection(pool)
        .await
        .expect("Error connecting to database");
    modified_sell_candlestick_agg
        .load::<ModifiedSellCandlestick>(&mut connection)
        .await
        .expect("Error loading modified sell candlesticks")
}