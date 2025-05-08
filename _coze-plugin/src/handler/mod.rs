use data_mind::handler::{news::mount_news_scope, tech_indi::mount_tech_indicator_scope};
use utoipa_actix_web::service_config::ServiceConfig;

pub mod a_index;
pub mod a_stock;

/// Run external configuration as part of the application building process
pub fn config() -> impl FnOnce(&mut ServiceConfig) {
    |config: &mut ServiceConfig| {
        mount_tech_indicator_scope(config);
        mount_news_scope(config);
    }
}
