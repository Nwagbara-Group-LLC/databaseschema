use std::sync::Arc;
use std::time::Instant;
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use diesel_async::pooled_connection::deadpool;
use diesel_async::AsyncPgConnection;
use anyhow::Error;

use ultra_logger::UltraLogger;
use uuid::Uuid;

use crate::models::sim_open_sell_order::{SimOpenSellOrder, NewSimOpenSellOrder};
use crate::get_timescale_connection;

/// Create a new simulation open sell order
pub async fn create_sim_open_sell_order(
    pool: Arc<deadpool::Pool<AsyncPgConnection>>,
    order: NewSimOpenSellOrder,
) -> Result<SimOpenSellOrder, Error> {
    let start_time = Instant::now();
    let mut conn = get_timescale_connection(pool).await
        .map_err(|e| anyhow::Error::from(e))?;

    use crate::schema::sim_open_sell_orders;

    let result = diesel::insert_into(sim_open_sell_orders::table)
        .values(&order)
        .get_result(&mut conn)
        .await
        .map_err(|e| anyhow::Error::from(e))?;

    let duration = start_time.elapsed();
    let logger = UltraLogger::new("databaseschema".to_string());
    let _ = logger.info(format!("Created simulation sell order in {:?}", duration)).await;

    Ok(result)
}

/// Get simulation open sell orders by backtest ID
pub async fn get_sim_open_sell_orders_by_backtest(
    pool: Arc<deadpool::Pool<AsyncPgConnection>>,
    backtest_id_param: Uuid,
) -> Result<Vec<SimOpenSellOrder>, Error> {
    let start_time = Instant::now();
    let mut conn = get_timescale_connection(pool).await
        .map_err(|e| anyhow::Error::from(e))?;

    use crate::schema::sim_open_sell_orders::dsl::*;

    let result = sim_open_sell_orders
        .filter(backtest_id.eq(backtest_id_param))
        .load::<SimOpenSellOrder>(&mut conn)
        .await
        .map_err(|e| anyhow::Error::from(e))?;

    let duration = start_time.elapsed();
    let logger = UltraLogger::new("databaseschema".to_string());
    let _ = logger.info(format!("Retrieved {} simulation sell orders in {:?}", result.len(), duration)).await;

    Ok(result)
}

/// Delete simulation open sell order
pub async fn delete_sim_open_sell_order(
    pool: Arc<deadpool::Pool<AsyncPgConnection>>,
    unique_order_id: &str,
    backtest_id_param: Uuid,
) -> Result<usize, Error> {
    let start_time = Instant::now();
    let mut conn = get_timescale_connection(pool).await
        .map_err(|e| anyhow::Error::from(e))?;

    use crate::schema::sim_open_sell_orders::dsl::*;

    let result = diesel::delete(
        sim_open_sell_orders
            .filter(unique_id.eq(unique_order_id))
            .filter(backtest_id.eq(backtest_id_param))
    )
    .execute(&mut conn)
    .await
    .map_err(|e| anyhow::Error::from(e))?;

    let duration = start_time.elapsed();
    let logger = UltraLogger::new("databaseschema".to_string());
    let _ = logger.info(format!("Deleted {} simulation sell orders in {:?}", result, duration)).await;

    Ok(result)
}
