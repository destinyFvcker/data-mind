use chrono::NaiveDate;
use clickhouse::Row;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// 50ETF 期权波动率指数 QVIX; 又称中国版的恐慌指数 K线数据
#[derive(Debug, Deserialize, Serialize, Row, ToSchema)]
pub struct IndexOption50EtfQvix {
    /// 开盘价
    pub open: f64,
    /// 收盘价
    pub close: f64,
    /// 最高价
    pub high: f64,
    /// 最低价
    pub low: f64,
    /// 数据日期，格式为YYY-MM-DD
    pub date: String,
}

impl IndexOption50EtfQvix {
    /// 从clickhouse之中获取limit条数据，假如传入的limit小于0，则获取全量数据
    pub async fn fetch_with_limit(
        ch_client: &clickhouse::Client,
        limit: i32,
    ) -> anyhow::Result<Vec<String>> {
        let sql = if limit < 0 {
            r#"
SELECT
    open,
    close,
    high,
    low,
    date
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
) AS sub
ORDER BY date ASC       
            "#
        } else {
            r#"
SELECT
    open,
    close,
    high,
    low,
    date
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
ORDER BY date ASC
            "#
        };

        let mut query_builder = ch_client.query(sql);
        query_builder = if limit < 0 {
            query_builder
        } else {
            query_builder.bind(limit)
        };
        let data = query_builder.fetch_all().await?;

        Ok(data)
    }
}

pub struct StockZhIndexDaily {
    pub code: String,
    pub open: String,
    pub close: String,
    pub high: String,
    pub low: String,
    pub date: NaiveDate,
}
