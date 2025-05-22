-- Your SQL goes here
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