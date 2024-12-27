use crate::{get_connection, models::open_sell_order::{NewOpenSellOrder, OpenSellOrder}, CustomAsyncPgConnectionManager};
use bigdecimal::BigDecimal;
use deadpool::managed::Pool;
use diesel::prelude::*;
use diesel::QueryDsl;
use diesel_async::RunQueryDsl;
use std::sync::Arc;

pub async fn create_open_sell_order(pool: Arc<Pool<CustomAsyncPgConnectionManager>>, order: NewOpenSellOrder) -> OpenSellOrder {
    println!("Creating open sell order: {:?}", order);
    use crate::schema::open_sell_orders::dsl::*;

    let mut connection = get_connection(pool)
        .await
        .expect("Error connecting to database");

    diesel::insert_into(open_sell_orders)
        .values(&order)
        .returning(OpenSellOrder::as_returning())
        .get_result(&mut connection)
        .await
        .expect("Error saving new open sell order")
}

pub async fn create_open_sell_orders(pool: Arc<Pool<CustomAsyncPgConnectionManager>>, orders: Vec<NewOpenSellOrder>) -> Vec<OpenSellOrder> {
    println!("Creating open sell order: {:?}", orders);
    use crate::schema::open_sell_orders::dsl::*;

    let mut connection = get_connection(pool)
        .await
        .expect("Error connecting to database");

    diesel::insert_into(open_sell_orders)
        .values(&orders)
        .returning(OpenSellOrder::as_returning())
        .get_results(&mut connection)
        .await
        .expect("Error saving new open sell order")
}

pub async fn delete_open_sell_order(pool: Arc<Pool<CustomAsyncPgConnectionManager>>, id: &str) {
    println!("Deleting open sell order");
    use crate::schema::open_sell_orders::dsl::*;

    let mut connection = get_connection(pool)
        .await
        .expect("Error connecting to database");
    diesel::delete(open_sell_orders.filter(unique_id.eq(id)))
        .execute(&mut connection)
        .await
        .expect("Error deleting open sell order");
}

pub async fn delete_open_sell_orders(pool: Arc<Pool<CustomAsyncPgConnectionManager>>, ids: Vec<&String>) {
    println!("Deleting open sell orders: {:?}", ids);
    use crate::schema::open_sell_orders::dsl::*;

    let mut connection = get_connection(pool)
        .await
        .expect("Error connecting to database");
    diesel::delete(open_sell_orders.filter(unique_id.eq_any(ids)))
        .execute(&mut connection)
        .await
        .expect("Error deleting open sell orders");
}

pub async fn get_open_sell_orders(pool: Arc<Pool<CustomAsyncPgConnectionManager>>) -> Vec<OpenSellOrder> {
    println!("Getting open sell orders");
    use crate::schema::open_sell_orders::dsl::*;

    let mut connection = get_connection(pool)
        .await
        .expect("Error connecting to database");
    open_sell_orders
        .load::<OpenSellOrder>(&mut connection)
        .await
        .expect("Error loading open sell orders")
}

pub async fn get_total_sell_quantity(pool: Arc<Pool<CustomAsyncPgConnectionManager>>) -> BigDecimal {
    println!("Getting total sell quantity");
    use crate::schema::open_sell_orders::dsl::*;

    let mut connection = get_connection(pool)
        .await
        .expect("Error connecting to database");
    open_sell_orders
        .select(diesel::dsl::sum(sell_quantity))
        .first::<Option<BigDecimal>>(&mut connection)
        .await
        .unwrap_or(Some(BigDecimal::from(0))) // Return 0 if no rows are found
        .unwrap_or(BigDecimal::from(0))
}
