use std::sync::Arc;
use std::time::Instant;
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use diesel_async::pooled_connection::deadpool;
use diesel_async::AsyncPgConnection;
use anyhow::Error;

use tracing::{info, error};
use uuid::Uuid;

use crate::models::sim_open_buy_order::{SimOpenBuyOrder, NewSimOpenBuyOrder};
use crate::get_timescale_connection;

/// Create a new simulation open buy order
pub async fn create_sim_open_buy_order(
    pool: Arc<deadpool::Pool<AsyncPgConnection>>,
    order: NewSimOpenBuyOrder,
) -> Result<SimOpenBuyOrder, Error> {
    let start_time = Instant::now();
    let mut conn = get_timescale_connection(pool).await
        .map_err(|e| anyhow::Error::from(e))?;

    use crate::schema::sim_open_buy_orders;

    let result = diesel::insert_into(sim_open_buy_orders::table)
        .values(&order)
        .get_result(&mut conn)
        .await
        .map_err(|e| anyhow::Error::from(e))?;

    let duration = start_time.elapsed();
    info!("Created simulation buy order in {:?}", duration);

    Ok(result)
}

/// Get simulation open buy orders by backtest ID
pub async fn get_sim_open_buy_orders_by_backtest(
    pool: Arc<deadpool::Pool<AsyncPgConnection>>,
    backtest_id_param: Uuid,
) -> Result<Vec<SimOpenBuyOrder>, Error> {
    let start_time = Instant::now();
    let mut conn = get_timescale_connection(pool).await
        .map_err(|e| anyhow::Error::from(e))?;

    use crate::schema::sim_open_buy_orders::dsl::*;

    let result = sim_open_buy_orders
        .filter(backtest_id.eq(backtest_id_param))
        .load::<SimOpenBuyOrder>(&mut conn)
        .await
        .map_err(|e| anyhow::Error::from(e))?;

    let duration = start_time.elapsed();
    info!("Retrieved {} simulation buy orders in {:?}", result.len(), duration);

    Ok(result)
}

/// Delete simulation open buy order
pub async fn delete_sim_open_buy_order(
    pool: Arc<deadpool::Pool<AsyncPgConnection>>,
    unique_order_id: &str,
    backtest_id_param: Uuid,
) -> Result<usize, Error> {
    let start_time = Instant::now();
    let mut conn = get_timescale_connection(pool).await
        .map_err(|e| anyhow::Error::from(e))?;

    use crate::schema::sim_open_buy_orders::dsl::*;

    let result = diesel::delete(
        sim_open_buy_orders
            .filter(unique_id.eq(unique_order_id))
            .filter(backtest_id.eq(backtest_id_param))
    )
    .execute(&mut conn)
    .await
    .map_err(|e| anyhow::Error::from(e))?;

    let duration = start_time.elapsed();
    info!("Deleted {} simulation buy orders in {:?}", result, duration);

    Ok(result)
}
