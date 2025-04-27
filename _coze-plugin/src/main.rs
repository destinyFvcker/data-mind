use std::{net::Ipv4Addr, sync::Arc};

use actix_web::{App, HttpServer, web::Data};
use ftlog::appender::{FileAppender, Period};
use init::{init_ch_client, init_reqwest_client};
use init_config::InitConfig;
use time::Duration;
use utoipa::OpenApi;
use utoipa_actix_web::AppExt;
use utoipa_scalar::{Scalar, Servable};

mod handler;
mod init;
mod init_config;

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    // 初始化设置⚙️ -----
    let init_config = Arc::new(InitConfig::new().expect("config init can't be wrong!"));

    // 初始化日志 -----
    let time_format = time::format_description::parse_owned::<1>(
        "[year]-[month]-[day] [hour]:[minute]:[second].[subsecond digits:6]",
    )
    .unwrap();
    let _guard = ftlog::builder()
        .max_log_level(ftlog::LevelFilter::Info)
        .time_format(time_format)
        .root(
            FileAppender::builder()
                .path(format!("{}/server.log", init_config.server.logdir))
                .rotate(Period::Day)
                .expire(Duration::days(7))
                .build(),
        )
        .try_init()
        .expect("logger build or set failed");

    // 初始化一些外部资源(init.rs) ------
    let ch_client = init_ch_client(&init_config);
    let reqwest_client = init_reqwest_client();

    // openapi 挂载点 -----
    #[derive(OpenApi)]
    // #[openapi(
    //     tags(
    //         (name = auth::API_TAG, description = auth::API_DESC)
    //     )
    // )]
    struct ApiDoc;

    ftlog::info!("Data Mind coze plugin stated!");
    let shared_config = init_config.clone();
    HttpServer::new(move || {
        App::new()
            .into_utoipa_app()
            .openapi(ApiDoc::openapi())
            .app_data(Data::new(ch_client.clone()))
            .app_data(Data::new(reqwest_client.clone()))
            .app_data(Data::from(shared_config.clone()))
            .openapi_service(|api| Scalar::with_url("/scalar-doc", api))
            .into_app()
    })
    .bind((Ipv4Addr::UNSPECIFIED, init_config.server.port))
    .unwrap()
    .run()
    .await
    .unwrap();

    Ok(())
}
