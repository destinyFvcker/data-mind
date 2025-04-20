use actix_web::web::Data;
use github::mount_github_scope;
use utoipa_actix_web::service_config::ServiceConfig;

use crate::background::github_state::GithubStateCache;

mod error;
mod github;
pub mod jwt_mw;

/// Run external configuration as part of the application building process
pub fn config(github_state: Data<GithubStateCache>) -> impl FnOnce(&mut ServiceConfig) {
    |config: &mut ServiceConfig| {
        mount_github_scope(config, github_state);
    }
}

pub const API_TAG: &'static str = "auths";
pub const API_DESC: &'static str =
    "web服务鉴权功能模块，接入github/wechat OAuth登陆，提供jwt middleware";
