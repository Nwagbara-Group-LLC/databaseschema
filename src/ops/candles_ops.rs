use crate::{
    get_timescale_connection, 
    models::candles::Candle
};
use diesel_async::pooled_connection::deadpool;
use diesel_async::AsyncPgConnection;

use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use anyhow::Error;
use tokio_retry::{strategy::{jitter, ExponentialBackoff}, Retry};
use std::sync::Arc;
use chrono::{DateTime, Utc, Duration};
use tracing::{info, error, warn};
use std::time::Instant;

/// Get 1-minute candles for a symbol and exchange
pub async fn get_candles_1m(
    pool: Arc<deadpool::Pool<AsyncPgConnection>>, 
    sym: &str, 
    xchange: &str,
    start_time: Option<DateTime<Utc>>,
    end_time: Option<DateTime<Utc>>,
    limit: Option<usize>
) -> Result<Vec<Candle>, Error> {
    let query_start = Instant::now();
    use crate::schema::candles::dsl::*;

    // Security: Input validation
    if sym.is_empty() || sym.len() > 20 {
        error!("Invalid symbol length: {}", sym.len());
        return Err(anyhow::anyhow!("Invalid symbol length: {}", sym.len()));
    }
    
    if xchange.is_empty() || xchange.len() > 50 {
        error!("Invalid exchange length: {}", xchange.len());
        return Err(anyhow::anyhow!("Invalid exchange length: {}", xchange.len()));
    }

    // Security: Validate date range to prevent excessive queries
    if let (Some(start), Some(end)) = (start_time, end_time) {
        let duration = end - start;
        if duration > Duration::days(365) {
            error!("Date range too large: {} days", duration.num_days());
            return Err(anyhow::anyhow!("Date range too large: {} days", duration.num_days()));
        }
    }

    // Security: Validate limit to prevent memory exhaustion
    const MAX_LIMIT: usize = 100000;
    let safe_limit = match limit {
        Some(l) if l > MAX_LIMIT => {
            warn!("Limit {} exceeds maximum {}, using maximum", l, MAX_LIMIT);
            MAX_LIMIT
        },
        Some(l) => l,
        None => 10000, // Default reasonable limit
    };

    let retry_strategy = ExponentialBackoff::from_millis(10).map(jitter).take(3);

    Retry::spawn(retry_strategy, || async {
        let mut connection = get_timescale_connection(pool.clone())
            .await
            .map_err(|e| {
                error!("Failed to get database connection: {}", e);
                anyhow::Error::from(e)
            })?;

        let mut query = candles
            .filter(symbol.eq(sym).and(exchange.eq(xchange)))
            .filter(timeframe.eq("1m"))
            .order(timestamp.asc())
            .limit(safe_limit as i64)
            .into_boxed();

        if let Some(start) = start_time {
            query = query.filter(timestamp.ge(start));
        }
        if let Some(end) = end_time {
            query = query.filter(timestamp.le(end));
        }

        let result = query
            .select(Candle::as_select())
            .load::<Candle>(&mut connection)
            .await
            .map_err(|e| anyhow::Error::from(e))?;
            
        info!("Fetched {} 1m candles in {}ms", result.len(), query_start.elapsed().as_millis());
        Ok(result)
    }).await
}

/// Get 5-minute candles for a symbol and exchange
pub async fn get_candles_5m(
    pool: Arc<deadpool::Pool<AsyncPgConnection>>, 
    sym: &str, 
    xchange: &str,
    start_time: Option<DateTime<Utc>>,
    end_time: Option<DateTime<Utc>>,
    limit: Option<usize>
) -> Result<Vec<Candle>, Error> {
    let query_start = Instant::now();
    use crate::schema::candles::dsl::*;

    // Security: Input validation
    if sym.is_empty() || sym.len() > 20 {
        error!("Invalid symbol length: {}", sym.len());
        return Err(anyhow::anyhow!("Invalid symbol length: {}", sym.len()));
    }
    
    if xchange.is_empty() || xchange.len() > 50 {
        error!("Invalid exchange length: {}", xchange.len());
        return Err(anyhow::anyhow!("Invalid exchange length: {}", xchange.len()));
    }

    // Security: Validate date range to prevent excessive queries
    if let (Some(start), Some(end)) = (start_time, end_time) {
        let duration = end - start;
        if duration > Duration::days(365) {
            error!("Date range too large: {} days", duration.num_days());
            return Err(anyhow::anyhow!("Date range too large: {} days", duration.num_days()));
        }
    }

    // Security: Validate limit to prevent memory exhaustion
    const MAX_LIMIT: usize = 100000;
    let safe_limit = match limit {
        Some(l) if l > MAX_LIMIT => {
            warn!("Limit {} exceeds maximum {}, using maximum", l, MAX_LIMIT);
            MAX_LIMIT
        },
        Some(l) => l,
        None => 10000, // Default reasonable limit
    };

    let retry_strategy = ExponentialBackoff::from_millis(10).map(jitter).take(3);

    Retry::spawn(retry_strategy, || async {
        let mut connection = get_timescale_connection(pool.clone())
            .await
            .map_err(|e| {
                error!("Failed to get database connection: {}", e);
                anyhow::Error::from(e)
            })?;

        let mut query = candles
            .filter(symbol.eq(sym).and(exchange.eq(xchange)))
            .filter(timeframe.eq("5m"))
            .order(timestamp.asc())
            .limit(safe_limit as i64)
            .into_boxed();

        if let Some(start) = start_time {
            query = query.filter(timestamp.ge(start));
        }
        if let Some(end) = end_time {
            query = query.filter(timestamp.le(end));
        }

        let result = query
            .select(Candle::as_select())
            .load::<Candle>(&mut connection)
            .await
            .map_err(|e| anyhow::Error::from(e))?;
            
        info!("Fetched {} 5m candles in {}ms", result.len(), query_start.elapsed().as_millis());
        Ok(result)
    }).await
}

