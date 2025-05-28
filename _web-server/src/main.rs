use actix_files::NamedFile;
use actix_web::{
    dev::{fn_service, ServiceRequest, ServiceResponse},
    web::Data,
    App, HttpServer,
};
use data_mind::handler::{a_stock, indicator, news};
use ftlog::appender::{FileAppender, Period};
use handler::{auth::jwt_mw::JwtAuthGuard, *};
use init::DbClients;
use std::{env, net::Ipv4Addr, sync::Arc};
use time::Duration;
use utoipa::{
    openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme},
    Modify, OpenApi,
};
use utoipa_actix_web::AppExt;
use utoipa_scalar::{Scalar, Servable};
use utoipa_swagger_ui::SwaggerUi;

mod background;
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
    let DbClients { clickhouse, mysql } = init::init_db(&app_config).await;
    // init github_state refresh on main thread
    let github_state = background::github_state::GithubStateCache::begin_processing();
    // init reqwest client
    let reqwest_client = init::init_reqwest_client();
    // ----------------------------------------- done

    #[derive(OpenApi)]
    #[openapi(
        tags(
            (name = auth::API_TAG, description = auth::API_DESC),
            (name = indicator::API_TAG, description = indicator::API_DESC),
            (name = news::API_TAG, description = news::API_DESC),
            (name = a_stock::API_TAG, description = a_stock::API_DESC),
            (name = manage::API_TAG, description = manage::API_DESC)
        ),
        servers(
            (url = "http://localhost:8800", description = "本地测试环境"),
            (url = "https://www.destinyfvcker.cn/data-mind", description = "线上部署环境")
        ),
        modifiers(&SecurityAddon)
    )]
    struct ApiDoc;

    struct SecurityAddon;

    impl Modify for SecurityAddon {
        fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
            let components = openapi.components.as_mut().unwrap(); // we can unwrap safely since there already is components registered.
            components.add_security_scheme(
                "bearer",
                SecurityScheme::Http(
                    HttpBuilder::new()
                        .scheme(HttpAuthScheme::Bearer)
                        .bearer_format("JWT")
                        .build(),
                ),
            )
        }
    }

    ftlog::info!("Data Mind web server stated!");
    let shared_config = app_config.clone();
    HttpServer::new(move || {
        let mut app = App::new()
            .into_utoipa_app()
            .openapi(ApiDoc::openapi())
            .app_data(Data::new(clickhouse.clone()))
            .app_data(Data::new(mysql.clone()))
            .app_data(Data::new(reqwest_client.clone()))
            .app_data(Data::from(shared_config.clone()));

        let api_scope = utoipa_actix_web::scope("/api")
            .configure(handler::quant_data::config())
            .configure(handler::manage::config());

        app = if local_dev != "1" {
            app.service(api_scope.wrap(JwtAuthGuard::new(shared_config.jwt_secret_key.clone())))
        } else {
            app.service(api_scope)
        };

        let mut app = app
            .service(
                utoipa_actix_web::scope("/auths")
                    .configure(handler::auth::config(Data::from(github_state.clone()))),
            )
            .openapi_service(|api| Scalar::with_url("/scalar-doc", api))
            .openapi_service(|api| {
                SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", api)
            })
            .into_app();

        app = if local_dev == "1" {
            app
        } else {
            let inner_config = shared_config.clone(); // TMD
            app.service(
                actix_files::Files::new("/", &shared_config.server.fe)
                    .index_file("index.html") // .default_handler(f)
                    .default_handler(fn_service(move |req: ServiceRequest| {
                        let config = inner_config.clone();
                        async move {
                            let (req, _) = req.into_parts();
                            let file =
                                NamedFile::open_async(format!("{}/index.html", config.server.fe))
                                    .await?;
                            let res = file.into_response(&req);

                            Ok(ServiceResponse::new(req, res))
                        }
                    })),
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
