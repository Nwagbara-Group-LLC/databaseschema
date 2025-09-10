use crate::models::strategy_order::*;
use crate::schema;
use anyhow::Result;
use chrono::Utc;
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, AsyncConnection, RunQueryDsl};
use uuid::Uuid;
use tracing::{info, error, warn, debug};
use std::time::Instant;
use bigdecimal::BigDecimal;

/// Strategy Order Operations
pub struct StrategyOrderOps;

impl StrategyOrderOps {
    /// Create a new strategy order
    pub async fn create_order(
        conn: &mut AsyncPgConnection,
        order: NewStrategyOrder,
    ) -> Result<StrategyOrder> {
        let start_time = Instant::now();
        
        // Input validation
        if order.unique_id.is_empty() {
            error!("Order creation failed: empty unique_id");
            return Err(anyhow::anyhow!("unique_id cannot be empty"));
        }
        
        if order.symbol.is_empty() || order.symbol.len() > 20 {
            error!("Order creation failed: invalid symbol length");
            return Err(anyhow::anyhow!("symbol must be 1-20 characters"));
        }
        
        if order.original_quantity <= BigDecimal::from(0) {
            error!("Order creation failed: invalid quantity");
            return Err(anyhow::anyhow!("quantity must be positive"));
        }

        let inserted_order = diesel::insert_into(schema::strategy_orders::table)
            .values(&order)
            .get_result(conn)
            .await
            .map_err(|e| {
                error!("Database error creating order: {}", e);
                anyhow::anyhow!("Failed to create order")
            })?;
            
        debug!("Order created in {}ms", start_time.elapsed().as_millis());
        Ok(inserted_order)
    }

    /// Get order by ID
    pub async fn get_order_by_id(
        conn: &mut AsyncPgConnection,
        order_id: Uuid,
    ) -> Result<Option<StrategyOrder>> {
        let order = schema::strategy_orders::table
            .filter(schema::strategy_orders::id.eq(order_id))
            .select(StrategyOrder::as_select())
            .first::<StrategyOrder>(conn)
            .await
            .optional()
            .map_err(|e| {
                error!("Database error fetching order by id: {}", e);
                anyhow::anyhow!("Failed to fetch order")
            })?;
        Ok(order)
    }

    /// Get order by unique ID
    pub async fn get_order_by_unique_id(
        conn: &mut AsyncPgConnection,
        unique_order_id: String,
    ) -> Result<Option<StrategyOrder>> {
        if unique_order_id.is_empty() {
            error!("get_order_by_unique_id called with empty unique_id");
            return Err(anyhow::anyhow!("unique_id cannot be empty"));
        }
        
        let order = schema::strategy_orders::table
            .filter(schema::strategy_orders::unique_id.eq(unique_order_id))
            .select(StrategyOrder::as_select())
            .first::<StrategyOrder>(conn)
            .await
            .optional()
            .map_err(|e| {
                error!("Database error fetching order by unique_id: {}", e);
                anyhow::anyhow!("Failed to fetch order")
            })?;
        Ok(order)
    }

    /// Get all orders for a strategy instance
    pub async fn get_orders_by_strategy_instance(
        conn: &mut AsyncPgConnection,
        strategy_instance_id: Uuid,
    ) -> Result<Vec<StrategyOrder>> {
        let start_time = Instant::now();
        
        let orders = schema::strategy_orders::table
            .filter(schema::strategy_orders::strategy_instance_id.eq(Some(strategy_instance_id)))
            .order(schema::strategy_orders::order_created_at.desc())
            .limit(10000) // Prevent memory exhaustion
            .select(StrategyOrder::as_select())
            .load::<StrategyOrder>(conn)
            .await
            .map_err(|e| {
                error!("Database error fetching orders by strategy instance: {}", e);
                anyhow::anyhow!("Failed to fetch orders")
            })?;
            
        debug!("Fetched {} orders in {}ms", orders.len(), start_time.elapsed().as_millis());
        Ok(orders)
    }

