use std::{fs::File, io::Write, sync::LazyLock, time::Duration};

use data_mind::schema::{self, akshare::StockFinancialAbstractThs};
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
        let res: Vec<StockFinancialAbstractThs> = TEST_HTTP_CLIENT
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

#[actix_web::test]
async fn test_stock_rank_cxg_ths() {
    let symbols = ["创月新高", "半年新高", "一年新高", "历史新高"];

    for symbol in symbols {
        let res: Vec<schema::akshare::StockRankCxgThs> = TEST_HTTP_CLIENT
            .get(with_base_url("/stock_rank_cxg_ths"))
            .query(&[("symbol", symbol)])
            .send()
            .await
            .unwrap()
            .error_for_status()
            .unwrap()
            .json()
            .await
            .unwrap();

        println!("symbol = {symbol}, res len = {}", res.len());

        let mut file = File::create(format!("../tmp/技术指标-创新高-{}.json", symbol)).unwrap();
        file.write_all(serde_json::to_string_pretty(&res).unwrap().as_bytes())
            .unwrap();
    }
}

#[actix_web::test]
async fn test_stock_rank_cxd_ths() {
    let symbols = ["创月新低", "半年新低", "一年新低", "历史新低"];

    for symbol in symbols {
        let res: Vec<schema::akshare::StockRankCxdThs> = TEST_HTTP_CLIENT
            .get(with_base_url("/stock_rank_cxd_ths"))
            .query(&[("symbol", symbol)])
            .send()
            .await
            .unwrap()
            .error_for_status()
            .unwrap()
            .json()
            .await
            .unwrap();

        println!("symbol = {symbol}, res len = {}", res.len());

        let mut file = File::create(format!("../tmp/技术指标-创新低-{}.json", symbol)).unwrap();
        file.write_all(serde_json::to_string_pretty(&res).unwrap().as_bytes())
            .unwrap();
    }
}

#[actix_web::test]
async fn test_stock_rank_lxsz_ths() {
    let res: Vec<schema::akshare::StockRankLxszThs> = TEST_HTTP_CLIENT
        .get(with_base_url("/stock_rank_lxsz_ths"))
        .send()
        .await
        .unwrap()
        .error_for_status()
        .unwrap()
        .json()
        .await
        .unwrap();

    println!("res len = {}", res.len());

    let mut file = File::create("../tmp/技术指标-连续上涨.json").unwrap();
    file.write_all(serde_json::to_string_pretty(&res).unwrap().as_bytes())
        .unwrap();
}

#[actix_web::test]
async fn test_stock_rank_lxxd_ths() {
    let res: Vec<schema::akshare::StockRankLxxdThs> = TEST_HTTP_CLIENT
        .get(with_base_url("/stock_rank_lxxd_ths"))
        .send()
        .await
        .unwrap()
        .error_for_status()
        .unwrap()
        .json()
        .await
        .unwrap();

    println!("res len = {}", res.len());

    let mut file = File::create("../tmp/技术指标-连续下跌.json").unwrap();
    file.write_all(serde_json::to_string_pretty(&res).unwrap().as_bytes())
        .unwrap();
}

#[actix_web::test]
async fn test_stock_rank_cxfl_ths() {
    let res: Vec<schema::akshare::StockRankCxflThs> = TEST_HTTP_CLIENT
        .get(with_base_url("/stock_rank_cxfl_ths"))
        .send()
        .await
        .unwrap()
        .error_for_status()
        .unwrap()
        .json()
        .await
        .unwrap();

    println!("res len = {}", res.len());

    let mut file = File::create("../tmp/技术指标-持续放量.json").unwrap();
    file.write_all(serde_json::to_string_pretty(&res).unwrap().as_bytes())
        .unwrap();
}

#[actix_web::test]
async fn test_stock_rank_cxsl_ths() {
    let res: Vec<schema::akshare::StockRankCxslThs> = TEST_HTTP_CLIENT
        .get(with_base_url("/stock_rank_cxsl_ths"))
        .send()
        .await
        .unwrap()
        .error_for_status()
        .unwrap()
        .json()
        .await
        .unwrap();

    println!("res len = {}", res.len());

    let mut file = File::create("../tmp/技术指标-持续缩量.json").unwrap();
    file.write_all(serde_json::to_string_pretty(&res).unwrap().as_bytes())
        .unwrap();
}

#[actix_web::test]
async fn test_stock_news_em() {
    let res: Vec<schema::akshare::StockNewsEm> = TEST_HTTP_CLIENT
        .get(with_base_url("/stock_news_em"))
        .query(&[("symbol", "300059")])
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();

    println!("res len = {}", res.len());

    let mut file = File::create("../tmp/个股新闻.json").unwrap();
    file.write_all(serde_json::to_string_pretty(&res).unwrap().as_bytes())
        .unwrap();
}

#[actix_web::test]
async fn test_stock_individual_info_em() {
    let res: Vec<Value> = TEST_HTTP_CLIENT
        .get(with_base_url("/stock_individual_info_em"))
        .query(&[("symbol", "603777")])
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();

    println!("res len = {}", res.len());

    let mut file = File::create("../tmp/个股信息查询.json").unwrap();
    file.write_all(serde_json::to_string_pretty(&res).unwrap().as_bytes())
        .unwrap();
}
