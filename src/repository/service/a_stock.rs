use chrono::NaiveDate;
use clickhouse::Row;
use serde::{Deserialize, Serialize};

use crate::repository::akshare::StockAdjustmentType;

/// 移动平均线数据(MA5/MA10/MA20)
#[derive(Debug, Serialize, Deserialize, Row)]
pub struct MALinesFetch {
    /// 数据点日期
    #[serde(with = "clickhouse::serde::chrono::date")]
    pub date: NaiveDate,
    /// 数据点日期对应的MA5值
    pub ma5: Option<f64>,
    /// 数据点日期对应的MA10值
    pub ma10: Option<f64>,
    /// 数据点日期对应的MA20值
    pub ma20: Option<f64>,
}

impl MALinesFetch {
    /// 获取对应`stock_id`从当日开始倒推`limit_days`之中每天对应的5日平均线、
    /// 10日平均线、20日平均线的数据。
    pub async fn fetch_with_limit(
        ch_client: &clickhouse::Client,
        stock_id: &str,
        limit_days: u32,
    ) -> anyhow::Result<Vec<Self>> {
        let data = ch_client
            .query(
                r#"
SELECT
    date,
    ma5,
    ma10,
    ma20
FROM (
    SELECT
        date,
        CASE
            WHEN count() OVER (
                PARTITION BY code
                ORDER BY date ASC
                ROWS BETWEEN 4 PRECEDING AND CURRENT ROW
            ) = 5
            THEN round(avg(argMax(close, ts)) OVER (
                PARTITION BY code
                ORDER BY date ASC
                ROWS BETWEEN 4 PRECEDING AND CURRENT ROW
            ), 2)
            ELSE NULL
        END AS ma5,

        CASE
            WHEN count() OVER (
                PARTITION BY code
                ORDER BY date ASC
                ROWS BETWEEN 9 PRECEDING AND CURRENT ROW
            ) = 10
            THEN round(avg(argMax(close, ts)) OVER (
                PARTITION BY code
                ORDER BY date ASC
                ROWS BETWEEN 9 PRECEDING AND CURRENT ROW
            ), 2)
            ELSE NULL
        END AS ma10,

        CASE
            WHEN count() OVER (
                PARTITION BY code
                ORDER BY date ASC
                ROWS BETWEEN 19 PRECEDING AND CURRENT ROW
            ) = 20
            THEN round(avg(argMax(close, ts)) OVER (
                PARTITION BY code
                ORDER BY date ASC
                ROWS BETWEEN 19 PRECEDING AND CURRENT ROW
            ), 2)
            ELSE NULL
        END AS ma20
    FROM stock_zh_a_hist
    WHERE adj_type = 0
        AND code = ?
    GROUP BY code, date
    ORDER BY date DESC
    LIMIT ?
) AS sub
ORDER BY date ASC
        "#,
            )
            .bind(stock_id)
            .bind(limit_days)
            .fetch_all()
            .await?;
        Ok(data)
    }
}

/// 日频K线数据
#[derive(Debug, Serialize, Deserialize, Row)]
pub struct DailyKlineFetch {
    /// 数据日期
    #[serde(with = "clickhouse::serde::chrono::date")]
    pub date: NaiveDate,
    /// 开盘价
    pub open: f64,
    /// 收盘价
    pub close: f64,
    /// 最高价
    pub high: f64,
    /// 最低价
    pub low: f64,
}

impl DailyKlineFetch {
    pub async fn fetch_with_limit(
        ch_client: &clickhouse::Client,
        adj_type: StockAdjustmentType,
        stock_id: &str,
        limit_days: u32,
    ) -> anyhow::Result<Vec<Self>> {
        let data = ch_client
            .query(
                r#"
SELECT
    date,
    open,
    close,
    high,
    low
FROM (
    SELECT
        date,
        argMax(open, ts) as open,
        argMax(close, ts) as close,
        argMax(high, ts) as high,
        argMax(low, ts) as low
    FROM stock_zh_a_hist
    WHERE adj_type = ?
        AND code = ?
    GROUP BY code, date
    ORDER BY date DESC
    LIMIT ?
) AS sub
ORDER BY date ASC 
        "#,
            )
            .bind(adj_type)
            .bind(stock_id)
            .bind(limit_days)
            .fetch_all()
            .await?;

        Ok(data)
    }
}

