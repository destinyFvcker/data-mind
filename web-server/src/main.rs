use config::CONFIG;
use ftlog::appender::{FileAppender, Period};
use poem::{get, handler, listener::TcpListener, Route, Server};
use time::Duration;

mod config;

#[handler]
async fn hello_world() -> String {
    "Hello! there is datamind!".to_string()
}

fn get_app() -> Route {
    Route::new().nest("/api", Route::new().at("/test", get(hello_world)))
}

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

    Server::new(TcpListener::bind(format!("0.0.0.0:{}", CONFIG.server.port)))
        .run(get_app())
        .await?;

    Ok(())
}
