use config::CONFIG;
use ftlog::appender::{FileAppender, Period};
use handler::get_app;
use poem::{listener::TcpListener, Server};
use time::Duration;

mod config;
mod handler;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let time_format = time::format_description::parse_owned::<1>(
        "[year]-[month]-[day] [hour]:[minute]:[second].[subsecond digits:6]",
    )
    .unwrap();
    // TODO 这里应该做一下区分，就是测试分支和部署分支不使用同一个日志等级
    let _guard = ftlog::builder()
        .max_log_level(ftlog::LevelFilter::Info)
        .time_format(time_format)
        .root(
            FileAppender::builder()
                .path("./logs/server.log")
                .rotate(Period::Day)
                .expire(Duration::days(7))
                .build(),
        )
        .try_init()
        .expect("logger build or set failed");

    ftlog::info!("Data Mind web server stated!");
    Server::new(TcpListener::bind(format!("0.0.0.0:{}", CONFIG.server.port)))
        .run(get_app())
        .await?;

    Ok(())
}
