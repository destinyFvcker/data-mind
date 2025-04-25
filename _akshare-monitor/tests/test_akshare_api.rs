use data_mind::schema;
use reqwest::{Client, ClientBuilder};
use std::{sync::LazyLock, time::Duration};

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
