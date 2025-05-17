//! 普通输入密码进行鉴权的相关handler定义

use serde::Deserialize;
use utoipa::ToSchema;
use utoipa_actix_web::service_config::ServiceConfig;

use super::error::AuthError;

pub(super) fn mount_github_scope(config: &mut ServiceConfig) {
    todo!()
}

/// 注册data-mind账号请求体结构
#[derive(Debug, Deserialize, ToSchema)]
struct RegisterBody {
    /// 邮箱账号
    #[schema(example = "destinyfunk@163.com")]
    email: String,
    /// 密码
    #[schema(example = "12345678")]
    password: String,
}

async fn register_user() -> Result<(), AuthError> {
    todo!()
}
