use crate::{
    repository::akshare::StockAdjustmentType,
    schema::service::serv_astock::{self, StockDailyTradingVolume},
};

/// 判断一个指定的stock code是否存在
pub async fn is_stock_code_exists(
    ch_client: &clickhouse::Client,
    stock_code: &str,
) -> anyhow::Result<bool> {
    Ok(ch_client
        .query("SELECT exists(SELECT 1 FROM stock_zh_a_hist WHERE code = ?) AS code_exists")
        .bind(stock_code)
        .fetch_one()
        .await?)
}

impl serv_astock::StockMALines {
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

impl serv_astock::StockDailyKline {
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

impl StockDailyTradingVolume {
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

impl serv_astock::StockDailyIndicator {
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
        let data = serv_astock::StockMALines::fetch_with_limit(&TEST_CH_CLIENT, "603777", 90)
            .await
            .unwrap();

        println!("{}", serde_json::to_string_pretty(&data).unwrap())
    }

    #[tokio::test]
    async fn test_fetch_daily_stock_infos() {
        let data = serv_astock::StockDailyKline::fetch_with_limit(
            &TEST_CH_CLIENT,
            StockAdjustmentType::Backward,
            "603777",
            30,
        )
        .await
        .unwrap();
        println!("{:?}\n", data);

        let data = serv_astock::StockDailyTradingVolume::fetch_with_limit(
            &TEST_CH_CLIENT,
            StockAdjustmentType::Backward,
            "603777",
            30,
        )
        .await
        .unwrap();
        println!("{:?}\n", data);

        let data = serv_astock::StockDailyIndicator::fetch_with_limit(
            &TEST_CH_CLIENT,
            StockAdjustmentType::Backward,
            "603777",
            30,
        )
        .await
        .unwrap();
        println!("{:?}\n", data);
    }

    #[tokio::test]
    async fn test_is_exist() {
        assert!(is_stock_code_exists(&TEST_CH_CLIENT, "000063")
            .await
            .unwrap());
        assert!(!is_stock_code_exists(&TEST_CH_CLIENT, "0w-1").await.unwrap());
    }
}