/// 日频成交量数据
#[derive(Debug, Serialize, Deserialize, Row)]
pub struct DailyTradingVolumeFetch {
    /// 数据日期
    #[serde(with = "clickhouse::serde::chrono::date")]
    pub date: NaiveDate,
    /// 交易量(手)
    pub trading_volume: f64,
}

impl DailyTradingVolumeFetch {
    pub async fn fetch_with_limit(
        ch_client: &clickhouse::Client,
        adj_type: StockAdjustmentType,
        stock_id: &str,
        limit_days: u32,
    ) -> anyhow::Result<Vec<Self>> {
        let data = ch_client
            .query(
                r#"
SELECT
    date,
    trading_volume
FROM (
    SELECT
        date,
        argMax(trading_volume, ts) as trading_volume
    FROM stock_zh_a_hist
    WHERE adj_type = ?
        AND code = ?
    GROUP BY code, date
    ORDER BY date DESC
    LIMIT ?
) AS sub
ORDER BY date ASC 
    "#,
            )
            .bind(adj_type)
            .bind(stock_id)
            .bind(limit_days)
            .fetch_all()
            .await?;

        Ok(data)
    }
}

/// 日频其它指标数据
#[derive(Debug, Serialize, Deserialize, Row)]
pub struct DailyIndicatorFetch {
    /// 数据日期
    #[serde(with = "clickhouse::serde::chrono::date")]
    pub date: NaiveDate,
    /// 成交额,注意单位(元)
    pub trading_value: f64,
    /// 振幅(%)
    pub amplitude: f64,
    /// 换手率(%)
    pub turnover_rate: f64,
    /// 涨跌幅(%)
    pub change_percent: f64,
    /// 涨跌额,注意单位(元)
    pub change_amount: f64,
}

impl DailyIndicatorFetch {
    pub async fn fetch_with_limit(
        ch_client: &clickhouse::Client,
        adj_type: StockAdjustmentType,
        stock_id: &str,
        limit_days: u32,
    ) -> anyhow::Result<Vec<Self>> {
        let data = ch_client
            .query(
                r#"
SELECT
    date,
    trading_value,
    amplitude,
    turnover_rate,
    change_percentage,
    change_amount
FROM (
    SELECT
        date,
        argMax(trading_value, ts) as trading_value,
        argMax(amplitude, ts) as amplitude,
        argMax(turnover_rate, ts) as turnover_rate,
        argMax(change_percentage, ts) as change_percentage,
        argMax(change_amount, ts) as change_amount
    FROM stock_zh_a_hist
    WHERE adj_type = ?
        AND code = ?
    GROUP BY code, date
    ORDER BY date DESC
    LIMIT ?
) AS sub
ORDER BY date ASC 
        "#,
            )
            .bind(adj_type)
            .bind(stock_id)
            .bind(limit_days)
            .fetch_all()
            .await?;
        Ok(data)
    }
}

#[cfg(test)]
mod test {
    use crate::{repository::akshare::StockAdjustmentType, utils::TEST_CH_CLIENT};

    use super::*;

    #[tokio::test]
    async fn test_fetch_ma_with_limit() {
        let data = MALinesFetch::fetch_with_limit(&TEST_CH_CLIENT, "603777", 90)
            .await
            .unwrap();

        println!("{}", serde_json::to_string_pretty(&data).unwrap())
    }

    #[tokio::test]
    async fn test_fetch_daily_stock_infos() {
        let data = DailyKlineFetch::fetch_with_limit(
            &TEST_CH_CLIENT,
            StockAdjustmentType::Backward,
            "603777",
            30,
        )
        .await
        .unwrap();
        println!("{:?}\n", data);

        let data = DailyTradingVolumeFetch::fetch_with_limit(
            &TEST_CH_CLIENT,
            StockAdjustmentType::Backward,
            "603777",
            30,
        )
        .await
        .unwrap();
        println!("{:?}\n", data);

        let data = DailyIndicatorFetch::fetch_with_limit(
            &TEST_CH_CLIENT,
            StockAdjustmentType::Backward,
            "603777",
            30,
        )
        .await
        .unwrap();
        println!("{:?}\n", data);
    }
}
