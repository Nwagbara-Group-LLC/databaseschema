use crate::{
    get_connection,
    models::{ModifiedSellOrder, NewModifiedSellOrder},
    CustomAsyncPgConnectionManager,
};
use bigdecimal::BigDecimal;
use deadpool::managed::Pool;
use diesel::prelude::*;
use diesel::QueryDsl;
use diesel_async::RunQueryDsl;
use std::sync::Arc;

pub async fn create_modified_sell_order(
    pool: Arc<Pool<CustomAsyncPgConnectionManager>>,
    order: NewModifiedSellOrder,
) -> ModifiedSellOrder {
    println!("Creating modified sell order: {:?}", order);
    use crate::schema::modified_sell_orders::dsl::*;

    let mut connection = get_connection(pool)
        .await
        .expect("Error connecting to database");

    diesel::insert_into(modified_sell_orders)
        .values(&order)
        .returning(ModifiedSellOrder::as_returning())
        .get_result(&mut connection)
        .await
        .expect("Error saving new modified sell order")
}

pub async fn create_modified_sell_orders(
    pool: Arc<Pool<CustomAsyncPgConnectionManager>>,
    orders: Vec<NewModifiedSellOrder>,
) -> Vec<ModifiedSellOrder> {
    println!("Creating modified sell orders: {:?}", orders);
    use crate::schema::modified_sell_orders::dsl::*;

    let mut connection = get_connection(pool)
        .await
        .expect("Error connecting to database");

    diesel::insert_into(modified_sell_orders)
        .values(&orders)
        .returning(ModifiedSellOrder::as_returning())
        .get_results(&mut connection)
        .await
        .expect("Error saving new modified sell order")
}

pub async fn delete_modified_sell_order(pool: Arc<Pool<CustomAsyncPgConnectionManager>>, id: &str) {
    println!("Deleting modified sell order");
    use crate::schema::modified_sell_orders::dsl::*;

    let mut connection = get_connection(pool)
        .await
        .expect("Error connecting to database");
    diesel::delete(modified_sell_orders.filter(unique_id.eq(id)))
        .execute(&mut connection)
        .await
        .expect("Error deleting modified sell order");
}

pub async fn delete_modified_sell_orders(pool: Arc<Pool<CustomAsyncPgConnectionManager>>, ids: Vec<&String>) {
    println!("Deleting modified sell orders: {:?}", ids);
    use crate::schema::modified_sell_orders::dsl::*;

    let mut connection = get_connection(pool)
        .await
        .expect("Error connecting to database");
    diesel::delete(modified_sell_orders.filter(unique_id.eq_any(ids)))
        .execute(&mut connection)
        .await
        .expect("Error deleting modified sell orders");
}

pub async fn get_modified_sell_orders(pool: Arc<Pool<CustomAsyncPgConnectionManager>>) -> Vec<ModifiedSellOrder> {
    println!("Getting modified sell orders");
    use crate::schema::modified_sell_orders::dsl::*;

    let mut connection = get_connection(pool)
        .await
        .expect("Error connecting to database");
    modified_sell_orders
        .load::<ModifiedSellOrder>(&mut connection)
        .await
        .expect("Error loading modified sell orders")
}

pub async fn get_total_new_sell_quantity(pool: Arc<Pool<CustomAsyncPgConnectionManager>>) -> BigDecimal {
    println!("Getting total sell quantity remaining");
    use crate::schema::modified_sell_orders::dsl::*;

    let mut connection = get_connection(pool)
        .await
        .expect("Error connecting to database");
    modified_sell_orders
        .select(diesel::dsl::sum(new_sell_quantity))
        .first::<Option<BigDecimal>>(&mut connection)
        .await
        .unwrap_or(Some(BigDecimal::from(0))) // Return 0 if no rows are found
        .unwrap_or(BigDecimal::from(0)) 
}