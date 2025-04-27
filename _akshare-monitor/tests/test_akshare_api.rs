use backoff::ExponentialBackoff;
use data_mind::schema;
use reqwest::{Client, ClientBuilder};
use serde_json::Value;
use std::{env, fs::File, io::Write, sync::LazyLock, time::Duration};

static TEST_HTTP_CLIENT: LazyLock<Client> = LazyLock::new(|| {
    ClientBuilder::new()
        .connect_timeout(Duration::from_secs(5))
        .timeout(Duration::from_secs(20))
        .build()
        .unwrap()
});

const AK_TOOLS_BASE_URL: &'static str = "http://127.0.0.1:8080/api/public";

fn with_base_url(path: &str) -> String {
    format!("{}{}", AK_TOOLS_BASE_URL, path)
}

#[tokio::test]
async fn test_stock_zh_a_hist() {
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
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    let value: Vec<Value> = serde_json::from_str(&res).unwrap();

    println!("values len = {}", value.len());
    let mut file = File::create("../tmp/历史行情数据-东财.json").unwrap();
    file.write_all(serde_json::to_string_pretty(&value).unwrap().as_bytes())
        .unwrap();
}

async fn test_retry_api_data() -> anyhow::Result<Vec<Value>> {
    async fn test_get_api_data() -> anyhow::Result<Vec<Value>> {
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

        let value: Vec<Value> = serde_json::from_str(&res).unwrap();
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
async fn test_retry1() {
    let result = test_retry_api_data().await;

    match result {
        Ok(value) => println!("value = {:#?}", value),
        Err(err) => println!("err = {}", err),
    }
}

#[tokio::test]
async fn test_stock_zh_index_spot_em() {
    println!("current dir = {:?}", env::current_dir());
    let params = [
        "沪深重要指数",
        "上证系列指数",
        "深证系列指数",
        "指数成份",
        "中证系列指数",
    ];

    for param in params {
        let res = TEST_HTTP_CLIENT
            .get(with_base_url("/stock_zh_index_spot_em"))
            .query(&[("symbol", param)])
            .send()
            .await
            .unwrap()
            .text()
            .await
            .unwrap();

        let mut file = File::create(format!("../tmp/{}.json", param)).unwrap();
        file.write_all(res.as_bytes()).unwrap();
    }
}

#[tokio::test]
async fn test_stock_zh_index_spot_sina() {
    let res = TEST_HTTP_CLIENT
        .get(with_base_url("/stock_zh_index_spot_sina"))
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    let values: Vec<schema::akshare::StockZhIndexSpotSina> = serde_json::from_str(&res).unwrap();
    println!("res len = {}", values.len());

    let mut file = File::create("../tmp/实时行情数据-新浪.json").unwrap();
    file.write_all(res.as_bytes()).unwrap();
}

#[tokio::test]
async fn test_stock_zh_index_daily() {
    let res = TEST_HTTP_CLIENT
        .get(with_base_url("/stock_zh_index_daily"))
        .query(&[("symbol", "sz399552")])
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    let values: Vec<Value> = serde_json::from_str(&res).unwrap();
    println!("res len = {}", values.len());

    let mut file = File::create("../tmp/历史行情数据-新浪.json").unwrap();
    file.write_all(serde_json::to_string_pretty(&values).unwrap().as_bytes())
        .unwrap();
}

#[tokio::test]
async fn test_index_option_50etf_qvix() {
    let values: Vec<Value> = TEST_HTTP_CLIENT
        .get(with_base_url("/index_option_50etf_qvix"))
        .send()
        .await
        .unwrap()
        .error_for_status()
        .unwrap()
        .json()
        .await
        .unwrap();

    println!("length of res = {}", values.len());

    let mut file = File::create("../tmp/50ETF 期权波动率指数.json").unwrap();
    file.write_all(serde_json::to_string_pretty(&values).unwrap().as_bytes())
        .unwrap();
}

#[tokio::test]
async fn test_stock_hsgt_fund_flow_summary_em() {
    let values: Vec<Value> = TEST_HTTP_CLIENT
        .get(with_base_url("/stock_hsgt_fund_flow_summary_em"))
        .send()
        .await
        .unwrap()
        .error_for_status()
        .unwrap()
        .json()
        .await
        .unwrap();

    println!("length of res = {}", values.len());

    let mut file = File::create(format!("../tmp/沪深港通资金流向.json")).unwrap();
    file.write_all(serde_json::to_string_pretty(&values).unwrap().as_bytes())
        .unwrap();
}

#[tokio::test]
async fn test_stock_hsgt_hist_em() {
    let symbols = [
        "北向资金",
        // "沪股通",
        // "深股通",
        "南向资金",
        // "港股通沪",
        // "港股通深",
    ];

    for symbol in symbols {
        let values: Vec<Value> = TEST_HTTP_CLIENT
            .get(with_base_url("/stock_hsgt_hist_em"))
            .query(&[("symbol", symbol)])
            .send()
            .await
            .unwrap()
            .error_for_status()
            .unwrap()
            .json()
            .await
            .unwrap();

        // for value in values {
        //     println!("{:#?}", value);
        //     let data = serde_json::from_value::<schema::akshare::StockHsgtHistEm>(value);
        //     match data {
        //         Ok(data) => {}
        //         Err(err) => {
        //             println!("err = {:?}", err);
        //             panic!()
        //         }
        //     }
        // }

        println!("length of res = {}", values.len());

        let mut file = File::create(format!("../tmp/{}.json", symbol)).unwrap();
        file.write_all(serde_json::to_string_pretty(&values).unwrap().as_bytes())
            .unwrap();
    }
}

#[tokio::test]
async fn test_stock_zt_pool_em() {
    let res: Vec<Value> = TEST_HTTP_CLIENT
        .get(with_base_url("/stock_zt_pool_em"))
        .query(&[("date", "20250411")])
        .send()
        .await
        .unwrap()
        .error_for_status()
        .unwrap()
        .json()
        .await
        .unwrap();

    println!("len of res = {}", res.len());

    let mut file = File::create("../tmp/涨停股池.json").unwrap();
    file.write_all(serde_json::to_string_pretty(&res).unwrap().as_bytes())
        .unwrap();
}

#[tokio::test]
async fn test_stock_sse_summary() {
    let res: Vec<Value> = TEST_HTTP_CLIENT
        .get(with_base_url("/stock_sse_deal_daily"))
        .query(&[("date", "20250221")])
        .send()
        .await
        .unwrap()
        .error_for_status()
        .unwrap()
        .json()
        .await
        .unwrap();

    println!("len of res = {}", res.len());

    let mut file = File::create("../tmp/市场总貌-上海证券交易所.json").unwrap();
    file.write_all(serde_json::to_string_pretty(&res).unwrap().as_bytes())
        .unwrap();
}

#[tokio::test]
async fn test_stock_szse_summary() {
    let res: Vec<Value> = TEST_HTTP_CLIENT
        .get(with_base_url("/stock_szse_summary"))
        .query(&[("date", "20250221")])
        .send()
        .await
        .unwrap()
        .error_for_status()
        .unwrap()
        .json()
        .await
        .unwrap();

    println!("len of res = {}", res.len());

    let mut file = File::create("../tmp/证券类别统计-深圳证券交易所.json").unwrap();
    file.write_all(serde_json::to_string_pretty(&res).unwrap().as_bytes())
        .unwrap();
}