/// Get 15-minute candles for a symbol and exchange
pub async fn get_candles_15m(
    pool: Arc<deadpool::Pool<AsyncPgConnection>>, 
    sym: &str, 
    xchange: &str,
    start_time: Option<DateTime<Utc>>,
    end_time: Option<DateTime<Utc>>,
    limit: Option<usize>
) -> Result<Vec<Candle>, Error> {
    let query_start = Instant::now();
    use crate::schema::candles::dsl::*;

    // Security: Input validation
    if sym.is_empty() || sym.len() > 20 {
        error!("Invalid symbol length: {}", sym.len());
        return Err(anyhow::anyhow!("Invalid symbol length: {}", sym.len()));
    }
    
    if xchange.is_empty() || xchange.len() > 50 {
        error!("Invalid exchange length: {}", xchange.len());
        return Err(anyhow::anyhow!("Invalid exchange length: {}", xchange.len()));
    }

    // Security: Validate date range to prevent excessive queries
    if let (Some(start), Some(end)) = (start_time, end_time) {
        let duration = end - start;
        if duration > Duration::days(365) {
            error!("Date range too large: {} days", duration.num_days());
            return Err(anyhow::anyhow!("Date range too large: {} days", duration.num_days()));
        }
    }

    // Security: Validate limit to prevent memory exhaustion
    const MAX_LIMIT: usize = 100000;
    let safe_limit = match limit {
        Some(l) if l > MAX_LIMIT => {
            warn!("Limit {} exceeds maximum {}, using maximum", l, MAX_LIMIT);
            MAX_LIMIT
        },
        Some(l) => l,
        None => 10000, // Default reasonable limit
    };

    let retry_strategy = ExponentialBackoff::from_millis(10).map(jitter).take(3);

    Retry::spawn(retry_strategy, || async {
        let mut connection = get_timescale_connection(pool.clone())
            .await
            .map_err(|e| {
                error!("Failed to get database connection: {}", e);
                anyhow::Error::from(e)
            })?;

        let mut query = candles
            .filter(symbol.eq(sym).and(exchange.eq(xchange)))
            .filter(timeframe.eq("15m"))
            .order(timestamp.asc())
            .limit(safe_limit as i64)
            .into_boxed();

        if let Some(start) = start_time {
            query = query.filter(timestamp.ge(start));
        }
        if let Some(end) = end_time {
            query = query.filter(timestamp.le(end));
        }

        let result = query
            .select(Candle::as_select())
            .load::<Candle>(&mut connection)
            .await
            .map_err(|e| anyhow::Error::from(e))?;
            
        info!("Fetched {} 15m candles in {}ms", result.len(), query_start.elapsed().as_millis());
        Ok(result)
    }).await
}

