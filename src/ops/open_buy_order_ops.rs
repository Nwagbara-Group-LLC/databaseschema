use crate::{get_connection, models::open_buy_order::{NewOpenBuyOrder, OpenBuyOrder}, CustomAsyncPgConnectionManager};
use bigdecimal::BigDecimal;
use deadpool::managed::Pool;
use diesel::prelude::*;
use diesel::QueryDsl;
use diesel_async::RunQueryDsl;
use std::sync::Arc;

pub async fn create_open_buy_order(pool: Arc<Pool<CustomAsyncPgConnectionManager>>, order: NewOpenBuyOrder) -> OpenBuyOrder {
    println!("Creating open buy order: {:?}", order);
    use crate::schema::open_buy_orders::dsl::*;

    let mut connection = get_connection(pool)
        .await
        .expect("Error connecting to database");

    diesel::insert_into(open_buy_orders)
        .values(&order)
        .returning(OpenBuyOrder::as_returning())
        .get_result(&mut connection)
        .await
        .expect("Error saving new open buy order")
}

pub async fn create_open_buy_orders(pool: Arc<Pool<CustomAsyncPgConnectionManager>>, orders: Vec<NewOpenBuyOrder>) -> Vec<OpenBuyOrder> {
    println!("Creating open buy orders: {:?}", orders);
    use crate::schema::open_buy_orders::dsl::*;

    let mut connection = get_connection(pool)
        .await
        .expect("Error connecting to database");

    diesel::insert_into(open_buy_orders)
        .values(&orders)
        .returning(OpenBuyOrder::as_returning())
        .get_results(&mut connection)
        .await
        .expect("Error saving new open buy order")
}

pub async fn delete_open_buy_order(pool: Arc<Pool<CustomAsyncPgConnectionManager>>, id: &str) {
    println!("Deleting open buy order");
    use crate::schema::open_buy_orders::dsl::*;

    let mut connection = get_connection(pool)
        .await
        .expect("Error connecting to database");
    diesel::delete(open_buy_orders.filter(unique_id.eq(id)))
        .execute(&mut connection)
        .await
        .expect("Error deleting open buy order");
}

pub async fn delete_open_buy_orders(pool: Arc<Pool<CustomAsyncPgConnectionManager>>, ids: Vec<&String>) {
    println!("Deleting open buy orders: {:?}", ids);
    use crate::schema::open_buy_orders::dsl::*;

    let mut connection = get_connection(pool)
        .await
        .expect("Error connecting to database");
    diesel::delete(open_buy_orders.filter(unique_id.eq_any(ids)))
        .execute(&mut connection)
        .await
        .expect("Error deleting open buy orders");
}

pub async fn get_open_buy_orders(pool: Arc<Pool<CustomAsyncPgConnectionManager>>) -> Vec<OpenBuyOrder> {
    println!("Getting open buy orders");
    use crate::schema::open_buy_orders::dsl::*;

    let mut connection = get_connection(pool)
        .await
        .expect("Error connecting to database");
    open_buy_orders
        .load::<OpenBuyOrder>(&mut connection)
        .await
        .expect("Error loading open buy orders")
}

pub async fn get_total_buy_quantity(pool: Arc<Pool<CustomAsyncPgConnectionManager>>) -> BigDecimal {
    println!("Getting total buy quantity");
    use crate::schema::open_buy_orders::dsl::*;

    let mut connection = get_connection(pool)
        .await
        .expect("Error connecting to database");
    open_buy_orders
        .select(diesel::dsl::sum(buy_quantity))
        .first::<Option<BigDecimal>>(&mut connection)
        .await
        .unwrap_or(Some(BigDecimal::from(0))) // Return 0 if no rows are found
        .unwrap_or(BigDecimal::from(0))
}
