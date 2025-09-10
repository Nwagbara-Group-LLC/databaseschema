use std::sync::Arc;
use std::time::Instant;
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use diesel_async::pooled_connection::deadpool;
use diesel_async::AsyncPgConnection;
use anyhow::Error;

use tracing::{info, error};
use uuid::Uuid;

use crate::models::sim_trade::{SimTrade, NewSimTrade};
use crate::get_timescale_connection;

/// Create a new simulation trade
pub async fn create_sim_trade(
    pool: Arc<deadpool::Pool<AsyncPgConnection>>,
    trade: NewSimTrade,
) -> Result<SimTrade, Error> {
    let start_time = Instant::now();
    let mut conn = get_timescale_connection(pool).await
        .map_err(|e| anyhow::Error::from(e))?;

    use crate::schema::sim_trades;

    let result = diesel::insert_into(sim_trades::table)
        .values(&trade)
        .get_result(&mut conn)
        .await
        .map_err(|e| anyhow::Error::from(e))?;

    let duration = start_time.elapsed();
    info!("Created simulation trade in {:?}", duration);

    Ok(result)
}

/// Create multiple simulation trades
pub async fn create_sim_trades(
    pool: Arc<deadpool::Pool<AsyncPgConnection>>,
    trades: Vec<NewSimTrade>,
) -> Result<Vec<SimTrade>, Error> {
    let start_time = Instant::now();
    let mut conn = get_timescale_connection(pool).await
        .map_err(|e| anyhow::Error::from(e))?;

    use crate::schema::sim_trades;

    let result = diesel::insert_into(sim_trades::table)
        .values(&trades)
        .get_results(&mut conn)
        .await
        .map_err(|e| anyhow::Error::from(e))?;

    let duration = start_time.elapsed();
    info!("Created {} simulation trades in {:?}", result.len(), duration);

    Ok(result)
}

/// Get simulation trades by backtest ID
pub async fn get_sim_trades_by_backtest(
    pool: Arc<deadpool::Pool<AsyncPgConnection>>,
    backtest_id_param: Uuid,
) -> Result<Vec<SimTrade>, Error> {
    let start_time = Instant::now();
    let mut conn = get_timescale_connection(pool).await
        .map_err(|e| anyhow::Error::from(e))?;

    use crate::schema::sim_trades::dsl::*;

    let result = sim_trades
        .filter(backtest_id.eq(backtest_id_param))
        .order(created_at.asc())
        .load::<SimTrade>(&mut conn)
        .await
        .map_err(|e| anyhow::Error::from(e))?;

    let duration = start_time.elapsed();
    info!("Retrieved {} simulation trades in {:?}", result.len(), duration);

    Ok(result)
}

/// Get simulation trades by symbol and backtest ID
pub async fn get_sim_trades_by_symbol_and_backtest(
    pool: Arc<deadpool::Pool<AsyncPgConnection>>,
    symbol_param: &str,
    backtest_id_param: Uuid,
) -> Result<Vec<SimTrade>, Error> {
    let start_time = Instant::now();
    let mut conn = get_timescale_connection(pool).await
        .map_err(|e| anyhow::Error::from(e))?;

    use crate::schema::sim_trades::dsl::*;

    let result = sim_trades
        .filter(symbol.eq(symbol_param))
        .filter(backtest_id.eq(backtest_id_param))
        .order(created_at.asc())
        .load::<SimTrade>(&mut conn)
        .await
        .map_err(|e| anyhow::Error::from(e))?;

    let duration = start_time.elapsed();
    info!("Retrieved {} simulation trades for {} in {:?}", result.len(), symbol_param, duration);

    Ok(result)
}
