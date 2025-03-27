use std::{sync::LazyLock, time::Duration};

use chrono::{DateTime, Timelike, Utc};
use clean_up::CleanUp;
use reqwest::{Client, ClientBuilder};

use crate::scheduler::{CST, SCHEDULE_TASK_MANAGER};

mod a_stock;
mod clean_up;

const AK_TOOLS_BASE_URL: &'static str = "http://127.0.0.1:8080/api/public";
static HTTP_CLIENT: LazyLock<Client> = LazyLock::new(|| {
    ClientBuilder::new()
        .connect_timeout(Duration::from_secs(5))
        .timeout(Duration::from_secs(20))
        .build()
        .unwrap()
});

fn with_base_url(path: &str) -> String {
    format!("{}{}", AK_TOOLS_BASE_URL, path)
}

/// 一个接近于交易时间的cron表达式
const TRADE_TIME_CRON: &'static str = "*/30 * 9-11,13-14 * * MON-FRI";

// 定义交易时间段（以分钟表示）
const MORNING_START: u32 = 9 * 60 + 30; // 9:30
const MORNING_END: u32 = 11 * 60 + 30; // 11:30
const AFTERNOON_START: u32 = 13 * 60; // 13:00
const AFTERNOON_END: u32 = 15 * 60; // 15:00

/// 判断是否当前处于交易时间内
fn in_trade_time(now: &DateTime<Utc>) -> bool {
    // 使用提供的时间或获取当前CST时间
    let dt = now.with_timezone(&CST);
    let hour = dt.hour();
    let minute = dt.minute();
    let current_minutes = hour * 60 + minute;

    (current_minutes >= MORNING_START && current_minutes < MORNING_END)
        || (current_minutes >= AFTERNOON_START && current_minutes < AFTERNOON_END)
}

pub async fn start_up_monitor_tasks() {
    SCHEDULE_TASK_MANAGER.add_task(CleanUp).await;

    a_stock::start_a_stock_tasks().await;
}
