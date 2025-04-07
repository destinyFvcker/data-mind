use actix_web::{App, HttpServer};
use ftlog::appender::{FileAppender, Period};
use handler::*;
use sqlx::mysql::MySqlPoolOptions;
use std::{net::Ipv4Addr, sync::Arc};
use time::Duration;
use utoipa::OpenApi;
use utoipa_actix_web::AppExt;
use utoipa_scalar::{Scalar, Servable};

mod config;
mod handler;
mod schema;

#[tokio::main]
async fn main() {
    let app_config = Arc::new(config::Config::new().unwrap());
    let time_format = time::format_description::parse_owned::<1>(
        "[year]-[month]-[day] [hour]:[minute]:[second].[subsecond digits:6]",
    )
    .unwrap();
    let _guard = ftlog::builder()
        .max_log_level(ftlog::LevelFilter::Info)
        .time_format(time_format)
        .root(
            FileAppender::builder()
                .path(format!("{}/server.log", app_config.server.logdir))
                .rotate(Period::Day)
                .expire(Duration::days(7))
                .build(),
        )
        .try_init()
        .expect("logger build or set failed");

    let ch_client = clickhouse::Client::default()
        .with_url(format!(
            "http://{}:{}",
            app_config.clickhouse.host, app_config.clickhouse.port
        ))
        .with_user(&app_config.clickhouse.user)
        .with_password(&app_config.clickhouse.password)
        .with_database(&app_config.clickhouse.database);

    let pool = MySqlPoolOptions::new()
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

    data_mind::utils::perform_mysql_ddl(&pool, include_str!("../ddl/auth.sql")).await;

    #[derive(OpenApi)]
    #[openapi(
        tags(
            (name = user::API_TAG, description = user::API_DESC)
        )
    )]
    struct ApiDoc;

    ftlog::info!("Data Mind web server stated!");
    let shared_config = app_config.clone();
    HttpServer::new(move || {
        App::new()
            .into_utoipa_app()
            .openapi(ApiDoc::openapi())
            .service(utoipa_actix_web::scope("/api").configure(handler::user::config()))
            .openapi_service(|api| Scalar::with_url("/scalar-doc", api))
            .into_app()
            .service(
                actix_files::Files::new("/", &shared_config.server.fe).index_file("index.html"),
            )
    })
    .bind((Ipv4Addr::UNSPECIFIED, app_config.server.port))
    .unwrap()
    .run()
    .await
    .unwrap();

    unreachable!()
}
