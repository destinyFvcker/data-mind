use chrono::NaiveDate;
use clickhouse::Row;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::utils::limit_or_not;

/// 判断一个指数代码是否存在
pub async fn is_index_code_exists(
    ch_client: &clickhouse::Client,
    index_code: &str,
) -> anyhow::Result<bool> {
    Ok(ch_client
        .query("SELECT exists(SELECT 1 FROM stock_zh_index_daily WHERE code = ?) AS code_exists")
        .bind(index_code)
        .fetch_one()
        .await?)
}

/// 50ETF 期权波动率指数 QVIX; 又称中国版的恐慌指数 K线数据
#[derive(Debug, Deserialize, Serialize, Row, ToSchema)]
pub struct IndexOption50EtfQvixKlineFetch {
    /// 开盘价
    pub open: f64,
    /// 收盘价
    pub close: f64,
    /// 最高价
    pub high: f64,
    /// 最低价
    pub low: f64,
    /// 数据日期，格式为YYY-MM-DD
    #[serde(deserialize_with = "clickhouse::serde::chrono::date::deserialize")]
    pub date: NaiveDate,
}

impl IndexOption50EtfQvixKlineFetch {
    /// 从clickhouse之中获取limit条数据，假如传入的limit小于0，则获取全量数据
    pub async fn fetch_with_limit(
        ch_client: &clickhouse::Client,
        limit_days: i32,
    ) -> anyhow::Result<Vec<Self>> {
        let sql = r#"
SELECT open, close, high, low, date
FROM
(
    SELECT
        argMax(open, ts) AS open,
        argMax(close, ts) AS close,
        argMax(high, ts) AS high,
        argMax(low, ts) AS low,
        date
    FROM index_option_50etf_qvix
    GROUP BY date
    ORDER BY date DESC
    LIMIT ?
) AS sub
ORDER BY date ASC"#;

        let limit = limit_or_not(limit_days);
        let data = ch_client.query(sql).bind(limit).fetch_all().await?;
        Ok(data)
    }
}

// ---------------------------------------------------------------------------------

/// 50ETF 期权波动率指数 QVIX; 各种移动平均线数据
#[derive(Debug, Serialize, Deserialize, Row, ToSchema)]
pub struct IndexOption50EtfQvixMAFetch {
    /// 数据日期，格式为YYYY-DD-MM
    #[serde(deserialize_with = "clickhouse::serde::chrono::date::deserialize")]
    pub date: NaiveDate,
    /// 对应数据日期的5日平均线数据
    pub ma5: Option<f64>,
    /// 对应数据日期的10日平均线数据
    pub ma10: Option<f64>,
    /// 对应数据日期的20日平均线数据
    pub ma20: Option<f64>,
}

impl IndexOption50EtfQvixMAFetch {
    pub async fn fetch_with_limit(
        ch_client: &clickhouse::Client,
        limit_days: i32,
    ) -> anyhow::Result<Vec<Self>> {
        let sql = r#"
SELECT date, ma5, ma10, ma20
FROM
(
    SELECT
        date,
        multiIf(count() OVER (ORDER BY date ASC ROWS BETWEEN 4 PRECEDING AND CURRENT ROW) = 5, round(avg(argMax(close, ts)) OVER (ORDER BY date ASC ROWS BETWEEN 4 PRECEDING AND CURRENT ROW), 2), NULL) AS ma5,
        multiIf(count() OVER (ORDER BY date ASC ROWS BETWEEN 9 PRECEDING AND CURRENT ROW) = 10, round(avg(argMax(close, ts)) OVER (ORDER BY date ASC ROWS BETWEEN 9 PRECEDING AND CURRENT ROW), 2), NULL) AS ma10,
        multiIf(count() OVER (ORDER BY date ASC ROWS BETWEEN 19 PRECEDING AND CURRENT ROW) = 20, round(avg(argMax(close, ts)) OVER (ORDER BY date ASC ROWS BETWEEN 19 PRECEDING AND CURRENT ROW), 2), NULL) AS ma20
    FROM index_option_50etf_qvix
    GROUP BY date
    ORDER BY date DESC
    LIMIT ?
) AS sub
ORDER BY date ASC"#;

        let limit = limit_or_not(limit_days);
        let data = ch_client.query(sql).bind(limit).fetch_all().await?;
        Ok(data)
    }
}

