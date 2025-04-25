use config::INIT_CONFIG;
use ftlog::{
    LevelFilter,
    appender::{Duration, FileAppender, Period},
};
use handler::get_app;
use init::ExternalResource;
use poem::{Server, listener::TcpListener};

mod config;
mod handler;
mod init;
mod scheduler;
mod tasks;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let time_format = time::format_description::parse_owned::<1>(
        "[year]-[month]-[day] [hour]:[minute]:[second].[subsecond digits:6]",
    )
    .unwrap();
    let _guard = ftlog::builder()
        .filter("scheduler::info", "scheduler", LevelFilter::Info)
        .appender(
            "scheduler",
            FileAppender::rotate(
                format!("{}/scheduler.log", INIT_CONFIG.server.logdir),
                Period::Day,
            ),
        )
        .max_log_level(ftlog::LevelFilter::Info)
        .time_format(time_format)
        .root(
            FileAppender::builder()
                .path(format!("{}/server.log", INIT_CONFIG.server.logdir))
                .rotate(Period::Day)
                .expire(Duration::days(7))
                .build(),
        )
        .try_init()
        .expect("logger build or set failed");

    let ext_res = ExternalResource::init();

    perform_ddl(&ext_res.ch_client).await;
    scheduler::scheduler_start_up(ext_res).await?;

    ftlog::info!("Data Mind akshare monitor stated!");
    Server::new(TcpListener::bind(format!(
        "0.0.0.0:{}",
        INIT_CONFIG.server.port
    )))
    .run(get_app())
    .await?;

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

    async fn query_ddl_by_line(ddl: String, ch_client: &clickhouse::Client) {
        let ddl: Vec<String> = ddl.split(";").map(|s| s.to_string()).collect();
        for sql in ddl.into_iter() {
            if sql.is_empty() {
                continue;
            }
            ch_client.query(&sql).execute().await.unwrap();
        }
    }

    let ddls = [
        include_str!("../ddl/init_stock.sql"),
        include_str!("../ddl/init_index.sql"),
    ];

    for ddl in ddls {
        let stock_ddl = cleanup(ddl);
        query_ddl_by_line(stock_ddl, ch_client).await;
    }
}
