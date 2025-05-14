use crate::{
    schema::service::serv_aindex::{self},
    utils::limit_or_not,
};

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

impl serv_aindex::IndexOption50EtfQvixKline {
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

impl serv_aindex::IndexOption50EtfQvixMA {
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

impl serv_aindex::StockZhIndexDailyKline {
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

impl serv_aindex::StockZhIndexDailyVolume {
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

impl serv_aindex::StockZhIndexDailyMA {
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

impl serv_aindex::StockZhIndexDailyPagin {
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
