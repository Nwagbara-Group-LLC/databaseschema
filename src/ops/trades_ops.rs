use crate::{get_connection, models::{NewTrade, Trade}, CustomAsyncPgConnectionManager};
use deadpool::managed::Pool;
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use std::sync::Arc;

pub async fn create_trades(pool: Arc<Pool<CustomAsyncPgConnectionManager>>, orders: Vec<NewTrade>) -> Vec<Trade> {
    println!("Creating trades: {:?}", orders);
    use crate::schema::trades::dsl::*;

    let mut connection = get_connection(pool)
        .await
        .expect("Error connecting to database");

    diesel::insert_into(trades)
        .values(&orders)
        .returning(Trade::as_returning())
        .get_results(&mut connection)
        .await
        .expect("Error saving new trades")
}