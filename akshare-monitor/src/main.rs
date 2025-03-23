use std::sync::LazyLock;

use ch::CH_CLIENT;
use chrono::{FixedOffset, Utc};
use config::CONFIG;
use data_mind::models::{
    akshare::{self, a_stock},
    ch_db,
};
use ftlog::appender::{Duration, FileAppender, Period};
use handler::get_app;
use monitor_tasks::{AK_TOOLS_BASE_URL, HTTP_CLIENT};
use poem::{Server, listener::TcpListener};

mod ch;
mod config;
mod handler;
mod monitor_tasks;
mod scheduler;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    perform_ddl(&CH_CLIENT).await;

    let result: Vec<akshare::RealtimeStockMarketRecord> = HTTP_CLIENT
        .get(format!("{}/{}", AK_TOOLS_BASE_URL, "stock_zh_a_spot_em"))
        .send()
        .await?
        .json()
        .await?;

    // let cst = FixedOffset::east_opt(8 * 3600).unwrap();
    // let ts = Utc::now().with_timezone(&cst);
    let ts = Utc::now();
    let astock_realtime_data_row = result
        .into_iter()
        .map(|record| ch_db::RealtimeStockMarketRecord::from_with_ts(record, ts))
        .collect::<Vec<_>>();

    let mut inserter = CH_CLIENT.inserter("astock_realtime_data")?;
    for row in astock_realtime_data_row {
        inserter.write(&row)?;
    }
    let stats = inserter.commit().await?;
    if stats.rows > 0 {
        println!(
            "{} bytes, {} rows, {} transactions have been inserted",
            stats.bytes, stats.rows, stats.transactions,
        );
    }
    inserter.end().await?;

    // let time_format = time::format_description::parse_owned::<1>(
    //     "[year]-[month]-[day] [hour]:[minute]:[second].[subsecond digits:6]",
    // )
    // .unwrap();
    // // TODO 这里应该做一下区分，就是测试分支和部署分支不使用同一个日志等级
    // let _guard = ftlog::builder()
    //     .max_log_level(ftlog::LevelFilter::Info)
    //     .time_format(time_format)
    //     .root(
    //         FileAppender::builder()
    //             .path("./logs/server.log")
    //             .rotate(Period::Day)
    //             .expire(Duration::days(7))
    //             .build(),
    //     )
    //     .try_init()
    //     .expect("logger build or set failed");

    // ftlog::info!("Data Mind web server stated!");
    // Server::new(TcpListener::bind(format!("0.0.0.0:{}", CONFIG.server.port)))
    //     .run(get_app())
    //     .await?;

    Ok(())
}

async fn perform_ddl(ch_client: &clickhouse::Client) {
    let cleanup = |raw_ddl: &str| {
        raw_ddl
            .to_string()
            .trim()
            .lines()
            .map(|s| s.to_string())
            .filter(|line| {
                !(line.trim().starts_with("/*") || line.trim().starts_with("--") || line.is_empty())
            })
            .map(|line| match line.find("--") {
                Some(pos) => line[..pos].trim().to_owned(),
                None => line.trim().to_owned(),
            })
            .reduce(|s, line| s + " " + &line)
            .map(|str| str.trim().to_owned())
            .unwrap_or("".to_string())
    };

    let stock_ddl = cleanup(include_str!("../ddl/init_stock.sql"));

    async fn query_ddl_by_line(ddl: String, ch_client: &clickhouse::Client) {
        let ddl: Vec<String> = ddl.split(";").map(|s| s.to_string()).collect();
        for sql in ddl.into_iter() {
            if sql.is_empty() {
                continue;
            }
            ch_client.query(&sql).execute().await.unwrap();
        }
    }

    query_ddl_by_line(stock_ddl, ch_client).await;
}
