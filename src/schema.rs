// @generated automatically by Diesel CLI.

diesel::table! {
    coin_categories (id) {
        #[max_length = 64]
        id -> Varchar,
        #[max_length = 128]
        name -> Varchar,
        market_cap -> Nullable<Decimal>,
        market_cap_change_24h -> Nullable<Decimal>,
        content -> Nullable<Text>,
        top_3_coins_id -> Json,
        top_3_coins -> Json,
        volume_24h -> Nullable<Decimal>,
        updated_at -> Nullable<Datetime>,
    }
}

diesel::table! {
    coin_data_info (id) {
        #[max_length = 64]
        id -> Varchar,
        #[max_length = 16]
        symbol -> Varchar,
        #[max_length = 64]
        name -> Varchar,
        #[max_length = 64]
        web_slug -> Nullable<Varchar>,
        #[max_length = 64]
        asset_platform_id -> Nullable<Varchar>,
        platforms -> Nullable<Json>,
        detail_platforms -> Nullable<Json>,
        block_time_in_minutes -> Nullable<Unsigned<Integer>>,
        #[max_length = 64]
        hashing_algorithm -> Nullable<Varchar>,
        categories -> Nullable<Json>,
        preview_listing -> Nullable<Bool>,
        public_notice -> Nullable<Text>,
        additional_notices -> Nullable<Json>,
        description -> Nullable<Json>,
        links -> Nullable<Json>,
        image -> Nullable<Json>,
        #[max_length = 64]
        country_origin -> Nullable<Varchar>,
        genesis_date -> Nullable<Date>,
        sentiment_votes_up_percentage -> Nullable<Decimal>,
        sentiment_votes_down_percentage -> Nullable<Decimal>,
        watchlist_portfolio_users -> Nullable<Unsigned<Integer>>,
        market_cap_rank -> Nullable<Unsigned<Integer>>,
        status_updates -> Nullable<Json>,
        last_updated -> Nullable<Datetime>,
    }
}

diesel::table! {
    coin_rank_info (id) {
        #[max_length = 64]
        id -> Varchar,
        #[max_length = 16]
        symbol -> Varchar,
        #[max_length = 64]
        name -> Varchar,
        #[max_length = 255]
        image -> Nullable<Varchar>,
        current_price -> Nullable<Decimal>,
        price_change_24h -> Nullable<Decimal>,
        price_change_percentage_24h -> Nullable<Decimal>,
        market_cap -> Nullable<Decimal>,
        market_cap_rank -> Nullable<Unsigned<Integer>>,
        market_cap_change_24h -> Nullable<Decimal>,
        market_cap_change_percentage_24h -> Nullable<Decimal>,
        fully_diluted_valuation -> Nullable<Decimal>,
        total_volume -> Nullable<Decimal>,
        high_24h -> Nullable<Decimal>,
        low_24h -> Nullable<Decimal>,
        circulating_supply -> Nullable<Decimal>,
        total_supply -> Nullable<Decimal>,
        max_supply -> Nullable<Decimal>,
        ath -> Nullable<Decimal>,
        ath_change_percentage -> Nullable<Decimal>,
        ath_date -> Nullable<Datetime>,
        atl -> Nullable<Decimal>,
        atl_change_percentage -> Nullable<Decimal>,
        atl_date -> Nullable<Datetime>,
        roi -> Nullable<Json>,
        last_updated -> Nullable<Datetime>,
    }
}

diesel::table! {
    market_kline (id) {
        #[max_length = 250]
        id -> Varchar,
        #[max_length = 64]
        exchange -> Varchar,
        #[max_length = 64]
        symbol -> Varchar,
        #[max_length = 16]
        time_frame -> Varchar,
        open_time -> Bigint,
        open -> Double,
        high -> Double,
        low -> Double,
        close -> Double,
        volume -> Double,
        close_time -> Bigint,
        quote_asset_volume -> Nullable<Double>,
        number_of_trades -> Nullable<Unsigned<Bigint>> ,
        taker_buy_base_asset_volume -> Nullable<Double>,
        taker_buy_quote_asset_volume -> Nullable<Double>,
    }
}

diesel::table! {
    market_symbol (id) {
        #[max_length = 250]
        id -> Varchar,
        #[max_length = 50]
        exchange -> Varchar,
        #[max_length = 50]
        symbol -> Varchar,
        #[max_length = 50]
        pair -> Varchar,
        #[max_length = 30]
        contract_type -> Varchar,
        delivery_date -> Bigint,
        onboard_date -> Bigint,
        #[max_length = 30]
        status -> Varchar,
        #[max_length = 50]
        maint_margin_percent -> Varchar,
        #[max_length = 50]
        required_margin_percent -> Varchar,
        #[max_length = 30]
        base_asset -> Varchar,
        #[max_length = 30]
        quote_asset -> Varchar,
        #[max_length = 30]
        margin_asset -> Varchar,
        price_precision -> Unsigned<Bigint>,
        quantity_precision -> Unsigned<Bigint>,
        base_asset_precision -> Unsigned<Bigint>,
        quote_precision -> Unsigned<Bigint>,
        #[max_length = 50]
        underlying_type -> Varchar,
        underlying_sub_type -> Nullable<Json>,
        #[max_length = 50]
        trigger_protect -> Varchar,
        #[max_length = 50]
        liquidation_fee -> Varchar,
        #[max_length = 50]
        market_take_bound -> Varchar,
        max_move_order_limit -> Unsigned<Bigint>,
        filters -> Nullable<Json>,
        order_types -> Nullable<Json>,
        time_in_force -> Nullable<Json>,
        permission_sets -> Nullable<Json>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    coin_categories,
    coin_data_info,
    coin_rank_info,
    market_kline,
    market_symbol,
);