    /// Get orders by status
    pub async fn get_orders_by_status(
        conn: &mut AsyncPgConnection,
        order_status: OrderStatus,
        limit: Option<i64>,
    ) -> Result<Vec<StrategyOrder>> {
        let start_time = Instant::now();
        
        // Validate limit to prevent memory exhaustion
        const MAX_LIMIT: i64 = 10000;
        let safe_limit = match limit {
            Some(l) if l > MAX_LIMIT => {
                warn!("Limit {} exceeds maximum {}, using maximum", l, MAX_LIMIT);
                MAX_LIMIT
            },
            Some(l) if l <= 0 => {
                error!("Invalid limit: {}", l);
                return Err(anyhow::anyhow!("limit must be positive"));
            },
            Some(l) => l,
            None => 1000, // Default reasonable limit
        };

        let query = schema::strategy_orders::table
            .filter(schema::strategy_orders::status.eq(order_status))
            .order(schema::strategy_orders::order_created_at.desc())
            .limit(safe_limit)
            .select(StrategyOrder::as_select())
            .into_boxed();

        let orders = query.load::<StrategyOrder>(conn)
            .await
            .map_err(|e| {
                error!("Database error fetching orders by status: {}", e);
                anyhow::anyhow!("Failed to fetch orders")
            })?;
            
        info!("Fetched {} orders in {}ms", orders.len(), start_time.elapsed().as_millis());
        Ok(orders)
    }

    /// Update order status
    pub async fn update_order_status(
        conn: &mut AsyncPgConnection,
        order_id: Uuid,
        new_status: OrderStatus,
    ) -> Result<StrategyOrder> {
        let start_time = Instant::now();
        
        let updated_order = diesel::update(schema::strategy_orders::table.filter(schema::strategy_orders::id.eq(order_id)))
            .set((
                schema::strategy_orders::status.eq(new_status),
                schema::strategy_orders::updated_at.eq(Utc::now()),
            ))
            .get_result(conn)
            .await
            .map_err(|e| {
                error!("Database error updating order status: {}", e);
                anyhow::anyhow!("Failed to update order")
            })?;
            
        debug!("Order status updated in {}ms", start_time.elapsed().as_millis());
        Ok(updated_order)
    }

    /// Cancel order
    pub async fn cancel_order(
        conn: &mut AsyncPgConnection,
        order_id: Uuid,
        cancellation_reason: String,
    ) -> Result<StrategyOrder> {
        let start_time = Instant::now();
        
        if cancellation_reason.is_empty() {
            error!("Cancel order called with empty cancellation reason");
            return Err(anyhow::anyhow!("cancellation_reason cannot be empty"));
        }
        
        let updated_order = diesel::update(schema::strategy_orders::table.filter(schema::strategy_orders::id.eq(order_id)))
            .set((
                schema::strategy_orders::status.eq(OrderStatus::Cancelled),
                schema::strategy_orders::rejection_reason.eq(Some(cancellation_reason)),
                schema::strategy_orders::completed_at.eq(Some(Utc::now())),
                schema::strategy_orders::updated_at.eq(Utc::now()),
            ))
            .get_result(conn)
            .await
            .map_err(|e| {
                error!("Database error cancelling order: {}", e);
                anyhow::anyhow!("Failed to cancel order")
            })?;
            
        debug!("Order cancelled in {}ms", start_time.elapsed().as_millis());
        Ok(updated_order)
    }
}

/// Strategy Order Fill Operations
pub struct StrategyOrderFillOps;

impl StrategyOrderFillOps {
    /// Add a fill to an order
    pub async fn create_fill(
        conn: &mut AsyncPgConnection,
        fill: NewStrategyOrderFill,
    ) -> Result<StrategyOrderFill> {
        let start_time = Instant::now();
        
        // Input validation
        if fill.quantity <= BigDecimal::from(0) {
            error!("Create fill failed: invalid quantity");
            return Err(anyhow::anyhow!("fill quantity must be positive"));
        }
        
        if fill.price <= BigDecimal::from(0) {
            error!("Create fill failed: invalid price");
            return Err(anyhow::anyhow!("fill price must be positive"));
        }
        
        let inserted_fill = diesel::insert_into(schema::strategy_order_fills::table)
            .values(&fill)
            .get_result(conn)
            .await
            .map_err(|e| {
                error!("Database error creating fill: {}", e);
                anyhow::anyhow!("Failed to create fill")
            })?;
            
        debug!("Fill created in {}ms", start_time.elapsed().as_millis());
        Ok(inserted_fill)
    }

