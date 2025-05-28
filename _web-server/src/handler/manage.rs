//! 用户管理模块，提供一组api来对用户的相关信息进行获取和管理

use actix_web::{
    get, post,
    web::{Data, Json, ReqData},
};
use data_mind::{
    ding_robot::msg_client::DingTalkRobotReq,
    schema::{
        common::{EmptyOkRes, OkRes},
        error::{BadReqSnafu, DingErrSnafu, InternalServerSnafu, NotFoundSnafu, OrdinError},
    },
};
use serde::Deserialize;
use snafu::ResultExt;
use sqlx::MySqlPool;
use utoipa::ToSchema;
use utoipa_actix_web::{scope, service_config::ServiceConfig};

use crate::{
    handler::auth::jwt_mw::UserIdFromJwt,
    repository::user_config_repo::{self, DingTalkRebotConfigRepo},
    schema::user_config::UserConfigShow,
};

pub const API_TAG: &'static str = "用户信息管理";
pub const API_DESC: &'static str = "用于用户系统信息管理的一组api，更新密码昵称等等";

/// Run external configuration as part of the application building process
pub fn config() -> impl FnOnce(&mut ServiceConfig) {
    |config: &mut ServiceConfig| {
        config.service(
            scope("/manage")
                .service(get_user_config_info)
                .service(hook_ding_test_msg)
                .service(update_user_password)
                .service(update_user_nickname)
                .service(update_ding_robot),
        );
    }
}

/// 获取前端可展示的用户可配置信息
#[utoipa::path(
    tag = API_TAG,
    responses(
        (status = 200, description = "获取用户可配置信息成功 ✅", body = EmptyOkRes),
        (status = 500, description = "请求出现错误 💥", body = OrdinError),
        (status = 401, description = "没有权限访问对应资源 🚫", body = OrdinError),
        (status = 404, description = "系统不存在此用户", body = OrdinError) 
    )
)]
#[get("/")]
async fn get_user_config_info(
    mysql_client: Data<MySqlPool>,
    user_id: ReqData<UserIdFromJwt>,
) -> Result<Json<OkRes<UserConfigShow>>, OrdinError> {
    let user_config_show = UserConfigShow::fetch_with_user_id(&mysql_client, user_id.0)
        .await
        .context(InternalServerSnafu)?
        .ok_or(NotFoundSnafu.build())?;

    let res = OkRes::from_with_msg("成功获取用户可配置信息".to_string(), user_config_show);
    Ok(Json(res))
}

/// 向钉钉报警机器人发送测试信息
#[utoipa::path(
    tag = API_TAG,
    responses(
        (status = 200, description = "测试钉钉报警机器人消息发送成功 ✅", body = EmptyOkRes),
        (status = 500, description = "请求出现错误 💥", body = OrdinError),
        (status = 401, description = "没有权限访问对应资源 🚫", body = OrdinError),
        (status = 404, description = "系统不存在此用户", body = OrdinError) 
    )
)]
#[post("/ping_alarm_robot")]
async fn hook_ding_test_msg(
    mysql_client: Data<MySqlPool>,
    reqwest_client: Data<reqwest::Client>,
    user_id: ReqData<UserIdFromJwt>,
) -> Result<Json<EmptyOkRes>, OrdinError> {
    let ding_config_repo = DingTalkRebotConfigRepo::fetch_with_user_id(&mysql_client, user_id.0)
        .await
        .context(InternalServerSnafu)?
        .ok_or(())
        .map_err(|_| NotFoundSnafu.build())?;

    let ding_config: DingTalkRobotReq = ding_config_repo.try_into().map_err(|_| {
        BadReqSnafu {
            desc: "钉钉报警机器人配置不完整，请先进行配置".to_owned(),
        }
        .build()
    })?;

    ding_config
        .ping(&reqwest_client)
        .await
        .context(DingErrSnafu)?;

    Ok(Json(EmptyOkRes::from_msg(
        "发送测试报警消息成功 ✔".to_owned(),
    )))
}

