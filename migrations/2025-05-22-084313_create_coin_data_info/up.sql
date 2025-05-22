-- Your SQL goes here
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

    -- 约束条件
                                CONSTRAINT chk_sentiment_percentage CHECK (
                                    sentiment_votes_up_percentage BETWEEN 0 AND 100 AND
                                    sentiment_votes_down_percentage BETWEEN 0 AND 100
                                    ),
                                CONSTRAINT chk_sentiment_sum CHECK (
                                    sentiment_votes_up_percentage + sentiment_votes_down_percentage <= 100
                                    )
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4
  COLLATE=utf8mb4_0900_ai_ci
    COMMENT='加密货币详细信息表';