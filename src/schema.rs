// @generated automatically by Diesel CLI.

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

diesel::joinable!(historical_orders -> exchanges (exchange_id));
diesel::joinable!(historical_orders -> securities (security_id));
diesel::joinable!(historical_snapshot -> exchanges (exchange_id));
diesel::joinable!(historical_snapshot -> securities (security_id));

diesel::allow_tables_to_appear_in_same_query!(
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
);
