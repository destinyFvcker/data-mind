use actix_web::{App, HttpServer};
use config::CONFIG;
use ftlog::appender::{FileAppender, Period};
use handler::*;
use std::net::Ipv4Addr;
use time::Duration;
use utoipa::OpenApi;
use utoipa_actix_web::AppExt;
use utoipa_scalar::{Scalar, Servable};

mod config;
mod handler;
mod schema;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let time_format = time::format_description::parse_owned::<1>(
        "[year]-[month]-[day] [hour]:[minute]:[second].[subsecond digits:6]",
    )
    .unwrap();
    let _guard = ftlog::builder()
        .max_log_level(ftlog::LevelFilter::Info)
        .time_format(time_format)
        .root(
            FileAppender::builder()
                .path(format!("{}/server.log", CONFIG.server.logdir))
                .rotate(Period::Day)
                .expire(Duration::days(7))
                .build(),
        )
        .try_init()
        .expect("logger build or set failed");

    #[derive(OpenApi)]
    #[openapi(
        tags(
            (name = user::API_TAG, description = user::API_DESC)
        )
    )]
    struct ApiDoc;

    ftlog::info!("Data Mind web server stated!");
    HttpServer::new(|| {
        App::new()
            .into_utoipa_app()
            .openapi(ApiDoc::openapi())
            .service(utoipa_actix_web::scope("/api").configure(handler::user::config()))
            .openapi_service(|api| Scalar::with_url("/scalar-doc", api))
            .into_app()
            .service(actix_files::Files::new("/", &CONFIG.server.fe).index_file("index.html"))
    })
    .bind((Ipv4Addr::UNSPECIFIED, CONFIG.server.port))?
    .run()
    .await?;

    Ok(())
}
