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
# 优先使用 tokio::interval封装一个统一任务调度器模块
### dev env
```bash
docker-compose -f docker-compose.yml down -v
docker-compose -f docker-compose.yml up -d
```
### desiel

```bash
cargo install diesel_cli --no-default-features --features mysql
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
        id VARCHAR(64) PRIMARY KEY COMMENT '币种唯一标识符(如"ethereum")',
        symbol VARCHAR(16) NOT NULL COMMENT '币种缩写(如"eth")',
        name VARCHAR(64) NOT NULL COMMENT '币种全称',
        image VARCHAR(255) COMMENT '币种图标URL',
        
        -- 价格数据
        current_price DECIMAL(30, 8) COMMENT '当前价格(USD)',
        price_change_24h DECIMAL(30, 8) COMMENT '24小时价格变化(USD)',
        price_change_percentage_24h DECIMAL(30, 8) COMMENT '24小时价格变化百分比(%)',
        
        -- 市值数据
        market_cap DECIMAL(30, 8) COMMENT '当前市值(USD)',
        market_cap_rank INT UNSIGNED COMMENT '市值排名',
        market_cap_change_24h DECIMAL(30, 8) COMMENT '24小时市值变化(USD)',
        market_cap_change_percentage_24h DECIMAL(30, 8) COMMENT '24小时市值变化百分比(%)',
        fully_diluted_valuation DECIMAL(30, 8) COMMENT '完全稀释估值(USD)',
        
        -- 交易数据
        total_volume DECIMAL(30, 8) COMMENT '24小时交易量(USD)',
        high_24h DECIMAL(30, 8) COMMENT '24小时内最高价(USD)',
        low_24h DECIMAL(30, 8) COMMENT '24小时内最低价(USD)',
        
        -- 供应量数据
        circulating_supply DECIMAL(30, 8) COMMENT '流通供应量',
        total_supply DECIMAL(30, 8) COMMENT '总供应量',
        max_supply DECIMAL(30, 8) COMMENT '最大供应量(可为NULL)',
        
        -- 历史价格数据
        ath DECIMAL(30, 8) COMMENT '历史最高价(USD)',
        ath_change_percentage DECIMAL(30, 8) COMMENT '距历史最高价变化百分比(%)',
        ath_date DATETIME(3) COMMENT '历史最高价达成时间',
        
        atl DECIMAL(30, 8) COMMENT '历史最低价(USD)',
        atl_change_percentage DECIMAL(30, 8) COMMENT '距历史最低价变化百分比(%)',
        atl_date DATETIME(3) COMMENT '历史最低价达成时间',
        
        -- 投资回报率(JSON格式)
        roi JSON COMMENT '投资回报率数据 {
          times: 回报倍数,
          currency: 基准货币,
          percentage: 回报百分比
        }',
        
        -- 时间戳
        last_updated DATETIME(3) COMMENT '数据最后更新时间',
        
        -- 索引配置
        INDEX idx_symbol (symbol) COMMENT '币种缩写索引',
        INDEX idx_market_cap_rank (market_cap_rank) COMMENT '市值排名索引',
        INDEX idx_current_price (current_price) COMMENT '当前价格索引',
        INDEX idx_24h_change (price_change_percentage_24h) COMMENT '24小时变化索引',
        INDEX idx_last_updated (last_updated) COMMENT '更新时间索引',
        
        -- JSON字段索引(MySQL 8.0+)
        INDEX idx_roi_times ((CAST(roi->>'$.times' AS DECIMAL(20, 8)))) COMMENT 'ROI倍数索引',
        INDEX idx_roi_currency ((CAST(roi->>'$.currency' AS CHAR(10)))) COMMENT 'ROI货币索引',
        
        -- 约束条件
        CONSTRAINT chk_positive_price CHECK (current_price >= 0),
        CONSTRAINT chk_valid_market_rank CHECK (market_cap_rank > 0)
    ) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 
      COLLATE=utf8mb4_0900_ai_ci 
      COMMENT='加密货币市场数据信息表';


```
```bash
  -- create coin_data_info
  1. diesel migration generate create_coin_data_info
  2. diesel migration run
    DROP TABLE IF EXISTS coin_data_info;

    CREATE TABLE coin_data_info (
        id VARCHAR(64) PRIMARY KEY COMMENT '币种唯一标识符(如"bitcoin")',
        symbol VARCHAR(16) NOT NULL COMMENT '币种缩写(如"btc")',
        name VARCHAR(64) NOT NULL COMMENT '币种全称',
        web_slug VARCHAR(64) COMMENT '网页URL后缀',
        asset_platform_id VARCHAR(64) COMMENT '所属资产平台ID',
        
        -- 平台信息(JSON格式)
        platforms JSON COMMENT '支持的平台信息',
        detail_platforms JSON COMMENT '平台详细信息',
        
        -- 技术信息
        block_time_in_minutes INT UNSIGNED COMMENT '出块时间(分钟)',
        hashing_algorithm VARCHAR(64) COMMENT '哈希算法(如"SHA-256")',
        
        -- 分类信息(JSON数组)
        categories JSON COMMENT '所属分类数组',
        
        -- 状态信息
        preview_listing BOOLEAN DEFAULT FALSE COMMENT '是否预览上市',
        public_notice TEXT COMMENT '公共通知',
        additional_notices JSON COMMENT '附加通知数组',
        
        -- 描述信息(JSON多语言)
        description JSON COMMENT '多语言描述内容',
        
        -- 链接信息(JSON格式)
        links JSON COMMENT '相关链接 {
          homepage: 主页,
          whitepaper: 白皮书,
          blockchain_site: 区块链浏览器,
          official_forum_url: 官方论坛,
          chat_url: 聊天链接,
          announcement_url: 公告链接,
          twitter_screen_name: Twitter账号,
          facebook_username: Facebook账号,
          subreddit_url: Reddit链接,
          repos_url: 代码仓库
        }',
        
        -- 图片信息(JSON格式)
        image JSON COMMENT '图片URL {
          thumb: 缩略图,
          small: 小图,
          large: 大图
        }',
        
        -- 其他信息
        country_origin VARCHAR(64) COMMENT '起源国家',
        genesis_date DATE COMMENT '创世日期',
        sentiment_votes_up_percentage DECIMAL(5, 2) COMMENT '正面情绪投票百分比',
        sentiment_votes_down_percentage DECIMAL(5, 2) COMMENT '负面情绪投票百分比',
        watchlist_portfolio_users INT UNSIGNED COMMENT '关注用户数',
        market_cap_rank INT UNSIGNED COMMENT '市值排名',
        status_updates JSON COMMENT '状态更新数组',
        last_updated DATETIME(3) COMMENT '最后更新时间',
        
        -- 索引配置
        INDEX idx_symbol (symbol) COMMENT '币种缩写索引',
        INDEX idx_name (name) COMMENT '币种名称索引',
        INDEX idx_market_cap_rank (market_cap_rank) COMMENT '市值排名索引',
        INDEX idx_last_updated (last_updated) COMMENT '更新时间索引',
        INDEX idx_hashing_algorithm (hashing_algorithm) COMMENT '哈希算法索引',
        
        -- JSON字段索引(MySQL 8.0+)
        INDEX idx_web_slug ((CAST(web_slug AS CHAR(64)))) COMMENT '网页后缀索引',
        INDEX idx_twitter ((CAST(links->>'$.twitter_screen_name' AS CHAR(64)))) COMMENT 'Twitter账号索引',
        INDEX idx_facebook ((CAST(links->>'$.facebook_username' AS CHAR(64)))) COMMENT 'Facebook账号索引',
        
    ) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 
      COLLATE=utf8mb4_0900_ai_ci 
      COMMENT='加密货币详细信息表';
    
```

