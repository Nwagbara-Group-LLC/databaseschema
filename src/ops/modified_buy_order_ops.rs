use crate::{get_connection, models::modified_buy_order::{ModifiedBuyOrder, NewModifiedBuyOrder}, CustomAsyncPgConnectionManager};
use bigdecimal::BigDecimal;
use deadpool::managed::Pool;
use diesel::prelude::*;
use diesel::QueryDsl;
use diesel_async::RunQueryDsl;
use std::sync::Arc;

pub async fn create_modified_buy_order(pool: Arc<Pool<CustomAsyncPgConnectionManager>>, order: NewModifiedBuyOrder) -> ModifiedBuyOrder {
    println!("Creating modified buy order: {:?}", order);
    use crate::schema::modified_buy_orders::dsl::*;

    let mut connection = get_connection(pool)
        .await
        .expect("Error connecting to database");

    diesel::insert_into(modified_buy_orders)
        .values(&order)
        .returning(ModifiedBuyOrder::as_returning())
        .get_result(&mut connection)
        .await
        .expect("Error saving new modified buy order")
}

pub async fn create_modified_buy_orders(pool: Arc<Pool<CustomAsyncPgConnectionManager>>, orders: Vec<NewModifiedBuyOrder>) -> Vec<ModifiedBuyOrder> {
    println!("Creating modified buy orders: {:?}", orders);
    use crate::schema::modified_buy_orders::dsl::*;

    let mut connection = get_connection(pool)
        .await
        .expect("Error connecting to database");

    diesel::insert_into(modified_buy_orders)
        .values(&orders)
        .returning(ModifiedBuyOrder::as_returning())
        .get_results(&mut connection)
        .await
        .expect("Error saving new modified buy order")
}

pub async fn delete_modified_buy_order(pool: Arc<Pool<CustomAsyncPgConnectionManager>>, id: &str) {
    println!("Deleting modified buy order");
    use crate::schema::modified_buy_orders::dsl::*;

    let mut connection = get_connection(pool)
        .await
        .expect("Error connecting to database");
    diesel::delete(modified_buy_orders.filter(unique_id.eq(id)))
        .execute(&mut connection)
        .await
        .expect("Error deleting modified buy order");
}

pub async fn delete_modified_buy_orders(pool: Arc<Pool<CustomAsyncPgConnectionManager>>, ids: Vec<&String>) {
    println!("Deleting modified buy orders: {:?}", ids);
    use crate::schema::modified_buy_orders::dsl::*;

    let mut connection = get_connection(pool)
        .await
        .expect("Error connecting to database");
    diesel::delete(modified_buy_orders.filter(unique_id.eq_any(ids)))
        .execute(&mut connection)
        .await
        .expect("Error deleting modified buy orders");
}

pub async fn get_modified_buy_orders(pool: Arc<Pool<CustomAsyncPgConnectionManager>>) -> Vec<ModifiedBuyOrder> {
    println!("Getting modified buy orders");
    use crate::schema::modified_buy_orders::dsl::*;

    let mut connection = get_connection(pool)
        .await
        .expect("Error connecting to database");
    modified_buy_orders
        .load::<ModifiedBuyOrder>(&mut connection)
        .await
        .expect("Error loading modified buy orders")
}

pub async fn get_total_new_buy_quantity(pool: Arc<Pool<CustomAsyncPgConnectionManager>>) -> BigDecimal {
    println!("Getting total buy quantity remaining");
    use crate::schema::modified_buy_orders::dsl::*;

    let mut connection = get_connection(pool)
        .await
        .expect("Error connecting to database");
    modified_buy_orders
        .select(diesel::dsl::sum(new_buy_quantity))
        .first::<Option<BigDecimal>>(&mut connection)
        .await
        .unwrap_or(Some(BigDecimal::from(0))) // Return 0 if no rows are found
        .unwrap_or(BigDecimal::from(0))
}
