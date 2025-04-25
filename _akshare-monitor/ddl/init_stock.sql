-- A股实时数据表结构
CREATE TABLE IF NOT EXISTS astock_realtime_data
(
    -- 时间维度
    timestamp DateTime64(3, 'Asia/Shanghai') CODEC(Delta, ZSTD(1)),    -- 精确到毫秒的时间戳
    date Date MATERIALIZED toDate(timestamp),         -- 冗余日期字段，用于分区
    time DateTime MATERIALIZED toDateTime(timestamp), -- 精确到秒的时间，用于查询优化
    
    -- 股票标识
    code LowCardinality(String),                      -- 股票代码
    name LowCardinality(String),                      -- 公司名称
    idx Int32,                                        -- 序号
    
    -- 交易状态 - 明确区分停牌等特殊情况
    trading_status LowCardinality(String),            -- 'ACTIVE', 'SUSPENDED', 'LIMIT_UP', 'LIMIT_DOWN', 'NEW_LISTING', 等
    
    -- 价格信息 - 不再需要NULL，对于停牌股票可以使用最后已知价格
    latest_price Float64,                             -- 最新价
    change_percentage Float64,                        -- 涨跌幅
    change_amount Float64,                            -- 涨跌额
    amplitude Float64,                                -- 振幅
    high Float64,                                     -- 最高价
    low Float64,                                      -- 最低价
    today_open Float64,                               -- 今开
    previous_close Float64,                           -- 昨收
    
    -- 交易信息 - 停牌时设为0
    trading_volume Float64,                           -- 成交量
    trading_value Float64,                            -- 成交额
    volume_ratio Float64,                             -- 量比
    turnover_rate Float64,                            -- 换手率
    
    -- 估值信息 - 这些可以保持不变
    pe_ratio_ttm Float64,                             -- 市盈率(动态)
    pb_ratio Float64,                                 -- 市净率
    total_market_value Float64,                       -- 总市值
    circulating_market_value Float64,                 -- 流通市值
    
    -- 动态变化信息 - 停牌时设为0
    change_speed Float64,                             -- 涨速
    five_minute_change Float64,                       -- 5分钟涨跌
    sixty_day_change Float64,                         -- 60日涨跌幅
    ytd_change Float64,                               -- 年初至今涨跌幅
    
    -- 便于查询的衍生字段
    is_suspended UInt8 MATERIALIZED (trading_status = 'SUSPENDED'),  -- 是否停牌，用于快速过滤
    is_active UInt8 MATERIALIZED (trading_status = 'ACTIVE')         -- 是否活跃交易，用于快速过滤
)
ENGINE = MergeTree()
PARTITION BY toYYYYMMDD(date)  -- 按年月分区
ORDER BY (code, timestamp)   -- 复合排序键：先按股票代码，再按时间戳
SETTINGS index_granularity = 8192;

-- 东方财富-沪深京 A 股日频率数据; 历史数据按日频率更新, 当日收盘价在收盘后获取
CREATE TABLE IF NOT EXISTS stock_zh_a_hist
(
    `code` LowCardinality(String),
    `open` Float64,
    `close` Float64,
    `low` Float64,
    `high` Float64,
    `trading_volume` Float64,
    `trading_value` Float64,
    `amplitude` Float64,
    `turnover_rate` Float64,
    `change_percentage` Float64,
    `change_amount` Float64,
    `date` Date,
    -- `ts` DateTime64(3, 'Asia/Shanghai'),
    `adj_type` Enum8('None' = 0, 'Forward' = 1, 'Backward' = 2)
)
ENGINE = ReplacingMergeTree
PARTITION BY toYYYYMM(date)
ORDER BY (date, code);
