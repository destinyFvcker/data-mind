use data_mind::handler::{
    a_stock::mount_astock_scope, indicator::mount_tech_indicator_scope, news::mount_news_scope,
};
use utoipa_actix_web::service_config::ServiceConfig;

/// Run external configuration as part of the application building process
pub fn config() -> impl FnOnce(&mut ServiceConfig) {
    |config: &mut ServiceConfig| {
        mount_tech_indicator_scope(config);
        mount_news_scope(config);
        mount_astock_scope(config);
    }
}
