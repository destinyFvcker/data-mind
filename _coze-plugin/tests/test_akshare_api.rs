use std::{fs::File, io::Write, sync::LazyLock, time::Duration};

use reqwest::{Client, ClientBuilder};
use serde_json::Value;

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

#[actix_web::test]
async fn test_stock_financial_abstract_ths() {
    let indicators = ["按报告期", "按年度", "按单季度"];

    for indicator in indicators {
        let res: Vec<Value> = TEST_HTTP_CLIENT
            .get(with_base_url("/stock_financial_abstract_ths"))
            .query(&[("symbol", "000063"), ("indicator", indicator)])
            .send()
            .await
            .unwrap()
            .error_for_status()
            .unwrap()
            .json()
            .await
            .unwrap();

        println!("indicator = {indicator}, len = {}", res.len());

        let mut file = File::create(format!(
            "../tmp/同花顺-财务指标-主要指标_{}.json",
            indicator
        ))
        .unwrap();
        file.write_all(serde_json::to_string_pretty(&res).unwrap().as_bytes())
            .unwrap();
    }
}

#[actix_web::test]
async fn test_stock_news_main_cx() {
    let res: Vec<Value> = TEST_HTTP_CLIENT
        .get(with_base_url("/stock_news_main_cx"))
        .send()
        .await
        .unwrap()
        .error_for_status()
        .unwrap()
        .json()
        .await
        .unwrap();

    println!("res len = {}", res.len());
    let mut file = File::create("../tmp/财经内容精选.json").unwrap();
    file.write_all(serde_json::to_string_pretty(&res).unwrap().as_bytes())
        .unwrap();
}
