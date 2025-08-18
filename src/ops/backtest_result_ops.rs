use std::sync::Arc;
use anyhow::Error;
use diesel::prelude::*;
use diesel::upsert::excluded;
use diesel_async::RunQueryDsl;
use tokio_retry::{strategy::ExponentialBackoff, Retry, strategy::jitter};
use uuid::Uuid;

use crate::models::backtest_result::{BacktestResult, NewBacktestResult};
use crate::{CustomAsyncPgConnectionManager, Pool, get_timescale_connection};

/// Create a new backtest result
pub async fn create_backtest_result(
    pool: Arc<Pool<CustomAsyncPgConnectionManager>>,
    new_result: NewBacktestResult,
) -> Result<BacktestResult, Error> {
    println!("Creating backtest result for strategy: {}", new_result.strategy_name);
    use crate::schema::backtest_results::dsl::*;

    let retry_strategy = ExponentialBackoff::from_millis(10).map(jitter).take(3);

    Retry::spawn(retry_strategy, || async {
        let mut connection = get_timescale_connection(pool.clone())
            .await
            .expect("Error connecting to database");

        diesel::insert_into(backtest_results)
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
                eprintln!("Error creating backtest result: {}", e);
                anyhow::Error::from(e)
            })
    }).await
}

/// Get backtest result by ID
pub async fn get_backtest_result(
    pool: Arc<Pool<CustomAsyncPgConnectionManager>>,
    result_id: Uuid,
) -> Result<Option<BacktestResult>, Error> {
    use crate::schema::backtest_results::dsl::*;

    let retry_strategy = ExponentialBackoff::from_millis(10).map(jitter).take(3);

    Retry::spawn(retry_strategy, || async {
        let mut connection = get_timescale_connection(pool.clone())
            .await
            .expect("Error connecting to database");

        backtest_results
            .filter(id.eq(result_id))
            .first::<BacktestResult>(&mut connection)
            .await
            .optional()
            .map_err(|e| {
                eprintln!("Error fetching backtest result: {}", e);
                anyhow::Error::from(e)
            })
    }).await
}

/// Get backtest result by backtest_id
pub async fn get_backtest_result_by_backtest_id(
    pool: Arc<Pool<CustomAsyncPgConnectionManager>>,
    test_id: Uuid,
) -> Result<Option<BacktestResult>, Error> {
    use crate::schema::backtest_results::dsl::*;

    let retry_strategy = ExponentialBackoff::from_millis(10).map(jitter).take(3);

    Retry::spawn(retry_strategy, || async {
        let mut connection = get_timescale_connection(pool.clone())
            .await
            .expect("Error connecting to database");

        backtest_results
            .filter(backtest_id.eq(test_id))
            .first::<BacktestResult>(&mut connection)
            .await
            .optional()
            .map_err(|e| {
                eprintln!("Error fetching backtest result by backtest_id: {}", e);
                anyhow::Error::from(e)
            })
    }).await
}

/// Get all results for a strategy
pub async fn get_backtest_results_by_strategy(
    pool: Arc<Pool<CustomAsyncPgConnectionManager>>,
    strategy: &str,
) -> Result<Vec<BacktestResult>, Error> {
    use crate::schema::backtest_results::dsl::*;

    let retry_strategy = ExponentialBackoff::from_millis(10).map(jitter).take(3);

    Retry::spawn(retry_strategy, || async {
        let mut connection = get_timescale_connection(pool.clone())
            .await
            .expect("Error connecting to database");

        backtest_results
            .filter(strategy_name.eq(strategy))
            .order(created_at.desc())
            .load(&mut connection)
            .await
            .map_err(|e| {
                eprintln!("Error fetching backtest results by strategy: {}", e);
                anyhow::Error::from(e)
            })
    }).await
}