```bash
  -- create coin_categories
  1. diesel migration generate create_coin_categories
  2. diesel migration run
    
    DROP TABLE IF EXISTS coin_categories;

    CREATE TABLE coin_categories (
        id VARCHAR(64) PRIMARY KEY COMMENT '分类ID(如"world-liberty-financial-portfolio")',
        name VARCHAR(128) NOT NULL COMMENT '分类名称',
        market_cap DECIMAL(30, 8) COMMENT '总市值',
        market_cap_change_24h DECIMAL(30, 8) COMMENT '24小时市值变化百分比',
        content TEXT COMMENT '分类描述内容',
        top_3_coins_id JSON NOT NULL COMMENT '前三币种ID数组',
        top_3_coins JSON NOT NULL COMMENT '前三币种图片URL数组',
        volume_24h DECIMAL(30, 8) COMMENT '24小时交易量',
        updated_at DATETIME(3) COMMENT '更新时间(精确到毫秒)',
        
        -- 添加索引
        INDEX idx_id (id),
        INDEX idx_name (name),
        INDEX idx_market_cap (market_cap),
        INDEX idx_updated_at (updated_at),
        
        -- 为JSON数组添加多值索引(MySQL 8.0.17+)
        INDEX idx_top_coins_ids ((CAST(top_3_coins_id->'$[*]' AS CHAR(64) ARRAY))),
        
        -- 检查约束确保数组长度为3
        CONSTRAINT chk_top3_length CHECK (
            JSON_LENGTH(top_3_coins_id) <= 3 AND 
            JSON_LENGTH(top_3_coins) <= 3
        )
    ) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci COMMENT='加密货币分类表';

```