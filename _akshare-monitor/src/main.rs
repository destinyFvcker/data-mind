use config::INIT_CONFIG;
use data_mind::utils::perform_ch_ddl;
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
            FileAppender::builder()
                .path(format!("{}/scheduler.log", INIT_CONFIG.server.logdir))
                .rotate(Period::Day)
                .expire(Duration::days(2))
                .build(),
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
    let ddls = [
        include_str!("../ddl/init_stock.sql"),
        include_str!("../ddl/init_index.sql"),
    ];

    for ddl in ddls {
        perform_ch_ddl(ch_client, ddl).await;
    }
}
