use std::time::Duration;

use sqlx::{mysql::MySqlPoolOptions, MySqlPool};

use crate::init_config::InitConfig;

pub struct DbClients {
    pub clickhouse: clickhouse::Client,
    pub mysql: MySqlPool,
}

/// 初始化项目数据库
pub async fn init_db(app_config: &InitConfig) -> DbClients {
    let clickhouse = clickhouse::Client::default()
        .with_url(format!(
            "http://{}:{}",
            app_config.clickhouse.host, app_config.clickhouse.port
        ))
        .with_user(&app_config.clickhouse.user)
        .with_password(&app_config.clickhouse.password)
        .with_database(&app_config.clickhouse.database);

    let mysql = MySqlPoolOptions::new()
        .acquire_timeout(Duration::from_secs(10))
        .connect(&format!(
            "mysql://{}:{}@{}:{}/{}",
            app_config.mysql.user,
            app_config.mysql.password,
            app_config.mysql.host,
            app_config.mysql.port,
            app_config.mysql.database
        ))
        .await
        .unwrap();

    data_mind::utils::perform_mysql_ddl(&mysql, include_str!("../ddl/auth.sql")).await;
    DbClients { clickhouse, mysql }
}

/// 初始化reqwest客户端
pub fn init_reqwest_client() -> reqwest::Client {
    reqwest::ClientBuilder::new()
        .connect_timeout(Duration::from_secs(5))
        .timeout(Duration::from_secs(10))
        .build()
        .unwrap()
}
