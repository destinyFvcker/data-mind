use std::time::{SystemTime, UNIX_EPOCH};

use actix_web::{
    get,
    web::{self, Json, ReqData},
};
use data_mind::schema::{
    common::OkRes,
    error::{InternalServerSnafu, OrdinError},
};
use jsonwebtoken::{Algorithm, EncodingKey};
use rand::{distr::Alphanumeric, Rng};
use reqwest::header;
use serde::{Deserialize, Serialize};
use snafu::ResultExt;
use utoipa_actix_web::{scope, service_config::ServiceConfig};

use crate::{
    handler::auth::jwt_mw::UserIdFromJwt,
    init_config::{CozeConfig, InitConfig},
};

pub const API_TAG: &'static str = "Coze api";
pub const API_DESC: &'static str = "和Coze平台进行交互的相关api";

/// Run external configuration as part of the application building process
pub fn config() -> impl FnOnce(&mut ServiceConfig) {
    |config: &mut ServiceConfig| {
        config.service(scope("/coze").service(get_coze_access_token));
    }
}

#[utoipa::path(
    tag = API_TAG,
    responses(
        (status = 200, description = "成功获取coze access token ✅", body = OkRes<String>),
        (status = 500, description = "请求出现错误 💥", body = OrdinError),
        (status = 401, description = "没有权限访问对应资源 🚫", body = OrdinError),
    )
)]
#[get("/access_token")]
async fn get_coze_access_token(
    reqwest_client: web::Data<reqwest::Client>,
    init_config: web::Data<InitConfig>,
    user_id: ReqData<UserIdFromJwt>,
) -> Result<Json<OkRes<String>>, OrdinError> {
    let jwt = gen_coze_jwt(user_id.0, &init_config.coze).context(InternalServerSnafu)?;
    let coze_res = acquire_access_token(&reqwest_client, &jwt)
        .await
        .context(InternalServerSnafu)?;

    let res = OkRes::from_with_msg(
        "成功获取coze access token! ✅".to_owned(),
        coze_res.access_token,
    );
    Ok(Json(res))
}

#[derive(Debug, Deserialize)]
struct CozeAccessTokenRes {
    access_token: String,
    #[allow(unused)]
    expires_in: u64,
}

#[derive(Debug, Serialize)]
struct CozeAccessTokenReqBody {
    grant_type: String,
    duration_seconds: u64,
}

impl Default for CozeAccessTokenReqBody {
    fn default() -> Self {
        Self {
            grant_type: "urn:ietf:params:oauth:grant-type:jwt-bearer".to_owned(),
            duration_seconds: 86399, // 24小时
        }
    }
}

async fn acquire_access_token(
    reqwest_token: &reqwest::Client,
    jwt: &str,
) -> anyhow::Result<CozeAccessTokenRes> {
    let req_body = serde_json::to_string(&CozeAccessTokenReqBody::default()).unwrap();

    let res: CozeAccessTokenRes = reqwest_token
        .post("https://api.coze.cn/api/permission/oauth2/token")
        .header(header::CONTENT_TYPE, "application/json")
        .header(header::AUTHORIZATION, format!("Bearer {jwt}"))
        .body(req_body)
        .send()
        .await?
        .error_for_status()?
        .json()
        .await?;

    Ok(res)
}

/// coze 平台智能体
#[derive(Debug, Serialize)]
pub struct CozeJwtPayload {
    /// OAuth 应用的 ID，可以在OAuth 应用页面查看
    iss: String,
    /// 扣子 API 的 Endpoint，即 api.coze.cn
    aud: String,
    /// JWT 开始生效的时间，Unixtime 时间戳格式，精确到秒。  
    /// 一般为当前时刻
    iat: u64,
    /// JWT 过期的时间，Unixtime 时间戳格式，精确到秒。  
    /// 必须晚于 iat
    exp: u64,
    /// 随机字符串，用于防止重放攻击。  
    /// 建议长度大于 32 字节。每次签署 JWT 时应指定为不同的字符串
    jti: String,
    /// 访问令牌的会话标识。目前仅限在会话隔离场景下使用，  
    /// 即将 session_name 指定为用户在业务侧的 UID，  
    /// 以此区分不同业务侧用户的对话历史。
    session_name: String,
}

impl CozeJwtPayload {
    fn new(user_id: u64, iss: &str) -> Self {
        let iat = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();
        let exp = iat + 86400; // 86400 seconds = 24 hours
        let jti = gen_jti();

        Self {
            iss: iss.to_owned(),
            aud: "api.coze.cn".to_owned(),
            iat,
            exp,
            jti,
            session_name: format!("user_{user_id}"),
        }
    }
}

fn gen_coze_jwt(user_id: u64, coze_config: &CozeConfig) -> anyhow::Result<String> {
    let jwt_body = CozeJwtPayload::new(user_id, &coze_config.id);

    // let claims = JwtClaims { sub, exp };
    let header = jsonwebtoken::Header {
        alg: Algorithm::RS256,
        typ: Some("JWT".to_string()),
        kid: Some(coze_config.kid.to_owned()),
        ..Default::default()
    };

    let token = jsonwebtoken::encode(
        &header,
        &jwt_body,
        &EncodingKey::from_rsa_pem(coze_config.signingkey.as_bytes()).unwrap(),
    )?;
    Ok(token)
}

fn gen_jti() -> String {
    rand::rng()
        .sample_iter(&Alphanumeric)
        .take(32) // 这里是长度
        .map(char::from)
        .collect()
}
