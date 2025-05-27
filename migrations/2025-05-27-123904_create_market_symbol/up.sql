-- Your SQL goes here
CREATE TABLE market_symbol (

                               id VARCHAR(250) NOT NULL PRIMARY KEY COMMENT '唯一ID, 如 binance:BTCUSDT',
                               exchange VARCHAR(50) NOT NULL COMMENT '交易所名称',
                               symbol VARCHAR(50) NOT NULL COMMENT '交易对，例如 BTCUSDT',
                               status VARCHAR(32) NOT NULL COMMENT '交易对状态',
                               base_asset VARCHAR(50) NOT NULL COMMENT '基础资产',
                               base_asset_precision BIGINT UNSIGNED NOT NULL COMMENT '基础资产精度',
                               quote_asset VARCHAR(50) NOT NULL COMMENT '报价资产',
                               quote_precision BIGINT UNSIGNED NOT NULL COMMENT '报价资产精度',
                               order_types JSON  COMMENT '订单类型数组',
                               iceberg_allowed BOOLEAN COMMENT '是否允许冰山订单',
                               is_spot_trading_allowed BOOLEAN COMMENT '是否允许现货交易',
                               is_margin_trading_allowed BOOLEAN COMMENT '是否允许杠杆交易',
                               filters JSON COMMENT '过滤器规则',

                               INDEX idx_symbol_exchange (exchange),
                               UNIQUE KEY uq_exchange_symbol (exchange, symbol)

) ENGINE=InnoDB
    DEFAULT CHARSET = utf8mb4
    COLLATE = utf8mb4_0900_ai_ci
    COMMENT ='交易对信息表';