    /// Get all fills for an order
    pub async fn get_fills_by_order(
        conn: &mut AsyncPgConnection,
        order_id: Uuid,
    ) -> Result<Vec<StrategyOrderFill>> {
        let start_time = Instant::now();
        
        let fills = schema::strategy_order_fills::table
            .filter(schema::strategy_order_fills::order_id.eq(order_id))
            .order(schema::strategy_order_fills::fill_timestamp.asc())
            .limit(1000) // Prevent memory exhaustion
            .load::<StrategyOrderFill>(conn)
            .await
            .map_err(|e| {
                error!("Database error fetching fills: {}", e);
                anyhow::anyhow!("Failed to fetch fills")
            })?;
            
        debug!("Fetched {} fills in {}ms", fills.len(), start_time.elapsed().as_millis());
        Ok(fills)
    }
}

/// Strategy Order State Change Operations  
pub struct StrategyOrderStateChangeOps;

impl StrategyOrderStateChangeOps {
    /// Record a state change for an order
    pub async fn create_state_change(
        conn: &mut AsyncPgConnection,
        state_change: NewStrategyOrderStateChange,
    ) -> Result<StrategyOrderStateChange> {
        let start_time = Instant::now();
        
        let inserted_change = diesel::insert_into(schema::strategy_order_state_changes::table)
            .values(&state_change)
            .get_result(conn)
            .await
            .map_err(|e| {
                error!("Database error creating state change: {}", e);
                anyhow::anyhow!("Failed to create state change")
            })?;
            
        debug!("State change created in {}ms", start_time.elapsed().as_millis());
        Ok(inserted_change)
    }

    /// Get all state changes for an order
    pub async fn get_state_changes_by_order(
        conn: &mut AsyncPgConnection,
        order_id: Uuid,
    ) -> Result<Vec<StrategyOrderStateChange>> {
        let start_time = Instant::now();
        
        let changes = schema::strategy_order_state_changes::table
            .filter(schema::strategy_order_state_changes::order_id.eq(order_id))
            .order(schema::strategy_order_state_changes::changed_at.asc())
            .limit(1000) // Prevent memory exhaustion
            .load::<StrategyOrderStateChange>(conn)
            .await
            .map_err(|e| {
                error!("Database error fetching state changes: {}", e);
                anyhow::anyhow!("Failed to fetch state changes")
            })?;
            
        debug!("Fetched {} state changes in {}ms", changes.len(), start_time.elapsed().as_millis());
        Ok(changes)
    }
}

/// Combined Operations for Workflows
pub struct StrategyOrderWorkflow;

impl StrategyOrderWorkflow {
    /// Create order with initial state change record
    pub async fn create_order_with_state(
        conn: &mut AsyncPgConnection,
        order: NewStrategyOrder,
        created_by: Option<String>,
    ) -> Result<(StrategyOrder, StrategyOrderStateChange)> {
        let start_time = Instant::now();
        
        let result = conn.transaction::<_, anyhow::Error, _>(|conn| Box::pin(async move {
            // Create the order
            let created_order = StrategyOrderOps::create_order(conn, order).await?;
            
            // Record initial state change
            let initial_state = NewStrategyOrderStateChange {
                order_id: created_order.id,
                previous_status: None,
                new_status: created_order.status.clone(),
                previous_quantity: None,
                new_quantity: Some(created_order.original_quantity.clone()),
                change_reason: Some("Order created".to_string()),
                triggered_by: Some("System".to_string()),
                exchange_message: None,
                state_data: None,
                changed_by: created_by,
            };
            
            let state_change = StrategyOrderStateChangeOps::create_state_change(conn, initial_state).await?;
            
            Ok((created_order, state_change))
        })).await?;
        
        info!("Order with state created in {}ms", start_time.elapsed().as_millis());
        Ok(result)
    }
}
