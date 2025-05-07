use tech_indi::mount_tech_indicator_scopr;
use utoipa_actix_web::service_config::ServiceConfig;

pub mod a_index;
pub mod a_stock;
pub mod news;
pub mod tech_indi;

/// Run external configuration as part of the application building process
pub fn config() -> impl FnOnce(&mut ServiceConfig) {
    |config: &mut ServiceConfig| {
        mount_tech_indicator_scopr(config);
    }
}