// ---------------------------------------------------------------------------------

#[derive(Debug, Deserialize, Serialize, Row, ToSchema)]
pub struct StockZhIndexDailyKlineFetch {
    /// 开盘价(元)
    pub open: f64,
    /// 收盘价(元)
    pub close: f64,
    /// 最高价(元)
    pub high: f64,
    /// 最低价(元)
    pub low: f64,
    /// 交易量(手)
    pub volume: f64,
    /// 数据日期，格式为YYYY-MM-DD
    #[serde(deserialize_with = "clickhouse::serde::chrono::date::deserialize")]
    pub date: NaiveDate,
}

impl StockZhIndexDailyKlineFetch {
    /// 通过
    pub async fn fetch_with_limit(
        ch_client: &clickhouse::Client,
        index_code: &str,
        limit_days: i32,
    ) -> anyhow::Result<Vec<Self>> {
        let sql = r#"
SELECT open, close, high, low, volume, date
FROM (
    SELECT
        argMax(open, ts) as open,
        argMax(close, ts) as close,
        argMax(high, ts) as high,
        argMax(low, ts) as low,
        argMax(volume, ts) as volume,
        date
    FROM stock_zh_index_daily 
    WHERE code = ?
    GROUP BY date
    ORDER BY date DESC
    LIMIT ?
) AS sub
ORDER BY date ASC"#;

        let limit = limit_or_not(limit_days);
        let data = ch_client
            .query(sql)
            .bind(index_code)
            .bind(limit)
            .fetch_all()
            .await?;
        Ok(data)
    }
}

// ---------------------------------------------------------------------------------

#[derive(Debug, Deserialize, Serialize, Row, ToSchema)]
pub struct StockZhIndexDailyVolumeFetch {
    /// 交易日期
    #[serde(deserialize_with = "clickhouse::serde::chrono::date::deserialize")]
    pub date: NaiveDate,
    /// 交易量
    pub volume: f64,
}

impl StockZhIndexDailyVolumeFetch {
    pub async fn fetch_with_limit(
        ch_client: &clickhouse::Client,
        index_code: &str,
        limit_days: i32,
    ) -> anyhow::Result<Vec<Self>> {
        let sql = r#"
SELECT
    date,
    volume
FROM (
    SELECT
        date,
        argMax(volume, ts) as volume
    FROM stock_zh_index_daily
    WHERE code = ?
    GROUP BY code, date
    ORDER BY date DESC
    LIMIT ?
) AS sub
ORDER BY date ASC
"#;

        let limit = limit_or_not(limit_days);
        let data = ch_client
            .query(sql)
            .bind(index_code)
            .bind(limit)
            .fetch_all()
            .await?;
        Ok(data)
    }
}

// ---------------------------------------------------------------------------------

#[derive(Debug, Serialize, Deserialize, Row, ToSchema)]
pub struct StockZhIndexDailyMAFetch {
    /// 数据日期，格式为YYYY-MM-DD
    #[serde(deserialize_with = "clickhouse::serde::chrono::date::deserialize")]
    pub date: NaiveDate,
    /// 对应(指数代码, 数据日期)的5日移动平均线数据
    pub ma5: Option<f64>,
    /// 对应(指数代码, 数据日期)的10日移动平均线数据
    pub ma10: Option<f64>,
    /// 对应(指数代码, 数据日期)的20日移动平均线数据
    pub ma20: Option<f64>,
}

