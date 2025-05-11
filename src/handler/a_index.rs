use utoipa_actix_web::{scope, service_config::ServiceConfig};

pub const API_TAG: &'static str = "A股指数量化金融数据";
pub const API_DESC: &'static str = "一组用于获取A股指数相关金融交易信息的数据接口";

pub fn mount_aindex_scope(config: &mut ServiceConfig) {
    config.service(scope("/aindex"));
}
