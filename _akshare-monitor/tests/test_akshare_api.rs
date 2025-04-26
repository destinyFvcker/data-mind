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

    let value: Vec<schema::akshare::StockZhAHist> = serde_json::from_str(&res).unwrap();
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

    let values: Vec<schema::akshare::StockZhIndexSpotSina> = serde_json::from_str(&res).unwrap();
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

#[tokio::test]
async fn test_index_option_50etf_qvix() {
    let values: Vec<Value> = HTTP_CLIENT
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
    let values: Vec<Value> = HTTP_CLIENT
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
        let values: Vec<Value> = HTTP_CLIENT
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
