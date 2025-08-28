// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "order_status"))]
    pub struct Order_status;

    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "order_side"))]
    pub struct Order_side;

    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "order_type"))]
    pub struct Order_type;

    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "time_in_force"))]
    pub struct Time_in_force;

    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "execution_urgency"))]
    pub struct Execution_urgency;
}

diesel::table! {
    exchanges (exchange_id) {
        created_at -> Timestamptz,
        exchange_id -> Uuid,
        #[max_length = 8]
        exchange -> Varchar,
    }
}

diesel::table! {
    historical_orders (event_id) {
        event_id -> Uuid,
        timestamp -> Timestamptz,
        order_id -> Text,
        event_type -> Text,
        side -> Text,
        price_level -> Numeric,
        quantity -> Numeric,
        prev_price -> Nullable<Numeric>,
        prev_quantity -> Nullable<Numeric>,
        status -> Text,
        exchange -> Text,
        symbol -> Text,
        exchange_id -> Uuid,
        security_id -> Uuid,
    }
}

diesel::table! {
    historical_snapshot (event_id) {
        event_id -> Uuid,
        timestamp -> Timestamptz,
        order_id -> Text,
        event_type -> Text,
        side -> Text,
        price_level -> Numeric,
        quantity -> Numeric,
        status -> Text,
        exchange -> Text,
        symbol -> Text,
        exchange_id -> Uuid,
        security_id -> Uuid,
    }
}

diesel::table! {
    open_buy_orders (created_at, unique_id) {
        created_at -> Timestamptz,
        #[max_length = 7]
        symbol -> Varchar,
        #[max_length = 8]
        exchange -> Varchar,
        security_id -> Uuid,
        exchange_id -> Uuid,
        buy_order_book_id -> Uuid,
        #[max_length = 255]
        unique_id -> Varchar,
        price_level -> Numeric,
        buy_quantity -> Numeric,
    }
}

diesel::table! {
    open_sell_orders (created_at, unique_id) {
        created_at -> Timestamptz,
        #[max_length = 7]
        symbol -> Varchar,
        #[max_length = 8]
        exchange -> Varchar,
        security_id -> Uuid,
        exchange_id -> Uuid,
        sell_order_book_id -> Uuid,
        #[max_length = 255]
        unique_id -> Varchar,
        price_level -> Numeric,
        sell_quantity -> Numeric,
    }
}

diesel::table! {
    order_books (order_book_id) {
        created_at -> Timestamptz,
        updated_at -> Nullable<Timestamptz>,
        #[max_length = 7]
        symbol -> Varchar,
        #[max_length = 8]
        exchange -> Varchar,
        security_id -> Uuid,
        exchange_id -> Uuid,
        order_book_id -> Uuid,
        buy_order_book_id -> Uuid,
        sell_order_book_id -> Uuid,
        total_volume -> Numeric,
    }
}

diesel::table! {
    securities (security_id) {
        created_at -> Timestamptz,
        security_id -> Uuid,
        #[max_length = 7]
        symbol -> Varchar,
    }
}

diesel::table! {
    sim_open_buy_orders (created_at, backtest_id, unique_id) {
        backtest_id -> Uuid,
        created_at -> Timestamptz,
        #[max_length = 7]
        symbol -> Varchar,
        #[max_length = 8]
        exchange -> Varchar,
        #[max_length = 255]
        unique_id -> Varchar,
        price_level -> Numeric,
        buy_quantity -> Numeric,
        created_id -> Uuid,
    }
}

diesel::table! {
    sim_open_sell_orders (created_at, unique_id) {
        backtest_id -> Uuid,
        created_at -> Timestamptz,
        #[max_length = 7]
        symbol -> Varchar,
        #[max_length = 8]
        exchange -> Varchar,
        #[max_length = 255]
        unique_id -> Varchar,
        price_level -> Numeric,
        sell_quantity -> Numeric,
        created_id -> Uuid,
    }
}

