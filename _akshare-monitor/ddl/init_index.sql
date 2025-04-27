-- clickhouse tables，注意结尾一定要带分号

-- 历史行情数据-新浪 描述: 股票指数的历史数据按日频率更新
CREATE TABLE IF NOT EXISTS stock_zh_index_daily
(
    `code` LowCardinality(String), -- 指数代码
    `open` Float64, -- 开盘
    `close` Float64, -- 收盘
    `high` Float64, -- 最高
    `low` Float64, -- 最低
    `volume` Float64, -- 交易量
    `date` Date, -- 数据产生日期
    `ts` DateTime64(3, 'Asia/Shanghai') -- 数据收集时间
)
ENGINE = ReplacingMergeTree(ts)
ORDER BY (date, code);

-- 50ETF 期权波动率指数 QVIX; 又称中国版的恐慌指数
CREATE TABLE IF NOT EXISTS index_option_50etf_qvix
(
    `open` Float64, -- 开盘
    `close` Float64, -- 收盘
    `high` Float64, -- 最高
    `low` Float64, -- 最低
    `date` Date, -- 数据产生日期
    `ts` DateTime64(3, 'Asia/Shanghai') -- 数据收集时间
)
ENGINE = ReplacingMergeTree(ts)
ORDER BY date;