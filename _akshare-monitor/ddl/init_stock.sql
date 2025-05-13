-- clickhouse tables，注意后面一定要带分号

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
    -- 复权方式枚举，choice of ['不复权', '复权前', '复权后']
    `adj_type` Enum8('None' = 0, 'Forward' = 1, 'Backward' = 2),
    -- 股票代码
    `code` LowCardinality(String),
    -- 开盘价
    `open` Float64,
    -- 收盘价
    `close` Float64,
    -- 最低价
    `low` Float64,
    -- 最高价
    `high` Float64,
    -- 成交量,注意单位(手)
    `trading_volume` Float64,
    -- 成交额,注意单位(元)
    `trading_value` Float64,
    -- 振幅(%)
    `amplitude` Float64,
    -- 换手率(%)
    `turnover_rate` Float64,
    -- 涨跌幅(%)
    `change_percentage` Float64,
    -- 涨跌额,注意单位(元)
    `change_amount` Float64,
    -- 数据产生日期
    `date` Date,
    -- 数据收集时间戳，毫秒等级
    `ts` DateTime64(3, 'Asia/Shanghai')
)
ENGINE = ReplacingMergeTree
ORDER BY (code, date, adj_type);

-- 东方财富网-数据中心-资金流向-沪深港通资金流向-沪深港通历史数据
CREATE TABLE IF NOT EXISTS stock_hsgt_hist_em
(
    `flow_dir` Enum8('Northbound' = 0, 'Southbound' = 1), -- 资金流动方向(南向/北向)
    `buy_amount` Float64, -- 买入成交额，单位：亿元
    `sell_amount` Float64, -- 卖出成交额，单位：亿元
    `historical_net_buy_amount` Float64, -- 历史累计净买额，单位：万亿元
    `daily_balance` Float64, -- 当日余额，单位：亿元
    `daily_net_buy_amount` Float64, -- 当日成交净买额，单位：亿元
    `daily_inflow` Float64, -- 当日资金流入，单位：亿元
    `holding_market_value` Float64, -- 持股市值，单位：元
    `hs300_index` Float64, -- 沪深300指数点位
    `hs300_change_percent` Float64, -- 沪深300指数涨跌幅，单位：%
    `leading_stock_name` String, -- 领涨股名称
    `leading_stock_code` String, -- 领涨股代码，例如 "600198.SH"
    `leading_stock_change_percent` Float64, -- 领涨股涨跌幅，单位：%
    `date` Date, -- 数据产生日期
    `ts` DateTime64(3, 'Asia/Shanghai') -- 数据拉取日期
)
ENGINE = ReplacingMergeTree(ts)
ORDER BY (date, flow_dir);

-- 东方财富网-行情中心-涨停板行情-涨停股池
create table if not EXISTS stock_zt_pool_em
(
    `code` String, -- 股票代码
    `name` String, -- 股票名称
    `lockup_funds` Float64, -- 封板所需资金（单位：元）
    `serial_number` UInt32, -- 序号（基本无意义）
    `total_market_value` Float64, -- 总市值（单位：元）
    `turnover` Float64, -- 成交额（单位：元）
    `industry` String, -- 所属行业
    `turnover_rate` Float64, -- 换手率（百分比）
    `last_lockup_time` String, -- 最后封板时间（格式：HHMMSS）
    `latest_price` Float64, -- 最新价格
    `circulating_market_value` Float64, -- 流通市值（单位：元）
    `limit_up_statistics` String, -- 涨停统计（例如 "1/1"）
    `price_change_percentage` Float64, -- 涨跌幅（百分比）
    `failed_lockup_count` UInt32, -- 炸板次数（封板失败次数）
    `consecutive_limit_ups` UInt32, -- 连续涨停板数量
    `first_lockup_time` String, -- 首次封板时间（格式：HHMMSS）
    `date` Date, -- 数据生成时间
    `ts` DateTime64(3, 'Asia/Shanghai') -- 数据收集时间
)
ENGINE = ReplacingMergeTree(ts)
order by (date, code);

-- 目标地址: https://cxdata.caixin.com/pc/  
-- 描述: 财新网-财新数据通-内容精选  
-- 限量: 返回所有历史新闻数据
CREATE TABLE IF NOT EXISTS stock_news_main_cx
(
    -- 新闻被精选、推送或整理到内容库的时间。格式通常是 yyyy-MM-dd HH:mm。
    `interval_time` String,
    -- 新闻的正式发布时间，即新闻内容原文在财新网等发布的时间。格式是完整的 yyyy-MM-dd HH:mm:ss.sss。
    `pub_time` DateTime64(3, 'Asia/Shanghai'),
    -- 新闻的摘要内容，对新闻正文的简要概括，便于快速了解新闻主旨。
    `summary` String,
    -- 新闻的主题标签，通常由几个关键词组成，归纳了该新闻的主要话题或核心内容。
    `tag` String,
    -- 新闻的详情链接，点击可以跳转到财新网对应的新闻完整正文页面。
    `url` String,
    -- 数据收集时间，用于排除重复数据(取最新)
    `ts` DateTime64(3, 'Asia/Shanghai')
)
ENGINE = ReplacingMergeTree(ts)
ORDER BY (pub_time, summary, tag, url);

-- 技术指标-连续上涨  
-- 接口：stock_rank_lxsz_ths  
-- 目标地址：https://data.10jqka.com.cn/rank/lxsz/  
-- 描述：同花顺-数据中心-技术选股-连续上涨  
-- 限量：单次返回所有数据
-- 加入clickhouse记录原因：单次拉取数据量过大，响应缓慢
CREATE TABLE IF NOT EXISTS stock_rank_lxsz_ths
(
    `index` Int32 COMMENT '序号',
    `industry` String COMMENT '所属行业',
    `closing_price` Float64 COMMENT '收盘价(元)',
    `lowest_price` Float64 COMMENT '最低价(元)',
    `highest_price` Float64 COMMENT '最高价(元)',
    `cumulative_turnover_rate` Float64 COMMENT '累计换手率，单位：百分比 (%)',
    `stock_code` LowCardinality(String) COMMENT '股票代码',
    `stock_name` LowCardinality(String) COMMENT '股票简称',
    `consecutive_rising_days` Int32 COMMENT '连续上涨天数',
    `consecutive_change_percentage` Float64 COMMENT '连续涨跌幅，单位：百分比 (%)',
)
ENGINE = MergeTree
ORDER BY (stock_code)