diesel::table! {
    sim_trades (created_at, backtest_id, trade_id) {
        backtest_id -> Uuid,
        created_at -> Timestamptz,
        #[max_length = 7]
        symbol -> Varchar,
        #[max_length = 8]
        exchange -> Varchar,
        trade_id -> Text,
        #[max_length = 4]
        side -> Varchar,
        price -> Numeric,
        quantity -> Numeric,
        matched_user -> Bool,
    }
}

diesel::table! {
    trades (created_at, trade_id) {
        created_at -> Timestamptz,
        #[max_length = 7]
        symbol -> Varchar,
        #[max_length = 8]
        exchange -> Varchar,
        trade_id -> Varchar,
        security_id -> Uuid,
        exchange_id -> Uuid,
        #[max_length = 4]
        side -> Varchar,
        price -> Numeric,
        quantity -> Numeric,
    }
}

diesel::table! {
    candles_1m (timestamp, symbol, exchange) {
        timestamp -> Timestamptz,
        #[max_length = 7]
        symbol -> Varchar,
        #[max_length = 8]
        exchange -> Varchar,
        security_id -> Uuid,
        exchange_id -> Uuid,
        open_price -> Numeric,
        high_price -> Numeric,
        low_price -> Numeric,
        close_price -> Numeric,
        volume -> Numeric,
        trade_count -> Int8,
    }
}

diesel::table! {
    candles_5m (timestamp, symbol, exchange) {
        timestamp -> Timestamptz,
        #[max_length = 7]
        symbol -> Varchar,
        #[max_length = 8]
        exchange -> Varchar,
        security_id -> Uuid,
        exchange_id -> Uuid,
        open_price -> Numeric,
        high_price -> Numeric,
        low_price -> Numeric,
        close_price -> Numeric,
        volume -> Numeric,
        trade_count -> Int8,
    }
}

diesel::table! {
    candles_15m (timestamp, symbol, exchange) {
        timestamp -> Timestamptz,
        #[max_length = 7]
        symbol -> Varchar,
        #[max_length = 8]
        exchange -> Varchar,
        security_id -> Uuid,
        exchange_id -> Uuid,
        open_price -> Numeric,
        high_price -> Numeric,
        low_price -> Numeric,
        close_price -> Numeric,
        volume -> Numeric,
        trade_count -> Int8,
    }
}

diesel::table! {
    candles_1h (timestamp, symbol, exchange) {
        timestamp -> Timestamptz,
        #[max_length = 7]
        symbol -> Varchar,
        #[max_length = 8]
        exchange -> Varchar,
        security_id -> Uuid,
        exchange_id -> Uuid,
        open_price -> Numeric,
        high_price -> Numeric,
        low_price -> Numeric,
        close_price -> Numeric,
        volume -> Numeric,
        trade_count -> Int8,
    }
}

diesel::table! {
    candles_1d (timestamp, symbol, exchange) {
        timestamp -> Timestamptz,
        #[max_length = 7]
        symbol -> Varchar,
        #[max_length = 8]
        exchange -> Varchar,
        security_id -> Uuid,
        exchange_id -> Uuid,
        open_price -> Numeric,
        high_price -> Numeric,
        low_price -> Numeric,
        close_price -> Numeric,
        volume -> Numeric,
        trade_count -> Int8,
    }
}

diesel::joinable!(historical_orders -> exchanges (exchange_id));
diesel::joinable!(historical_orders -> securities (security_id));
diesel::joinable!(historical_snapshot -> exchanges (exchange_id));
diesel::joinable!(historical_snapshot -> securities (security_id));

