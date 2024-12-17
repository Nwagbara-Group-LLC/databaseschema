use crate::{get_connection, models::{Exchange, NewExchange}, CustomAsyncPgConnectionManager};
use deadpool::managed::Pool;
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use std::sync::Arc;

pub async fn create_exchange(pool: Arc<Pool<CustomAsyncPgConnectionManager>>, new_exchange: NewExchange) -> Exchange {
    println!("Creating exchange: {:?}", new_exchange);
    use crate::schema::exchanges::dsl::*;

    let mut connection = get_connection(pool).await.expect("Error connecting to database");

    diesel::insert_into(exchanges)
        .values(&new_exchange)
        .returning(Exchange::as_returning())
        .get_result(&mut connection)
        .await
        .expect("Error saving new exchange")
}

pub async fn get_exchanges(pool: Arc<Pool<CustomAsyncPgConnectionManager>>) -> Vec<Exchange> {
    println!("Getting exchanges");
    use crate::schema::exchanges::dsl::*;

    let mut connection = get_connection(pool).await.expect("Error connecting to database");
    exchanges
        .load::<Exchange>(&mut connection)
        .await
        .expect("Error loading exchanges")
}

pub async fn get_exchanges_by_id(pool: Arc<Pool<CustomAsyncPgConnectionManager>>, get_exchange: Exchange) -> Exchange {
    println!("Getting exchange");
    use crate::schema::exchanges::dsl::*;

    let mut connection = get_connection(pool).await.expect("Error connecting to database");
    exchanges
        .find(get_exchange.exchange_id)
        .first::<Exchange>(&mut connection)
        .await
        .expect("Error loading exchange")
}

pub async fn get_exchanges_by_name(pool: Arc<Pool<CustomAsyncPgConnectionManager>>, name: &String) -> Exchange {
    println!("Getting exchange");
    use crate::schema::exchanges::dsl::*;

    let mut connection = get_connection(pool).await.expect("Error connecting to database");
    exchanges
        .filter(exchange.eq(name))
        .first::<Exchange>(&mut connection)
        .await
        .expect("Error loading exchange")
}

pub async fn exchange_exists(pool: Arc<Pool<CustomAsyncPgConnectionManager>>, name: &String) -> bool {
    println!("Checking if exchange exists");
    use crate::schema::exchanges::dsl::*;

    let mut connection = get_connection(pool).await.expect("Error connecting to database");
    exchanges
        .filter(exchange.eq(name))
        .first::<Exchange>(&mut connection)
        .await
        .is_ok()
}