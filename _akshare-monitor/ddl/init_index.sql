-- 历史行情数据-新浪 描述: 股票指数的历史数据按日频率更新
CREATE TABLE IF NOT EXISTS stock_zh_index_daily
(
    `code` LowCardinality(String),
    `open` Float64,
    `close` Float64,
    `high` Float64,
    `low` Float64,
    `volume` Float64,
    `date` Date,
    `ts` DateTime64(3, 'Asia/Shanghai')
)
ENGINE = ReplacingMergeTree
ORDER BY (date, code);