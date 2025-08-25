use crate::{
    get_timescale_connection, 
    models::candles::{Candle1m, Candle5m, Candle15m, Candle1h, Candle1d}, 
    CustomAsyncPgConnectionManager
};
use deadpool::managed::Pool;
use diesel::{prelude::*, result::Error};
use diesel_async::RunQueryDsl;
use tokio_retry::{strategy::{jitter, ExponentialBackoff}, Retry};
use std::sync::Arc;
use chrono::{DateTime, Utc};

/// Get 1-minute candles for a symbol and exchange
pub async fn get_candles_1m(
    pool: Arc<Pool<CustomAsyncPgConnectionManager>>, 
    sym: &str, 
    xchange: &str,
    start_time: Option<DateTime<Utc>>,
    end_time: Option<DateTime<Utc>>,
    limit: Option<usize>
) -> Result<Vec<Candle1m>, Error> {
    use crate::schema::candles_1m::dsl::*;

    let retry_strategy = ExponentialBackoff::from_millis(10).map(jitter).take(3);

    Retry::spawn(retry_strategy, || async {
        let mut connection = get_timescale_connection(pool.clone())
            .await
            .expect("Error connecting to database");

        let mut query = candles_1m
            .filter(symbol.eq(sym).and(exchange.eq(xchange)))
            .order(timestamp.asc())
            .into_boxed();

        if let Some(start) = start_time {
            query = query.filter(timestamp.ge(start));
        }
        if let Some(end) = end_time {
            query = query.filter(timestamp.le(end));
        }
        if let Some(lim) = limit {
            query = query.limit(lim as i64);
        }

        query
            .select(Candle1m::as_select())
            .load::<Candle1m>(&mut connection)
            .await
            .map_err(|e| {
                eprintln!("Error loading 1m candles: {}", e);
                e
            })
    }).await
}

/// Get 5-minute candles for a symbol and exchange
pub async fn get_candles_5m(
    pool: Arc<Pool<CustomAsyncPgConnectionManager>>, 
    sym: &str, 
    xchange: &str,
    start_time: Option<DateTime<Utc>>,
    end_time: Option<DateTime<Utc>>,
    limit: Option<usize>
) -> Result<Vec<Candle5m>, Error> {
    use crate::schema::candles_5m::dsl::*;

    let retry_strategy = ExponentialBackoff::from_millis(10).map(jitter).take(3);

    Retry::spawn(retry_strategy, || async {
        let mut connection = get_timescale_connection(pool.clone())
            .await
            .expect("Error connecting to database");

        let mut query = candles_5m
            .filter(symbol.eq(sym).and(exchange.eq(xchange)))
            .order(timestamp.asc())
            .into_boxed();

        if let Some(start) = start_time {
            query = query.filter(timestamp.ge(start));
        }
        if let Some(end) = end_time {
            query = query.filter(timestamp.le(end));
        }
        if let Some(lim) = limit {
            query = query.limit(lim as i64);
        }

        query
            .select(Candle5m::as_select())
            .load::<Candle5m>(&mut connection)
            .await
            .map_err(|e| {
                eprintln!("Error loading 5m candles: {}", e);
                e
            })
    }).await
}

/// Get 15-minute candles for a symbol and exchange
pub async fn get_candles_15m(
    pool: Arc<Pool<CustomAsyncPgConnectionManager>>, 
    sym: &str, 
    xchange: &str,
    start_time: Option<DateTime<Utc>>,
    end_time: Option<DateTime<Utc>>,
    limit: Option<usize>
) -> Result<Vec<Candle15m>, Error> {
    use crate::schema::candles_15m::dsl::*;

    let retry_strategy = ExponentialBackoff::from_millis(10).map(jitter).take(3);

    Retry::spawn(retry_strategy, || async {
        let mut connection = get_timescale_connection(pool.clone())
            .await
            .expect("Error connecting to database");

        let mut query = candles_15m
            .filter(symbol.eq(sym).and(exchange.eq(xchange)))
            .order(timestamp.asc())
            .into_boxed();

        if let Some(start) = start_time {
            query = query.filter(timestamp.ge(start));
        }
        if let Some(end) = end_time {
            query = query.filter(timestamp.le(end));
        }
        if let Some(lim) = limit {
            query = query.limit(lim as i64);
        }

        query
            .select(Candle15m::as_select())
            .load::<Candle15m>(&mut connection)
            .await
            .map_err(|e| {
                eprintln!("Error loading 15m candles: {}", e);
                e
            })
    }).await
}

/// Get 1-hour candles for a symbol and exchange
pub async fn get_candles_1h(
    pool: Arc<Pool<CustomAsyncPgConnectionManager>>, 
    sym: &str, 
    xchange: &str,
    start_time: Option<DateTime<Utc>>,
    end_time: Option<DateTime<Utc>>,
    limit: Option<usize>
) -> Result<Vec<Candle1h>, Error> {
    use crate::schema::candles_1h::dsl::*;

    let retry_strategy = ExponentialBackoff::from_millis(10).map(jitter).take(3);

    Retry::spawn(retry_strategy, || async {
        let mut connection = get_timescale_connection(pool.clone())
            .await
            .expect("Error connecting to database");

        let mut query = candles_1h
            .filter(symbol.eq(sym).and(exchange.eq(xchange)))
            .order(timestamp.asc())
            .into_boxed();

        if let Some(start) = start_time {
            query = query.filter(timestamp.ge(start));
        }
        if let Some(end) = end_time {
            query = query.filter(timestamp.le(end));
        }
        if let Some(lim) = limit {
            query = query.limit(lim as i64);
        }

        query
            .select(Candle1h::as_select())
            .load::<Candle1h>(&mut connection)
            .await
            .map_err(|e| {
                eprintln!("Error loading 1h candles: {}", e);
                e
            })
    }).await
}

/// Get 1-day candles for a symbol and exchange
pub async fn get_candles_1d(
    pool: Arc<Pool<CustomAsyncPgConnectionManager>>, 
    sym: &str, 
    xchange: &str,
    start_time: Option<DateTime<Utc>>,
    end_time: Option<DateTime<Utc>>,
    limit: Option<usize>
) -> Result<Vec<Candle1d>, Error> {
    use crate::schema::candles_1d::dsl::*;

    let retry_strategy = ExponentialBackoff::from_millis(10).map(jitter).take(3);

    Retry::spawn(retry_strategy, || async {
        let mut connection = get_timescale_connection(pool.clone())
            .await
            .expect("Error connecting to database");

        let mut query = candles_1d
            .filter(symbol.eq(sym).and(exchange.eq(xchange)))
            .order(timestamp.asc())
            .into_boxed();

        if let Some(start) = start_time {
            query = query.filter(timestamp.ge(start));
        }
        if let Some(end) = end_time {
            query = query.filter(timestamp.le(end));
        }
        if let Some(lim) = limit {
            query = query.limit(lim as i64);
        }

        query
            .select(Candle1d::as_select())
            .load::<Candle1d>(&mut connection)
            .await
            .map_err(|e| {
                eprintln!("Error loading 1d candles: {}", e);
                e
            })
    }).await
}
