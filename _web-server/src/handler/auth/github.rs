use actix_web::{
    get, post,
    web::{self, Data, Json},
    Result,
};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use utoipa_actix_web::{scope, service_config::ServiceConfig};

use crate::behind::github_state::GithubState;

pub(super) fn mount_github_scope(config: &mut ServiceConfig, state: Data<GithubState>) {
    config
        .app_data(state)
        .service(scope("/github").service(github_hook).service(get_state));
}

/// 在从当前服务重定向到github OAuth界面需要的一个不可猜测的随机字符串，
/// 用于防止跨站请求伪造攻击
#[derive(Debug, Serialize, ToSchema)]
struct StateResponse {
    /// 不可猜测的随机字符串
    state: String,
}

#[utoipa::path(
    tag = super::API_TAG
)]
#[get("/state")]
async fn get_state(github_state: Data<GithubState>) -> Json<StateResponse> {
    Json(StateResponse {
        state: github_state.new_state(),
    })
}

/// 从github OAuth界面重定向回服务时github请求携带的请求体
#[derive(Debug, Deserialize, ToSchema)]
struct HookQuery {
    /// 收到的作为对用户同意使用github进行登陆的响应的代码。
    code: String,
    /// 不可猜测的随机字符串，用于防止跨站请求伪造攻击。
    state: String,
}

#[utoipa::path(
    tag = super::API_TAG
)]
#[post("/hook")]
async fn github_hook(hook_query: web::Json<HookQuery>) -> Result<()> {
    Ok(())
}