impl StockZhIndexDailyMAFetch {
    pub async fn fetch_with_limit(
        ch_client: &clickhouse::Client,
        index_code: &str,
        limit_days: i32,
    ) -> anyhow::Result<Vec<Self>> {
        let sql = r#"
SELECT date, ma5, ma10, ma20
FROM
(
    SELECT
        date,
        multiIf(count() OVER (PARTITION BY code ORDER BY date ASC ROWS BETWEEN 4 PRECEDING AND CURRENT ROW) = 5, round(avg(argMax(close, ts)) OVER (PARTITION BY code ORDER BY date ASC ROWS BETWEEN 4 PRECEDING AND CURRENT ROW), 2), NULL) AS ma5,
        multiIf(count() OVER (PARTITION BY code ORDER BY date ASC ROWS BETWEEN 9 PRECEDING AND CURRENT ROW) = 10, round(avg(argMax(close, ts)) OVER (PARTITION BY code ORDER BY date ASC ROWS BETWEEN 9 PRECEDING AND CURRENT ROW), 2), NULL) AS ma10,
        multiIf(count() OVER (PARTITION BY code ORDER BY date ASC ROWS BETWEEN 19 PRECEDING AND CURRENT ROW) = 20, round(avg(argMax(close, ts)) OVER (PARTITION BY code ORDER BY date ASC ROWS BETWEEN 19 PRECEDING AND CURRENT ROW), 2), NULL) AS ma20
    FROM stock_zh_index_daily
    WHERE code = ?
    GROUP BY
        code,
        date
    ORDER BY date DESC
    LIMIT ?
) AS sub
ORDER BY date ASC
"#;

        let limit = limit_or_not(limit_days);
        let data = ch_client
            .query(sql)
            .bind(index_code)
            .bind(limit)
            .fetch_all()
            .await?;
        Ok(data)
    }
}

// ---------------------------------------------------------------------------------

#[derive(Debug, Deserialize, Serialize, Row, ToSchema)]
pub struct StockZhIndexDailyPaginFetch {
    /// 指数代码
    pub code: String,
    /// 最新数据时间，格式为YYYY-MM-DD
    #[serde(deserialize_with = "clickhouse::serde::chrono::date::deserialize")]
    pub date: NaiveDate,
    /// 开盘价
    pub open: f64,
    /// 收盘价
    pub close: f64,
    /// 最高价
    pub high: f64,
    /// 最低价
    pub low: f64,
    // 最新的一组交易量(5日)
    // pub latest_volmes: Vec<f64>,
    /// 振幅(%)
    pub amplitude: f64,
    /// 涨跌幅(%)
    pub change_percentage: f64,
    /// 涨跌额(%)
    pub change_amount: f64,
}

impl StockZhIndexDailyPaginFetch {
    /// 分页获取所有指数最新一个交易日的数据
    pub async fn fetch_paginate(
        ch_client: &clickhouse::Client,
        page_size: u32,
        page_index: u32,
    ) -> anyhow::Result<Vec<Self>> {
        let sql = r#"
SELECT
    code,
    max(date) AS latest_date,
    argMax(open, date) AS open,
    argMax(close, date) AS close,
    argMax(high, date) AS high,
    argMax(low, date) AS low,
    round(((high - low) / close) * 100, 2) AS amplitude,
    round(((close - open) / open) * 100, 2) AS change_percentage,
    round(close - open, 2) AS change_amount
FROM stock_zh_index_daily
GROUP BY code
ORDER BY code ASC
LIMIT ?, ? 
        "#;

        let offset = page_size * (page_index - 1);
        let data = ch_client
            .query(sql)
            .bind(offset)
            .bind(page_size)
            .fetch_all()
            .await?;
        Ok(data)
    }
}

// ---------------------------------------------------------------------------------

#[cfg(test)]
mod test {
    use super::*;

    use crate::utils::TEST_CH_CLIENT;

    #[tokio::test]
    async fn test_is_exist() {
        assert!(is_index_code_exists(&TEST_CH_CLIENT, "sz399282")
            .await
            .unwrap());
        assert!(!is_index_code_exists(&TEST_CH_CLIENT, "399282")
            .await
            .unwrap());
    }
}
