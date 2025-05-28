//! 和钉钉报警机器人相关的错误类型定义，具体见
//! [消息机器人错误码](https://open.dingtalk.com/document/orgapp/custom-robots-send-group-messages#6a8e23113eggw)

use common_error::{common_code, ext::ErrorExt};
use common_macro::stack_trace_debug;
use snafu::Snafu;

use crate::schema::dingding::DingTalkRobotRes;

#[derive(Snafu)]
#[snafu(visibility(pub))]
#[stack_trace_debug]
pub enum Error {
    #[snafu(display("服务器webhook网络请求错误: {}", error))]
    NetReq {
        #[snafu(source)]
        error: reqwest::Error,
    },
    #[snafu(display("钉钉系统繁忙，请稍后重试"))] // -1
    SystemBusy,
    #[snafu(display("群已被解散，请向其他群发送消息"))] // 400013
    GroupDisbanded,
    #[snafu(display("机器人已停用，请联系管理员启用机器人"))] // 400102
    BotDisabled,
    #[snafu(display("机器人不存在，请确认机器人是否在群中"))] // 400106
    BotNotFound,
    #[snafu(display("发送速度太快而限流，请降低发送速度"))] // 410100
    RateLimited,
    #[snafu(display("其他错误: {}", reason))]
    Other { reason: String },
}

impl ErrorExt for Error {
    #[inline]
    fn common_code(&self) -> common_error::common_code::CommonCode {
        match self {
            Error::SystemBusy => common_code::CommonCode::DingTalkSystemBusy,
            Error::GroupDisbanded => common_code::CommonCode::DingTalkGroupDisbanded,
            Error::BotDisabled => common_code::CommonCode::DingTalkBotDisabled,
            Error::BotNotFound => common_code::CommonCode::DingTalkBotNotFound,
            Error::RateLimited => common_code::CommonCode::DingTalkRateLimited,
            Error::Other { .. } | Error::NetReq { .. } => common_code::CommonCode::Internal,
        }
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl Error {
    pub fn from_error_res(err_res: DingTalkRobotRes) -> Option<Self> {
        match err_res.errcode.as_str() {
            "0" => None,
            "-1" => Some(Self::SystemBusy),
            "400013" => Some(Self::GroupDisbanded),
            "400102" => Some(Self::BotDisabled),
            "400106" => Some(Self::BotNotFound),
            "410100" => Some(Self::RateLimited),
            _ => Some(Self::Other {
                reason: err_res.errmsg,
            }),
        }
    }
}

pub type DingResult<T> = Result<T, Error>;
