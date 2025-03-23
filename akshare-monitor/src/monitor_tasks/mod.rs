use std::{sync::LazyLock, time::Duration};

use reqwest::{Client, ClientBuilder};

pub const AK_TOOLS_BASE_URL: &'static str = "http://127.0.0.1:8080/api/public";
pub static HTTP_CLIENT: LazyLock<Client> = LazyLock::new(|| {
    ClientBuilder::new()
        .connect_timeout(Duration::from_secs(5))
        .timeout(Duration::from_secs(20))
        .build()
        .unwrap()
});

fn with_base_url(path: &str) -> String {
    // TODO 如何在编译时生成格式化字符串？
    format!("{}/{}", AK_TOOLS_BASE_URL, path)
}
