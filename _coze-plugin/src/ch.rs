use std::sync::LazyLock;

use clickhouse::Client;

use crate::config::CONFIG;

pub static CH_CLIENT: LazyLock<Client> = LazyLock::new(|| {
    clickhouse::Client::default()
        .with_url(format!(
            "http://{}:{}",
            CONFIG.clickhouse.host, CONFIG.clickhouse.port
        ))
        .with_user(&CONFIG.clickhouse.user)
        .with_password(&CONFIG.clickhouse.password)
        .with_database(&CONFIG.clickhouse.database)
});
