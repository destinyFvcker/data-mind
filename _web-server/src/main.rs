use actix_web::{web::Data, App, HttpServer};
use ftlog::appender::{FileAppender, Period};
use handler::*;
use std::{env, net::Ipv4Addr, sync::Arc, thread};
use time::Duration;
use utoipa::OpenApi;
use utoipa_actix_web::AppExt;
use utoipa_scalar::{Scalar, Servable};

mod behind;
mod handler;
mod init;
mod init_config;
mod repository;
mod schema;

#[actix_web::main]
async fn main() {
    // 在这里先初始化配置数据结构，因为相关日志的文件系统位置也存在配置文件之中
    let app_config = Arc::new(init_config::InitConfig::new().unwrap());

    let local_dev = env::var("LOCAL_DEV").unwrap_or_default();

    let _guard = ftlog::builder()
        .max_log_level(if "1" == local_dev {
            ftlog::LevelFilter::Debug
        } else {
            ftlog::LevelFilter::Info
        })
        .root(
            FileAppender::builder()
                .path(format!("{}/server.log", app_config.server.logdir))
                .rotate(Period::Day)
                .expire(Duration::days(7))
                .build(),
        )
        .try_init()
        .expect("logger build or set failed");

    // ----------------------------------------- ⬇️ some init operation
    // init db
    let db_clietns = Arc::new(init::init_db(&app_config).await);
    // init github_state refresh on main thread
    let github_state = behind::github_state::GithubState::begin_processing();

    #[derive(OpenApi)]
    #[openapi(
        tags(
            (name = user::API_TAG, description = user::API_DESC),
            (name = auth::API_TAG, description = auth::API_DESC)
        )
    )]
    struct ApiDoc;

    ftlog::info!("Data Mind web server stated!");
    let shared_config = app_config.clone();
    HttpServer::new(move || {
        let mut app = App::new()
            .into_utoipa_app()
            .openapi(ApiDoc::openapi())
            .app_data(Data::from(db_clietns.clone()))
            .service(utoipa_actix_web::scope("/api").configure(handler::user::config()))
            .service(
                utoipa_actix_web::scope("/auths")
                    .configure(handler::auth::config(Data::from(github_state.clone()))),
            )
            .openapi_service(|api| Scalar::with_url("/scalar-doc", api))
            .into_app();

        app = if "1" == local_dev {
            app
        } else {
            app.service(
                actix_files::Files::new("/", &shared_config.server.fe).index_file("index.html"),
            )
        };

        app
    })
    .bind((Ipv4Addr::UNSPECIFIED, app_config.server.port))
    .unwrap()
    .run()
    .await
    .unwrap();
}
