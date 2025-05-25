-- Your SQL goes here
CREATE TABLE market_kline (
                              exchange VARCHAR(64) NOT NULL COMMENT '交易所名称，例如 binance',
                              symbol VARCHAR(64) NOT NULL COMMENT '交易对名称，例如 BTCUSDT',
                              time_frame VARCHAR(16) NOT NULL COMMENT 'K线周期，例如 1m、5m、1h',

                              open_time BIGINT NOT NULL COMMENT 'K线开始时间戳（毫秒）',
                              open DOUBLE NOT NULL COMMENT '开盘价',
                              high DOUBLE NOT NULL COMMENT '最高价',
                              low DOUBLE NOT NULL COMMENT '最低价',
                              close DOUBLE NOT NULL COMMENT '收盘价',
                              volume DOUBLE NOT NULL COMMENT '成交量（基础资产）',
                              close_time BIGINT NOT NULL COMMENT 'K线结束时间戳（毫秒）',

                              quote_asset_volume DOUBLE NOT NULL COMMENT '成交量（计价资产）',
                              number_of_trades BIGINT UNSIGNED NOT NULL COMMENT '成交笔数',
                              taker_buy_base_asset_volume DOUBLE NOT NULL COMMENT '买方成交量（基础资产）',
                              taker_buy_quote_asset_volume DOUBLE NOT NULL COMMENT '买方成交量（计价资产）',

                              PRIMARY KEY (exchange, symbol, time_frame, close_time)

) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci COMMENT='市场K线数据表';

CREATE INDEX idx_exchange_symbol_time_frame ON market_kline (exchange, symbol, time_frame);