/// 通用的更新信息请求体
#[derive(Debug, ToSchema, Deserialize)]
struct UpdateMsgBody {
    /// 更新请求体内容
    value: String,
}

/// 更新用户密码
#[utoipa::path(
    tag = API_TAG,
    responses(
        (status = 200, description = "更新密码成功 ✅", body = EmptyOkRes),
        (status = 500, description = "请求出现错误 💥", body = OrdinError),
        (status = 401, description = "没有权限访问对应资源 🚫", body = OrdinError),
        (status = 400, description = "提供的密码不正确", body = OrdinError)
    )
)]
#[post("/update_pwd")]
async fn update_user_password(
    mysql_client: Data<MySqlPool>,
    body: Json<UpdateMsgBody>,
    user_id: ReqData<UserIdFromJwt>,
) -> Result<Json<EmptyOkRes>, OrdinError> {
    user_config_repo::check_password_right(&mysql_client, user_id.0, &body.value)
        .await
        .context(InternalServerSnafu)?
        .then_some(())
        .ok_or(
            BadReqSnafu {
                desc: "输入的密码不正确".to_string(),
            }
            .build(),
        )?;

    user_config_repo::update_user_password(&mysql_client, user_id.0, &body.value)
        .await
        .context(InternalServerSnafu)?;

    Ok(Json(EmptyOkRes::from_msg("更新密码成功 ✅".to_owned())))
}

/// 更新用户昵称
#[utoipa::path(
    tag = API_TAG,
    responses(
        (status = 200, description = "更新昵称成功 ✅", body = EmptyOkRes),
        (status = 500, description = "请求出现错误 💥", body = OrdinError),
        (status = 401, description = "没有权限访问对应资源 🚫", body = OrdinError)
    )
)]
#[post("/update_nickname")]
async fn update_user_nickname(
    mysql_client: Data<MySqlPool>,
    body: Json<UpdateMsgBody>,
    user_id: ReqData<UserIdFromJwt>,
) -> Result<Json<EmptyOkRes>, OrdinError> {
    user_config_repo::update_nick_name(&mysql_client, user_id.0, &body.value)
        .await
        .context(InternalServerSnafu)?;
    Ok(Json(EmptyOkRes::from_msg("更新昵称成功 ✅".to_owned())))
}

/// 更新钉钉报警机器人🤖请求体
#[derive(Debug, Deserialize, ToSchema)]
struct UpdateDingBody {
    /// 钉钉报警机器人webhook地址
    webhook_addr: Option<String>,
    /// 钉钉报警机器人密钥签名
    signature: Option<String>,
}

/// 更新钉钉报警机器人配置
#[utoipa::path(
    tag = API_TAG,
    responses(
        (status = 200, description = "更新钉钉报警机器人配置成功 ✅", body = EmptyOkRes),
        (status = 500, description = "请求出现错误 💥", body = OrdinError),
        (status = 401, description = "没有权限访问对应资源 🚫", body = OrdinError),
        (status = 400, description = "请求参数错误（全为空）", body = OrdinError)
    )
)]
#[post("/update_ding")]
async fn update_ding_robot(
    mysql_client: Data<MySqlPool>,
    body: Json<UpdateDingBody>,
    user_id: ReqData<UserIdFromJwt>,
) -> Result<Json<EmptyOkRes>, OrdinError> {
    if let Some(ref webhook_addr) = body.webhook_addr {
        user_config_repo::update_webhook_addr(&mysql_client, user_id.0, &webhook_addr)
            .await
            .context(InternalServerSnafu)?;
    }
    if let Some(ref signature) = body.signature {
        user_config_repo::update_ding_webhook_secret(&mysql_client, user_id.0, &signature)
            .await
            .context(InternalServerSnafu)?;
    }

    if body.signature.is_none() && body.webhook_addr.is_none() {
        Err(BadReqSnafu {
            desc: "请求参数错误（全为空）".to_owned(),
        }
        .build()
        .into())
    } else {
        Ok(Json(EmptyOkRes::from_msg(
            "更新钉钉报警机器人配置成功 ✅".to_owned(),
        )))
    }
}
