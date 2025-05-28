//! 用户信息管理模块相关schema定义

use data_mind::ding_robot::msg_client::DingTalkRobotReq;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use utoipa::ToSchema;

use crate::repository::user_config_repo::DingTalkRebotConfigRepo;

/// 在前端展示的用户相关信息
#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct UserConfigShow {
    /// 用户的钉钉报警机器人地址
    pub ding_webhook_addr: Option<String>,
    /// 用户的昵称
    pub nick_name: String,
}

impl TryFrom<DingTalkRebotConfigRepo> for DingTalkRobotReq {
    type Error = ();

    fn try_from(value: DingTalkRebotConfigRepo) -> Result<Self, Self::Error> {
        match (value.key_signature, value.webhook_address) {
            (Some(key_signature), Some(webhook_address)) => Ok(DingTalkRobotReq {
                webhook_address,
                key_signature,
            }),
            _ => Err(()),
        }
    }
}
