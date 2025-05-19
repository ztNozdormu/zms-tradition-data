-- Your SQL goes here
DROP TABLE IF EXISTS coin_categories;

CREATE TABLE coin_categories (
                                 id VARCHAR(64) PRIMARY KEY COMMENT '分类ID(如"world-liberty-financial-portfolio")',
                                 name VARCHAR(128) NOT NULL COMMENT '分类名称',
                                 market_cap DECIMAL(30, 8) COMMENT '总市值',
                                 market_cap_change_24h DECIMAL(10, 4) COMMENT '24小时市值变化百分比',
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
                                     JSON_LENGTH(top_3_coins_id) = 3 AND
                                     JSON_LENGTH(top_3_coins) = 3
                                     )
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci COMMENT='加密货币分类表';