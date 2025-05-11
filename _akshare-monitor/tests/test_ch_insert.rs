use std::{sync::LazyLock, time::Duration};

use backoff::ExponentialBackoff;
use chrono::Utc;
use data_mind::{
    repository::akshare::{StockAdjustmentType, StockZhAHistInsert},
    schema::akshare::AkStockZhAHist,
};
use reqwest::{Client, ClientBuilder};

static TEST_HTTP_CLIENT: LazyLock<Client> = LazyLock::new(|| {
    ClientBuilder::new()
        .connect_timeout(Duration::from_secs(5))
        .timeout(Duration::from_secs(20))
        .build()
        .unwrap()
});

static TEST_CH_CLIENT: LazyLock<clickhouse::Client> = LazyLock::new(|| {
    clickhouse::Client::default()
        .with_url("http://127.0.0.1:8123")
        .with_user("default")
        .with_password("defaultpassword")
        .with_database("akshare")
});

const AK_TOOLS_BASE_URL: &'static str = "http://127.0.0.1:8080/api/public";

fn with_base_url(path: &str) -> String {
    format!("{}{}", AK_TOOLS_BASE_URL, path)
}

async fn test_retry_api_data() -> anyhow::Result<Vec<AkStockZhAHist>> {
    async fn test_get_api_data() -> anyhow::Result<Vec<AkStockZhAHist>> {
        let res = TEST_HTTP_CLIENT
            .get(with_base_url("/stock_zh_a_hist"))
            .query(&[
                ("symbol", "000088"),
                ("period", "daily"),
                ("start_date", "20250421"),
                ("end_date", "20250425"),
                ("adjust", ""),
            ])
            .send()
            .await?
            .text()
            .await?;

        let value: Vec<AkStockZhAHist> = serde_json::from_str(&res).unwrap();
        Ok(value)
    }

    let backoff = ExponentialBackoff {
        initial_interval: Duration::from_millis(100), // 第一次失败后100ms重试
        randomization_factor: 0.5,                    // 加入一定的抖动，避免雪崩
        multiplier: 2.0,                              // 每次间隔翻倍
        max_interval: Duration::from_secs(1),         // 单次最大间隔1秒
        max_elapsed_time: Some(Duration::from_secs(3)), // 总最大重试时间3秒
        ..Default::default()
    };

    Ok(backoff::future::retry(backoff, || async { Ok(test_get_api_data().await?) }).await?)
}

#[tokio::test]
async fn test_insert_stock_zh_a_hist() {
    let api_data = test_retry_api_data().await.unwrap();
    let adj_type = StockAdjustmentType::None;

    let now = Utc::now();
    let rows = api_data
        .into_iter()
        .map(|value| StockZhAHistInsert::from_with_type(value, adj_type, now))
        .collect::<Vec<StockZhAHistInsert>>();

    let mut inserter = TEST_CH_CLIENT.inserter("stock_zh_a_hist").unwrap();
    for row in rows {
        inserter.write(&row).unwrap();
    }
    inserter.end().await.unwrap();
}
