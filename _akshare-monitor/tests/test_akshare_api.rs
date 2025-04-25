use data_mind::schema;
use reqwest::{Client, ClientBuilder};
use serde_json::Value;
use std::{env, fs::File, io::Write, sync::LazyLock, time::Duration};

static HTTP_CLIENT: LazyLock<Client> = LazyLock::new(|| {
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
    let res = HTTP_CLIENT
        .get(with_base_url("/stock_zh_a_hist"))
        .query(&[
            ("symbol", "000088"),
            ("period", "daily"),
            ("start_date", "20250418"),
            ("end_date", "20250424"),
            ("adjust", ""),
        ])
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    let value: Vec<schema::StockZhAHist> = serde_json::from_str(&res).unwrap();
    println!("values = {:#?}", value);
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
        let res = HTTP_CLIENT
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
    let res = HTTP_CLIENT
        .get(with_base_url("/stock_zh_index_spot_sina"))
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    let values: Vec<schema::StockZhIndexSpotSina> = serde_json::from_str(&res).unwrap();
    println!("res len = {}", values.len());

    let mut file = File::create("../tmp/实时行情数据-新浪.json").unwrap();
    file.write_all(res.as_bytes()).unwrap();
}

#[tokio::test]
async fn test_stock_zh_index_daily() {
    let res = HTTP_CLIENT
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
