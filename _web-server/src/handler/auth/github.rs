use std::collections::HashMap;

use actix_web::{
    get,
    web::{self, Data, Json},
    HttpResponse,
};
use data_mind::{
    schema::common::{EmptyOkRes, OkRes},
    utils::redirect_resp,
};
use serde::Deserialize;
use snafu::ResultExt;
use sqlx::MySqlPool;
use utoipa::IntoParams;
use utoipa_actix_web::{scope, service_config::ServiceConfig};

use crate::{
    background::github_state::GithubStateCache,
    handler::auth::{
        error::{
            AuthError, DbErrSnafu, EncodeJwtSnafu, GithubApiFailSnafu, GithubStateNotFoundSnafu,
        },
        jwt_mw::gen_jwt,
    },
    init_config::InitConfig,
    repository::{
        auth_repo::{self, GITHUB_PROVIDER},
        user_config_repo,
    },
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
            body = OkRes<auth_schema::GithubState>
        )
    )
)]
#[get("/state")]
async fn get_state(github_state: Data<GithubStateCache>) -> Json<OkRes<auth_schema::GithubState>> {
    let data = auth_schema::GithubState {
        state: github_state.new_state(),
    };
    let res = OkRes::from_with_msg("获取成功".to_string(), data);
    Json(res)
}
/// 从github OAuth界面重定向回服务时github请求携带的请求体
#[derive(Debug, Deserialize, IntoParams)]
pub struct GithubCallback {
    /// 收到的作为对用户同意使用github进行登陆的响应的代码。
    #[param(example = "A.u2r=n?N^Ea3Y5.?rLzF+U0ce")]
    pub code: String,
    /// 不可猜测的随机字符串，用于防止跨站请求伪造攻击。
    #[param(example = "VrEaJ191gmyuhB5CKq0x")]
    pub state: String,
}

