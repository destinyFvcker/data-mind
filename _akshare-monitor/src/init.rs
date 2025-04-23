use crate::config::INIT_CONFIG;
use reqwest::ClientBuilder;
use std::time::Duration;

/// 每一个调度任务会共享的外部资源
#[derive(Clone)]
pub struct ExternalResource {
    pub ch_client: clickhouse::Client,
    pub http_client: reqwest::Client,
}

impl ExternalResource {
    pub fn init() -> Self {
        let ch_client = clickhouse::Client::default()
            .with_url(format!(
                "http://{}:{}",
                INIT_CONFIG.clickhouse.host, INIT_CONFIG.clickhouse.port
            ))
            .with_user(&INIT_CONFIG.clickhouse.user)
            .with_password(&INIT_CONFIG.clickhouse.password)
            .with_database(&INIT_CONFIG.clickhouse.database);

        let http_client = ClientBuilder::new()
            .connect_timeout(Duration::from_secs(5))
            .timeout(Duration::from_secs(20))
            .build()
            .unwrap();

        Self {
            ch_client,
            http_client,
        }
    }
}
