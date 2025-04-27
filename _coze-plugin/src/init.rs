//! 一些全局资源的初始化，例如数据库连接池，网络客户端等等

use std::time::Duration;

use crate::init_config::InitConfig;

/// 初始化clickhouse数据库连接池
pub fn init_ch_client(init_config: &InitConfig) -> clickhouse::Client {
    clickhouse::Client::default()
        .with_url(format!(
            "http://{}:{}",
            init_config.clickhouse.host, init_config.clickhouse.port
        ))
        .with_user(&init_config.clickhouse.user)
        .with_password(&init_config.clickhouse.password)
        .with_database(&init_config.clickhouse.database)
}

/// 初始化reqwest客户端
pub fn init_reqwest_client() -> reqwest::Client {
    reqwest::ClientBuilder::new()
        .connect_timeout(Duration::from_secs(5))
        .timeout(Duration::from_secs(10))
        .build()
        .unwrap()
}