#[utoipa::path(
    tag = super::API_TAG,
    params (
        GithubCallback
    ),
    responses(
        (
            status = 302,
            description = "empty body with jwt token in the path",
        ),
        (status = 404, description = "github state not found", body = AuthError),
        (status = 500, description = "github api fail", body = AuthError),
    ),
)]
#[get("/callback")]
async fn github_callback(
    callback_body: web::Query<GithubCallback>,
    state_cache: Data<GithubStateCache>,
    reqwest_client: Data<reqwest::Client>,
    mysql_client: Data<MySqlPool>,
    init_config: Data<InitConfig>,
) -> actix_web::Either<HttpResponse, Result<Json<EmptyOkRes>, AuthError>> {
    if !state_cache.verify_state(&callback_body.state) {
        return actix_web::Either::Right(Err(GithubStateNotFoundSnafu.build().into()));
    }

    // 请求用户的github accesss token
    let mut req_body: HashMap<&str, &str> = HashMap::new();
    req_body.insert("client_id", &init_config.github.client_id);
    req_body.insert("client_secret", &init_config.github.secret);
    req_body.insert("code", &callback_body.code);
    let header_map = reqwest::header::HeaderMap::from_iter([(
        reqwest::header::ACCEPT,
        "application/json".parse().unwrap(),
    )]);
    let github_res = data_mind::try_or_either!(reqwest_client
        .post("https://github.com/login/oauth/access_token")
        .headers(header_map)
        .json(&req_body)
        .send()
        .await
        .context(GithubApiFailSnafu));
    let correct_res =
        data_mind::try_or_either!(github_res.error_for_status().context(GithubApiFailSnafu));
    let token = data_mind::try_or_either!(correct_res
        .json::<GithubTokenRes>()
        .await
        .context(GithubApiFailSnafu));

    ftlog::debug!("{:?}", token);

    let user_info_res = data_mind::try_or_either!(reqwest_client
        .get("https://api.github.com/user")
        .header(reqwest::header::USER_AGENT, "data_mind")
        .header(
            reqwest::header::AUTHORIZATION,
            format!("Bearer {}", token.access_token),
        )
        .header(reqwest::header::ACCEPT, "application/vnd.github+json")
        .header("X-GitHub-Api-Version", "2022-11-28")
        .send()
        .await
        .context(GithubApiFailSnafu));

    let correct_res =
        data_mind::try_or_either!(user_info_res.error_for_status().context(GithubApiFailSnafu));
    let github_user_info = data_mind::try_or_either!(correct_res
        .json::<GithubUserInfoRes>()
        .await
        .context(GithubApiFailSnafu));

    let (email_addr, avatar_url, provider_user_id) = match github_user_info {
        GithubUserInfoRes::PrivateUser(github_private_user_info) => (
            github_private_user_info.base.email,
            github_private_user_info.base.avatar_url,
            github_private_user_info.base.id.to_string(),
        ),
        GithubUserInfoRes::PublicUser(github_public_user_info) => (
            github_public_user_info.email,
            github_public_user_info.avatar_url,
            github_public_user_info.id.to_string(),
        ),
    };

    // HACK 这里假如对应账号没有绑定对应邮箱的话，直接跳转到一个错误页面
    let Some(email_addr) = email_addr else {
        return actix_web::Either::Left(redirect_resp(&format!(
            "{}/error/no-email",
            init_config.server.deploy_path
        )));
    };

    let user_iden = data_mind::try_or_either!(auth_repo::UserIdentityRepo::find_by_provider(
        &mysql_client,
        GITHUB_PROVIDER,
        &provider_user_id
    )
    .await
    .context(DbErrSnafu));

    match user_iden {
        Some(user_iden) => {
            let jwt =
                data_mind::try_or_either!(
                    gen_jwt(user_iden.user_id, &init_config.jwt_secret_key).context(EncodeJwtSnafu)
                );
            return actix_web::Either::Left(redirect_resp(&format!(
                "{}/oauth-loading#token={}",
                init_config.server.deploy_path, jwt
            )));
        }
        None => {
            // TODO 这里应该考虑用户邮箱已经存在的情况，但是我有点懒得做了
            // TODO 当然还有事务了😄，这里也没有进行考虑
            let user_id = data_mind::try_or_either!(auth_repo::UserRepo::insert(
                &mysql_client,
                &email_addr,
                "12345678",
                None,
                "用户z",
                &avatar_url
            )
            .await
            .context(DbErrSnafu));

            data_mind::try_or_either!(auth_repo::UserIdentityRepo::insert(
                &mysql_client,
                user_id,
                GITHUB_PROVIDER,
                &provider_user_id,
            )
            .await
            .context(DbErrSnafu));

            data_mind::try_or_either!(user_config_repo::insert_new_ding_robot(
                &**mysql_client,
                user_id
            )
            .await
            .context(DbErrSnafu));

            let jwt = data_mind::try_or_either!(
                gen_jwt(user_id, &init_config.jwt_secret_key).context(EncodeJwtSnafu)
            );
            return actix_web::Either::Left(redirect_resp(&format!(
                "{}/oauth-loading#token={}",
                init_config.server.deploy_path, jwt
            )));
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::init::init_reqwest_client;

    #[actix_web::test]
    async fn test_github_user_api() {
        let reqwest_clinet = init_reqwest_client();
        let req = reqwest_clinet
            .get("https://api.github.com/user")
            .header(
                "Authorization",
                format!("Bearer {}", "you should not put this in code"),
            )
            .header("Accept", "application/vnd.github+json")
            .header("X-GitHub-Api-Version", "2022-11-28");

        let user_info_res = req.send().await.context(GithubApiFailSnafu).unwrap();

        let correct_res = user_info_res
            .error_for_status()
            .context(GithubApiFailSnafu)
            .unwrap();
        let github_user_info = correct_res
            .json::<GithubUserInfoRes>()
            .await
            .context(GithubApiFailSnafu)
            .unwrap();

        println!("{:?}", github_user_info);
    }

    #[test]
    fn test_serde_unit() {
        let unit = ();
        let result = serde_json::to_string(&unit).unwrap();
        println!("{result}");
    }
}
