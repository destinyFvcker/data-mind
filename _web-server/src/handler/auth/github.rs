use actix_web::{
    get, post,
    web::{self, Data, Json},
    Result,
};
use utoipa_actix_web::{scope, service_config::ServiceConfig};

use crate::{
    background::github_state::GithubStateCache,
    schema::auth_schema::{self},
};

pub(super) fn mount_github_scope(config: &mut ServiceConfig, state: Data<GithubStateCache>) {
    config
        .app_data(state)
        .service(scope("/github").service(github_callback).service(get_state));
}

#[utoipa::path(
    tag = super::API_TAG
)]
#[get("/state")]
async fn get_state(github_state: Data<GithubStateCache>) -> Json<auth_schema::GithubState> {
    Json(auth_schema::GithubState {
        state: github_state.new_state(),
    })
}

#[utoipa::path(
    tag = super::API_TAG
)]
#[post("/callback")]
async fn github_callback(
    callback_query: web::Json<auth_schema::GithubCallback>,
    state_cache: Data<GithubStateCache>,
) -> Result<()> {
    Ok(())
}