diesel::table! {
    backtest_results (id) {
        id -> Uuid,
        backtest_id -> Uuid,
        #[max_length = 255]
        strategy_name -> Varchar,
        #[max_length = 50]
        symbol -> Varchar,
        start_date -> Timestamptz,
        end_date -> Timestamptz,
        initial_capital -> Numeric,
        commission_rate -> Numeric,
        #[max_length = 50]
        slippage_model_type -> Varchar,
        slippage_fixed_rate -> Nullable<Numeric>,
        slippage_sqrt_rate -> Nullable<Numeric>,
        slippage_linear_rate -> Nullable<Numeric>,
        temporary_impact -> Numeric,
        permanent_impact -> Numeric,
        participation_rate_limit -> Numeric,
        #[max_length = 50]
        benchmark -> Nullable<Varchar>,
        #[max_length = 50]
        rebalancing_frequency -> Varchar,
        point_in_time -> Bool,
        warmup_period_days -> Int4,
        total_return -> Numeric,
        annualized_return -> Numeric,
        volatility -> Numeric,
        sharpe_ratio -> Nullable<Numeric>,
        sortino_ratio -> Nullable<Numeric>,
        max_drawdown -> Numeric,
        calmar_ratio -> Nullable<Numeric>,
        win_rate -> Numeric,
        profit_factor -> Numeric,
        avg_trade_return -> Numeric,
        total_trades -> Int4,
        best_trade -> Nullable<Numeric>,
        worst_trade -> Nullable<Numeric>,
        avg_time_in_trade -> Nullable<Numeric>,
        value_at_risk_95 -> Nullable<Numeric>,
        expected_shortfall_95 -> Nullable<Numeric>,
        beta -> Nullable<Numeric>,
        correlation_with_benchmark -> Nullable<Numeric>,
        tracking_error -> Nullable<Numeric>,
        information_ratio -> Nullable<Numeric>,
        jensen_alpha -> Nullable<Numeric>,
        max_drawdown_duration_days -> Nullable<Int4>,
        current_drawdown -> Numeric,
        avg_drawdown -> Nullable<Numeric>,
        benchmark_return -> Nullable<Numeric>,
        excess_return -> Nullable<Numeric>,
        outperformance_periods -> Nullable<Int4>,
        underperformance_periods -> Nullable<Int4>,
        total_orders -> Int4,
        filled_orders -> Int4,
        cancelled_orders -> Int4,
        avg_slippage -> Numeric,
        total_commission_paid -> Numeric,
        avg_fill_time_seconds -> Nullable<Numeric>,
        strategy_metrics -> Nullable<Jsonb>,
        strategy_instance_id -> Nullable<Uuid>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    backtest_trades (id) {
        id -> Uuid,
        backtest_result_id -> Uuid,
        trade_id -> Uuid,
        order_id -> Uuid,
        #[max_length = 50]
        symbol -> Varchar,
        #[max_length = 10]
        side -> Varchar,
        quantity -> Numeric,
        price -> Numeric,
        commission -> Numeric,
        timestamp -> Timestamptz,
        metadata -> Nullable<Jsonb>,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    backtest_equity_curve (id) {
        id -> Uuid,
        backtest_result_id -> Uuid,
        timestamp -> Timestamptz,
        portfolio_value -> Numeric,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    backtest_position_history (id) {
        id -> Uuid,
        backtest_result_id -> Uuid,
        timestamp -> Timestamptz,
        #[max_length = 50]
        symbol -> Varchar,
        quantity -> Numeric,
        average_price -> Numeric,
        current_price -> Numeric,
        unrealized_pnl -> Numeric,
        #[max_length = 10]
        direction -> Varchar,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    backtest_drawdown_periods (id) {
        id -> Uuid,
        backtest_result_id -> Uuid,
        start_date -> Timestamptz,
        end_date -> Timestamptz,
        duration_days -> Int4,
        magnitude -> Numeric,
        recovery_date -> Nullable<Timestamptz>,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    strategies (id) {
        id -> Uuid,
        #[max_length = 255]
        strategy_name -> Varchar,
        #[max_length = 100]
        strategy_type -> Varchar,
        #[max_length = 50]
        version -> Varchar,
        description -> Nullable<Text>,
        #[max_length = 255]
        created_by -> Nullable<Varchar>,
        is_active -> Bool,
        base_configuration -> Nullable<Jsonb>,
        metadata -> Nullable<Jsonb>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    strategy_parameters (id) {
        id -> Uuid,
        strategy_id -> Uuid,
        #[max_length = 255]
        parameter_name -> Varchar,
        #[max_length = 50]
        parameter_type -> Varchar,
        is_required -> Bool,
        default_value -> Nullable<Jsonb>,
        min_value -> Nullable<Numeric>,
        max_value -> Nullable<Numeric>,
        allowed_values -> Nullable<Jsonb>,
        #[max_length = 500]
        validation_pattern -> Nullable<Varchar>,
        #[max_length = 255]
        display_name -> Nullable<Varchar>,
        description -> Nullable<Text>,
        #[max_length = 100]
        parameter_group -> Nullable<Varchar>,
        display_order -> Nullable<Int4>,
        is_optimizable -> Bool,
        optimization_min -> Nullable<Numeric>,
        optimization_max -> Nullable<Numeric>,
        optimization_step -> Nullable<Numeric>,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    strategy_instances (id) {
        id -> Uuid,
        strategy_id -> Uuid,
        #[max_length = 255]
        instance_name -> Nullable<Varchar>,
        description -> Nullable<Text>,
        parameters -> Jsonb,
        performance_summary -> Nullable<Jsonb>,
        risk_metrics -> Nullable<Jsonb>,
        is_template -> Bool,
        tags -> Nullable<Array<Nullable<Text>>>,
        #[max_length = 255]
        created_by -> Nullable<Varchar>,
        optimization_run_id -> Nullable<Uuid>,
        optimization_score -> Nullable<Numeric>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    optimization_runs (id) {
        id -> Uuid,
        strategy_id -> Uuid,
        #[max_length = 255]
        run_name -> Varchar,
        #[max_length = 100]
        optimization_method -> Varchar,
        #[max_length = 100]
        objective_function -> Varchar,
        optimization_config -> Nullable<Jsonb>,
        parameter_ranges -> Jsonb,
        constraints -> Nullable<Jsonb>,
        #[max_length = 50]
        status -> Varchar,
        total_iterations -> Nullable<Int4>,
        completed_iterations -> Nullable<Int4>,
        best_score -> Nullable<Numeric>,
        best_parameters -> Nullable<Jsonb>,
        started_at -> Nullable<Timestamptz>,
        completed_at -> Nullable<Timestamptz>,
        error_message -> Nullable<Text>,
        #[max_length = 255]
        created_by -> Nullable<Varchar>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    optimization_iterations (id) {
        id -> Uuid,
        optimization_run_id -> Uuid,
        iteration_number -> Int4,
        parameters -> Jsonb,
        objective_score -> Nullable<Numeric>,
        additional_metrics -> Nullable<Jsonb>,
        started_at -> Timestamptz,
        completed_at -> Nullable<Timestamptz>,
        execution_time_ms -> Nullable<Int4>,
        #[max_length = 50]
        status -> Varchar,
        error_message -> Nullable<Text>,
    }
}

diesel::table! {
    strategy_comparisons (id) {
        id -> Uuid,
        #[max_length = 255]
        comparison_name -> Varchar,
        description -> Nullable<Text>,
        strategies -> Jsonb,
        comparison_period -> Nullable<Jsonb>,
        #[max_length = 20]
        benchmark_symbol -> Nullable<Varchar>,
        results -> Nullable<Jsonb>,
        summary -> Nullable<Jsonb>,
        #[max_length = 255]
        created_by -> Nullable<Varchar>,
        created_at -> Timestamptz,
    }
}

diesel::joinable!(backtest_trades -> backtest_results (backtest_result_id));
diesel::joinable!(backtest_equity_curve -> backtest_results (backtest_result_id));
diesel::joinable!(backtest_position_history -> backtest_results (backtest_result_id));
diesel::joinable!(backtest_drawdown_periods -> backtest_results (backtest_result_id));
diesel::joinable!(backtest_results -> strategy_instances (strategy_instance_id));
diesel::joinable!(strategy_parameters -> strategies (strategy_id));
diesel::joinable!(strategy_instances -> strategies (strategy_id));
diesel::joinable!(strategy_instances -> optimization_runs (optimization_run_id));
diesel::joinable!(optimization_runs -> strategies (strategy_id));
diesel::joinable!(optimization_iterations -> optimization_runs (optimization_run_id));
diesel::joinable!(candles_1m -> exchanges (exchange_id));
diesel::joinable!(candles_1m -> securities (security_id));
diesel::joinable!(candles_5m -> exchanges (exchange_id));
diesel::joinable!(candles_5m -> securities (security_id));
diesel::joinable!(candles_15m -> exchanges (exchange_id));
diesel::joinable!(candles_15m -> securities (security_id));
diesel::joinable!(candles_1h -> exchanges (exchange_id));
diesel::joinable!(candles_1h -> securities (security_id));
diesel::joinable!(candles_1d -> exchanges (exchange_id));
diesel::joinable!(candles_1d -> securities (security_id));

diesel::allow_tables_to_appear_in_same_query!(
    backtest_results,
    backtest_trades,
    backtest_equity_curve,
    backtest_position_history,
    backtest_drawdown_periods,
    backtest_reports,
    backtest_report_access_log,
    exchanges,
    historical_orders,
    historical_snapshot,
    open_buy_orders,
    open_sell_orders,
    order_books,
    securities,
    sim_open_buy_orders,
    sim_open_sell_orders,
    sim_trades,
    trades,
    candles_1m,
    candles_5m,
    candles_15m,
    candles_1h,
    candles_1d,
    strategies,
    strategy_parameters,
    strategy_instances,
    optimization_runs,
    optimization_iterations,
    strategy_comparisons,
);

diesel::table! {
    backtest_reports (id) {
        id -> Uuid,
        backtest_result_id -> Uuid,
        #[max_length = 255]
        report_id -> Varchar,
        #[max_length = 255]
        report_name -> Varchar,
        #[max_length = 255]
        strategy_name -> Varchar,
        #[max_length = 50]
        symbol -> Varchar,
        #[max_length = 20]
        timeframe -> Varchar,
        start_date -> Date,
        end_date -> Date,
        initial_capital -> Numeric,
        generated_at -> Timestamptz,
        #[max_length = 255]
        generated_by -> Nullable<Varchar>,
        #[max_length = 50]
        generation_source -> Varchar,
        backtest_duration_seconds -> Nullable<Numeric>,
        data_points -> Nullable<Int4>,
        include_trades -> Bool,
        include_charts -> Bool,
        export_formats -> Array<Text>,
        custom_css -> Nullable<Text>,
        #[max_length = 50]
        template_version -> Nullable<Varchar>,
        file_paths -> Jsonb,
        file_sizes -> Nullable<Jsonb>,
        #[max_length = 50]
        storage_location -> Varchar,
        performance_summary -> Jsonb,
        risk_summary -> Jsonb,
        trade_summary -> Jsonb,
        #[max_length = 20]
        status -> Varchar,
        error_message -> Nullable<Text>,
        tags -> Nullable<Array<Text>>,
        notes -> Nullable<Text>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        accessed_at -> Nullable<Timestamptz>,
        access_count -> Int4,
    }
}

diesel::table! {
    backtest_report_access_log (id) {
        id -> Uuid,
        report_id -> Uuid,
        #[max_length = 255]
        accessed_by -> Nullable<Varchar>,
        #[max_length = 50]
        access_method -> Varchar,
        #[max_length = 20]
        format_requested -> Nullable<Varchar>,
        user_agent -> Nullable<Text>,
        ip_address -> Nullable<Inet>,
        response_time_ms -> Nullable<Int4>,
        success -> Bool,
        error_message -> Nullable<Text>,
        accessed_at -> Timestamptz,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::*;

    strategy_orders (id) {
        id -> Uuid,
        signal_id -> Int8,
        strategy_instance_id -> Nullable<Uuid>,
        parent_order_id -> Nullable<Uuid>,
        #[max_length = 255]
        exchange_order_id -> Nullable<Varchar>,
        #[max_length = 255]
        unique_id -> Varchar,
        #[max_length = 20]
        symbol -> Varchar,
        #[max_length = 50]
        exchange -> Varchar,
        side -> Order_side,
        order_type -> Order_type,
        time_in_force -> Nullable<Time_in_force>,
        original_quantity -> Numeric,
        remaining_quantity -> Numeric,
        filled_quantity -> Nullable<Numeric>,
        price -> Nullable<Numeric>,
        stop_price -> Nullable<Numeric>,
        avg_fill_price -> Nullable<Numeric>,
        status -> Order_status,
        urgency -> Nullable<Execution_urgency>,
        fees_paid -> Nullable<Numeric>,
        #[max_length = 255]
        strategy_name -> Varchar,
        #[max_length = 50]
        strategy_version -> Nullable<Varchar>,
        signal_confidence -> Nullable<Numeric>,
        signal_flags -> Nullable<Int4>,
        risk_score -> Nullable<Numeric>,
        compliance_checked -> Nullable<Bool>,
        risk_limits_checked -> Nullable<Bool>,
        #[max_length = 50]
        routing_algorithm -> Nullable<Varchar>,
        #[max_length = 50]
        execution_venue -> Nullable<Varchar>,
        child_order_count -> Nullable<Int4>,
        slippage_bps -> Nullable<Int4>,
        implementation_shortfall_bps -> Nullable<Int4>,
        market_impact_bps -> Nullable<Int4>,
        order_metadata -> Nullable<Jsonb>,
        execution_context -> Nullable<Jsonb>,
        tags -> Nullable<Array<Varchar>>,
        rejection_reason -> Nullable<Text>,
        error_message -> Nullable<Text>,
        retry_count -> Nullable<Int4>,
        signal_timestamp -> Timestamptz,
        order_created_at -> Timestamptz,
        order_submitted_at -> Nullable<Timestamptz>,
        first_fill_at -> Nullable<Timestamptz>,
        last_fill_at -> Nullable<Timestamptz>,
        completed_at -> Nullable<Timestamptz>,
        #[max_length = 255]
        created_by -> Nullable<Varchar>,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    use diesel::sql_types::*;

    strategy_order_fills (id) {
        id -> Uuid,
        order_id -> Uuid,
        #[max_length = 255]
        fill_id -> Varchar,
        #[max_length = 255]
        trade_id -> Nullable<Varchar>,
        quantity -> Numeric,
        price -> Numeric,
        fees -> Nullable<Numeric>,
        #[max_length = 10]
        fee_currency -> Nullable<Varchar>,
        bid_price -> Nullable<Numeric>,
        ask_price -> Nullable<Numeric>,
        mid_price -> Nullable<Numeric>,
        spread_bps -> Nullable<Int4>,
        is_maker -> Nullable<Bool>,
        #[max_length = 10]
        liquidity_flag -> Nullable<Varchar>,
        fill_timestamp -> Timestamptz,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::*;

    strategy_order_state_changes (id) {
        id -> Uuid,
        order_id -> Uuid,
        previous_status -> Nullable<Order_status>,
        new_status -> Order_status,
        previous_quantity -> Nullable<Numeric>,
        new_quantity -> Nullable<Numeric>,
        #[max_length = 255]
        change_reason -> Nullable<Varchar>,
        #[max_length = 100]
        triggered_by -> Nullable<Varchar>,
        exchange_message -> Nullable<Text>,
        state_data -> Nullable<Jsonb>,
        changed_at -> Timestamptz,
        #[max_length = 255]
        changed_by -> Nullable<Varchar>,
    }
}

diesel::joinable!(backtest_reports -> backtest_results (backtest_result_id));
diesel::joinable!(backtest_report_access_log -> backtest_reports (report_id));
diesel::joinable!(strategy_order_fills -> strategy_orders (order_id));
diesel::joinable!(strategy_order_state_changes -> strategy_orders (order_id));
diesel::joinable!(strategy_orders -> strategy_instances (strategy_instance_id));
