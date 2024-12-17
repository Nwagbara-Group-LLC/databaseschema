// @generated automatically by Diesel CLI.

diesel::table! {
    open_buy_candlestick_agg (bucket) {
        bucket -> Timestamptz,
        #[max_length = 7]
        symbol -> Varchar,
        #[max_length = 8]
        exchange -> Varchar,
        low_buy_price -> Numeric,
        high_buy_price -> Numeric,
        open_buy_price -> Numeric,
        close_buy_price -> Numeric,
        total_buy_volume -> Numeric,
    }
}

diesel::table! {
    modified_buy_candlestick_agg (bucket) {
        bucket -> Timestamptz,
        #[max_length = 7]
        symbol -> Varchar,
        #[max_length = 8]
        exchange -> Varchar,
        low_buy_price -> Numeric,
        high_buy_price -> Numeric,
        open_buy_price -> Numeric,
        close_buy_price -> Numeric,
        total_buy_volume -> Numeric,
    }
}

diesel::table! {
    open_sell_candlestick_agg (bucket) {
        bucket -> Timestamptz,
        #[max_length = 7]
        symbol -> Varchar,
        #[max_length = 8]
        exchange -> Varchar,
        low_sell_price -> Numeric,
        high_sell_price -> Numeric,
        open_sell_price -> Numeric,
        close_sell_price -> Numeric,
        total_sell_volume -> Numeric,
    }
}

diesel::table! {
    modified_sell_candlestick_agg (bucket) {
        bucket -> Timestamptz,
        #[max_length = 7]
        symbol -> Varchar,
        #[max_length = 8]
        exchange -> Varchar,
        low_sell_price -> Numeric,
        high_sell_price -> Numeric,
        open_sell_price -> Numeric,
        close_sell_price -> Numeric,
        total_sell_volume -> Numeric,
    }
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
    modified_buy_orders (created_at, unique_id) {
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
        new_buy_quantity -> Numeric,
    }
}

diesel::table! {
    modified_sell_orders (created_at, unique_id) {
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
        new_sell_quantity -> Numeric,
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
    trades (created_at, trade_id) {
        created_at -> Timestamptz,
        #[max_length = 7]
        symbol -> Varchar,
        #[max_length = 8]
        exchange -> Varchar,
        trade_id -> Uuid,
        security_id -> Uuid,
        exchange_id -> Uuid,
        #[max_length = 4]
        side -> Varchar,
        price -> Numeric,
        quantity -> Numeric,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    exchanges,
    modified_buy_orders,
    modified_sell_orders,
    open_buy_orders,
    open_sell_orders,
    order_books,
    securities,
    trades,
);