/// Get 1-hour candles for a symbol and exchange
pub async fn get_candles_1h(
    pool: Arc<deadpool::Pool<AsyncPgConnection>>, 
    sym: &str, 
    xchange: &str,
    start_time: Option<DateTime<Utc>>,
    end_time: Option<DateTime<Utc>>,
    limit: Option<usize>
) -> Result<Vec<Candle>, Error> {
    let query_start = Instant::now();
    use crate::schema::candles::dsl::*;

    // Security: Input validation
    if sym.is_empty() || sym.len() > 20 {
        error!("Invalid symbol length: {}", sym.len());
        return Err(anyhow::anyhow!("Invalid symbol length: {}", sym.len()));
    }
    
    if xchange.is_empty() || xchange.len() > 50 {
        error!("Invalid exchange length: {}", xchange.len());
        return Err(anyhow::anyhow!("Invalid exchange length: {}", xchange.len()));
    }

    // Security: Validate date range to prevent excessive queries
    if let (Some(start), Some(end)) = (start_time, end_time) {
        let duration = end - start;
        if duration > Duration::days(365) {
            error!("Date range too large: {} days", duration.num_days());
            return Err(anyhow::anyhow!("Date range too large: {} days", duration.num_days()));
        }
    }

    // Security: Validate limit to prevent memory exhaustion
    const MAX_LIMIT: usize = 100000;
    let safe_limit = match limit {
        Some(l) if l > MAX_LIMIT => {
            warn!("Limit {} exceeds maximum {}, using maximum", l, MAX_LIMIT);
            MAX_LIMIT
        },
        Some(l) => l,
        None => 10000, // Default reasonable limit
    };

    let retry_strategy = ExponentialBackoff::from_millis(10).map(jitter).take(3);

    Retry::spawn(retry_strategy, || async {
        let mut connection = get_timescale_connection(pool.clone())
            .await
            .map_err(|e| {
                error!("Failed to get database connection: {}", e);
                anyhow::Error::from(e)
            })?;

        let mut query = candles
            .filter(symbol.eq(sym).and(exchange.eq(xchange)))
            .filter(timeframe.eq("1h"))
            .order(timestamp.asc())
            .limit(safe_limit as i64)
            .into_boxed();

        if let Some(start) = start_time {
            query = query.filter(timestamp.ge(start));
        }
        if let Some(end) = end_time {
            query = query.filter(timestamp.le(end));
        }

        let result = query
            .select(Candle::as_select())
            .load::<Candle>(&mut connection)
            .await
            .map_err(|e| anyhow::Error::from(e))?;
            
        info!("Fetched {} 1h candles in {}ms", result.len(), query_start.elapsed().as_millis());
        Ok(result)
    }).await
}

/// Get 1-day candles for a symbol and exchange
pub async fn get_candles_1d(
    pool: Arc<deadpool::Pool<AsyncPgConnection>>, 
    sym: &str, 
    xchange: &str,
    start_time: Option<DateTime<Utc>>,
    end_time: Option<DateTime<Utc>>,
    limit: Option<usize>
) -> Result<Vec<Candle>, Error> {
    let query_start = Instant::now();
    use crate::schema::candles::dsl::*;

    // Security: Input validation
    if sym.is_empty() || sym.len() > 20 {
        error!("Invalid symbol length: {}", sym.len());
        return Err(anyhow::anyhow!("Invalid symbol length: {}", sym.len()));
    }
    
    if xchange.is_empty() || xchange.len() > 50 {
        error!("Invalid exchange length: {}", xchange.len());
        return Err(anyhow::anyhow!("Invalid exchange length: {}", xchange.len()));
    }

    // Security: Validate date range to prevent excessive queries
    if let (Some(start), Some(end)) = (start_time, end_time) {
        let duration = end - start;
        if duration > Duration::days(365) {
            error!("Date range too large: {} days", duration.num_days());
            return Err(anyhow::anyhow!("Date range too large: {} days", duration.num_days()));
        }
    }

    // Security: Validate limit to prevent memory exhaustion
    const MAX_LIMIT: usize = 100000;
    let safe_limit = match limit {
        Some(l) if l > MAX_LIMIT => {
            warn!("Limit {} exceeds maximum {}, using maximum", l, MAX_LIMIT);
            MAX_LIMIT
        },
        Some(l) => l,
        None => 10000, // Default reasonable limit
    };

    let retry_strategy = ExponentialBackoff::from_millis(10).map(jitter).take(3);

    Retry::spawn(retry_strategy, || async {
        let mut connection = get_timescale_connection(pool.clone())
            .await
            .map_err(|e| {
                error!("Failed to get database connection: {}", e);
                anyhow::Error::from(e)
            })?;

        let mut query = candles
            .filter(symbol.eq(sym).and(exchange.eq(xchange)))
            .filter(timeframe.eq("1d"))
            .order(timestamp.asc())
            .limit(safe_limit as i64)
            .into_boxed();

        if let Some(start) = start_time {
            query = query.filter(timestamp.ge(start));
        }
        if let Some(end) = end_time {
            query = query.filter(timestamp.le(end));
        }

        let result = query
            .select(Candle::as_select())
            .load::<Candle>(&mut connection)
            .await
            .map_err(|e| anyhow::Error::from(e))?;
            
        info!("Fetched {} 1d candles in {}ms", result.len(), query_start.elapsed().as_millis());
        Ok(result)
    }).await
}
