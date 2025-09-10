use std::sync::Arc;
use std::time::Instant;
use anyhow::Error;
use diesel::prelude::*;
use diesel::upsert::excluded;
use diesel_async::RunQueryDsl;
use tokio_retry::{strategy::ExponentialBackoff, Retry, strategy::jitter};
use uuid::Uuid;
use tracing::{info, error, debug};

use crate::models::backtest_result::{
    BacktestResult, NewBacktestResult, 
    BacktestReport, NewBacktestReport,
    BacktestReportAccessLog, NewBacktestReportAccessLog,
    BacktestTrade, NewBacktestTrade,
    BacktestEquityCurve, NewBacktestEquityCurve,
    BacktestPositionHistory, NewBacktestPositionHistory,
    BacktestDrawdownPeriod, NewBacktestDrawdownPeriod
};
use crate::get_timescale_connection;
use diesel_async::pooled_connection::deadpool;
use diesel_async::AsyncPgConnection;


/// Create a new backtest result
pub async fn create_backtest_result(
    pool: Arc<deadpool::Pool<AsyncPgConnection>>,
    new_result: NewBacktestResult,
) -> Result<BacktestResult, Error> {
    let start_time = Instant::now();
    info!("Creating backtest result for strategy: {}", new_result.strategy_name);
    use crate::schema::backtest_results::dsl::*;

    let retry_strategy = ExponentialBackoff::from_millis(10).map(jitter).take(3);

    Retry::spawn(retry_strategy, || async {
        let mut connection = get_timescale_connection(pool.clone())
            .await
            .map_err(|e| {
                error!("Failed to get database connection: {}", e);
                anyhow::Error::from(e)
            })?;

        let result = diesel::insert_into(backtest_results)
            .values(&new_result)
            .on_conflict(backtest_id)
            .do_update()
            .set((
                strategy_name.eq(excluded(strategy_name)),
                symbol.eq(excluded(symbol)),
                total_return.eq(excluded(total_return)),
                updated_at.eq(excluded(updated_at)),
            ))
            .get_result::<BacktestResult>(&mut connection)
            .await
            .map_err(|e| {
                error!("Error creating backtest result: {}", e);
                anyhow::Error::from(e)
            })?;
            
        debug!("Created backtest result in {}ms", start_time.elapsed().as_millis());
        Ok(result)
    }).await
}

/// Get backtest result by ID
pub async fn get_backtest_result(
    pool: Arc<deadpool::Pool<AsyncPgConnection>>,
    result_id: Uuid,
) -> Result<Option<BacktestResult>, Error> {
    let start_time = Instant::now();
    info!("Getting backtest result by ID: {}", result_id);
    use crate::schema::backtest_results::dsl::*;

    let retry_strategy = ExponentialBackoff::from_millis(10).map(jitter).take(3);

    Retry::spawn(retry_strategy, || async {
        let mut connection = get_timescale_connection(pool.clone())
            .await
            .map_err(|e| {
                error!("Failed to get database connection: {}", e);
                anyhow::Error::from(e)
            })?;

        let result = backtest_results
            .filter(id.eq(result_id))
            .select(BacktestResult::as_select())
            .first::<BacktestResult>(&mut connection)
            .await
            .optional()
            .map_err(|e| {
                error!("Error fetching backtest result: {}", e);
                anyhow::Error::from(e)
            })?;
            
        debug!("Fetched backtest result in {}ms", start_time.elapsed().as_millis());
        Ok(result)
    }).await
}

/// Get backtest result by backtest_id
pub async fn get_backtest_result_by_backtest_id(
    pool: Arc<deadpool::Pool<AsyncPgConnection>>,
    test_id: Uuid,
) -> Result<Option<BacktestResult>, Error> {
    let start_time = Instant::now();
    info!("Getting backtest result by backtest_id: {}", test_id);
    use crate::schema::backtest_results::dsl::*;

    let retry_strategy = ExponentialBackoff::from_millis(10).map(jitter).take(3);

    Retry::spawn(retry_strategy, || async {
        let mut connection = get_timescale_connection(pool.clone())
            .await
            .map_err(|e| {
                error!("Failed to get database connection: {}", e);
                anyhow::Error::from(e)
            })?;

        let result = backtest_results
            .filter(backtest_id.eq(test_id))
            .select(BacktestResult::as_select())
            .first::<BacktestResult>(&mut connection)
            .await
            .optional()
            .map_err(|e| {
                error!("Error fetching backtest result by backtest_id: {}", e);
                anyhow::Error::from(e)
            })?;
            
        debug!("Fetched backtest result by backtest_id in {}ms", start_time.elapsed().as_millis());
        Ok(result)
    }).await
}

/// Get all results for a strategy
pub async fn get_backtest_results_by_strategy(
    pool: Arc<deadpool::Pool<AsyncPgConnection>>,
    strategy: &str,
) -> Result<Vec<BacktestResult>, Error> {
    let start_time = Instant::now();
    info!("Getting backtest results for strategy: {}", strategy);
    
    // Input validation
    if strategy.is_empty() || strategy.len() > 100 {
        error!("Invalid strategy name length");
        return Err(anyhow::anyhow!("Strategy name must be 1-100 characters"));
    }
    
    use crate::schema::backtest_results::dsl::*;

    let retry_strategy = ExponentialBackoff::from_millis(10).map(jitter).take(3);

    Retry::spawn(retry_strategy, || async {
        let mut connection = get_timescale_connection(pool.clone())
            .await
            .map_err(|e| {
                error!("Failed to get database connection: {}", e);
                anyhow::Error::from(e)
            })?;

        let results = backtest_results
            .filter(strategy_name.eq(strategy))
            .order(created_at.desc())
            .limit(1000) // Prevent memory exhaustion
            .select(BacktestResult::as_select())
            .load(&mut connection)
            .await
            .map_err(|e| {
                error!("Error fetching backtest results by strategy: {}", e);
                anyhow::Error::from(e)
            })?;
            
        debug!("Fetched {} backtest results in {}ms", results.len(), start_time.elapsed().as_millis());
        Ok(results)
    }).await
}
