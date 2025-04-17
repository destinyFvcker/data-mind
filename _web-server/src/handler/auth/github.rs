use std::collections::HashMap;

use actix_web::{
    get, post,
    web::{self, Data, Json},
};
use snafu::{ensure, ResultExt};
use utoipa_actix_web::{scope, service_config::ServiceConfig};

use crate::{
    background::github_state::GithubStateCache,
    handler::auth::error::{AuthError, GithubApiFailSnafu, GithubStateNotFoundSnafu},
    init_config::InitConfig,
    schema::auth_schema::{self, GithubTokenRes, GithubUserInfoRes},
};

pub(super) fn mount_github_scope(config: &mut ServiceConfig, state: Data<GithubStateCache>) {
    config
        .app_data(state)
        .service(scope("/github").service(github_callback).service(get_state));
}

#[utoipa::path(
    tag = super::API_TAG,
    responses(
        (
            status = 200, 
            description = "在从当前服务重定向到github OAuth界面需要的一个不可猜测的随机字符串，\
                            用于防止跨站请求伪造攻击", 
            body = auth_schema::GithubState 
        )
    )
)]
#[get("/state")]
async fn get_state(github_state: Data<GithubStateCache>) -> Json<auth_schema::GithubState> {
    Json(auth_schema::GithubState {
        state: github_state.new_state(),
    })
}

#[utoipa::path(
    tag = super::API_TAG,
    responses(
        (status = 200, description = "empty body with jwt token in the header",
            headers(
                ("Authorization" = String, description = "New jwt token")
            )
        ),
        (status = 404, description = "github state not found", body = AuthError),
        (status = 500, description = "github api fail", body = AuthError),
    ),
)]
#[post("/callback")]
async fn github_callback(
    callback_body: web::Json<auth_schema::GithubCallback>,
    state_cache: Data<GithubStateCache>,
    reqwest_client: Data<reqwest::Client>,
    init_config: Data<InitConfig>,
) -> Result<(), AuthError> {
    ensure!(
        &state_cache.verify_state(&callback_body.state),
        GithubStateNotFoundSnafu
    );

    // 请求用户的github accesss token
    let mut req_body: HashMap<&str, &str> = HashMap::new();
    req_body.insert("client_id", &init_config.github.client_id);
    req_body.insert("client_secret", &init_config.github.secret);
    req_body.insert("code", &callback_body.code);
    let github_res = reqwest_client
        .post("https://github.com/login/oauth/access_token")
        .json(&req_body)
        .send()
        .await
        .context(GithubApiFailSnafu)?;
    let correct_res = github_res.error_for_status().context(GithubApiFailSnafu)?;
    let token = correct_res
        .json::<GithubTokenRes>()
        .await
        .context(GithubApiFailSnafu)?;

    let user_info_res = reqwest_client
        .get("https://api.github.com/user")
        .header(
            http::header::AUTHORIZATION,
            format!("Bear {}", token.access_token),
        )
        .send()
        .await
        .context(GithubApiFailSnafu)?;
    let correct_res = user_info_res
        .error_for_status()
        .context(GithubApiFailSnafu)?;
    let github_user_info = correct_res
        .json::<GithubUserInfoRes>()
        .await
        .context(GithubApiFailSnafu)?;

    // TODO 从数据库之中找到对应的用户信息 or 直接创建新用户
    ftlog::info!("github user info = {:#?}", github_user_info);

    Ok(())
}
