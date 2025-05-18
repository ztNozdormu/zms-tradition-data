# zms-tradition-data
zms-tradition-data 

## 组件

 clickhouse
 redis

## 设计思路
 1. 维护交易所(币安)历史数据[历史最后维护时间到现在,历史最远维护时间到过去;历史维护时间不存在就取当前时间到现在]
 2. 以ticker数据订阅事件更新最新k线数据 todo;弥补定时任务延时情况
 3. 提供数据接口 时间范围 周期 币种名称,redis缓存机制 
## 数据监听 统计
 1. 初筛币种，板块，市值，交易量流动性评估 直接第三方网站爬,保存到redis
 2. SOL链DEX交易实时监控，链上事件数据过滤； 
 3. 聪明钱包，dev统计，跟踪
 4. 机器人更敏感的数据监听狙击，跟单，套利
## 
https://coinmarketcap.com/api/documentation/v1/#section/Authentication
1. 板块划分
2. 热力图
3. 资金流向?

### desiel

```bash
cargo install diesel_cli --no-default-features --features sqlite
```
* diesel setup
* diesel migration generate create_tableName
* diesel migration run
  如果没有生成schema.rs运行下面一条命令
* diesel print-schema > src/schema.rs
* diesel migration revert
* diesel migration redo run+revert

```bash
  -- create coin_rank_info
  1. diesel migration generate create_coin_rank_info
  2. diesel migration run

DROP TABLE IF EXISTS coin_rank_info;

CREATE TABLE coin_rank_info (
    id BIGINT PRIMARY KEY NOT NULL AUTO_INCREMENT,
    symbol VARCHAR(32) NOT NULL,
    name VARCHAR(64) NOT NULL,
    image TEXT NOT NULL,
    current_price DOUBLE NOT NULL,
    market_cap DOUBLE NOT NULL,
    market_cap_rank INT UNSIGNED NOT NULL,
    fully_diluted_valuation DOUBLE,
    total_volume DOUBLE NOT NULL,
    high_24h DOUBLE NOT NULL,
    low_24h DOUBLE NOT NULL,
    price_change_24h DOUBLE NOT NULL,
    price_change_percentage_24h DOUBLE NOT NULL,
    market_cap_change_24h DOUBLE NOT NULL,
    market_cap_change_percentage_24h DOUBLE NOT NULL,
    circulating_supply DOUBLE NOT NULL,
    total_supply DOUBLE,
    max_supply DOUBLE,
    ath DOUBLE NOT NULL,
    ath_change_percentage DOUBLE NOT NULL,
    ath_date DATETIME(3) NOT NULL,
    atl DOUBLE NOT NULL,
    atl_change_percentage DOUBLE NOT NULL,
    atl_date VARCHAR(64) NOT NULL,
    roi TEXT,
    last_updated DATETIME(3) NOT NULL
);

```
```bash
  -- create coin_data_info
  1. diesel migration generate create_coin_data_info
  2. diesel migration run
    
    DROP TABLE IF EXISTS coin_data_info;
    
    CREATE TABLE coin_data_info (
        id BIGINT PRIMARY KEY NOT NULL AUTO_INCREMENT,
        symbol VARCHAR(32) NOT NULL,
        name VARCHAR(64) NOT NULL,
        web_slug VARCHAR(64),
        asset_platform_id VARCHAR(64),
        platforms JSON NOT NULL,
        detail_platforms JSON NOT NULL,
        block_time_in_minutes BIGINT UNSIGNED,
        hashing_algorithm VARCHAR(64),
        categories JSON NOT NULL,
        preview_listing BOOLEAN NOT NULL DEFAULT FALSE,
        public_notice TEXT,
        additional_notices JSON NOT NULL,
        description JSON NOT NULL,
        links JSON NOT NULL,
        image JSON NOT NULL,
        country_origin VARCHAR(64) NOT NULL,
        genesis_date VARCHAR(32),
        sentiment_votes_up_percentage DOUBLE,
        sentiment_votes_down_percentage DOUBLE,
        watchlist_portfolio_users BIGINT UNSIGNED,
        market_cap_rank INT UNSIGNED,
        status_updates JSON NOT NULL,
        last_updated VARCHAR(64)
    );
```

```bash
  -- create coin_categories
  1. diesel migration generate create_coin_categories
  2. diesel migration run
    
    DROP TABLE IF EXISTS coin_categories;
    
    CREATE TABLE coin_categories (
        id BIGINT PRIMARY KEY NOT NULL AUTO_INCREMENT,
        name VARCHAR(128) NOT NULL,
        market_cap DOUBLE,
        market_cap_change_24h DOUBLE,
        content TEXT,
        top_3_coins_id TEXT,
        top_3_coins TEXT,
        volume_24h DOUBLE,
        updated_at VARCHAR(64)
);

```

-- Your SQL goes here
CREATE TABLE IF NOT EXISTS `market_candle` (
`id` bigint unsigned NOT NULL AUTO_INCREMENT COMMENT '主键ID',
`exchange_type` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL COMMENT '交易所名称',
`period_minutes` bigint NOT NULL COMMENT '数据周期类型分钟换算',
`period_type` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL COMMENT '数据周期类型',
`currency_pair` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL COMMENT '交易对',
`open` decimal(16,8) NOT NULL,
`high` decimal(16,8) DEFAULT NULL,
`low` decimal(16,8) DEFAULT NULL,
`close` decimal(16,8) DEFAULT NULL,
`volume` decimal(20,8) DEFAULT NULL,
`quota_volume` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci DEFAULT NULL,
`ts_time` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
`created_at`   timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
PRIMARY KEY (`id`) USING BTREE
) ENGINE=InnoDB AUTO_INCREMENT=1 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci ROW_FORMAT=DYNAMIC;