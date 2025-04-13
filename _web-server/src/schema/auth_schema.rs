use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::repository::{
    self,
    auth_repo::{UserIdentityRepo, UserRepo},
};

/// 在从当前服务重定向到github OAuth界面需要的一个不可猜测的随机字符串，
/// 用于防止跨站请求伪造攻击
#[derive(Debug, Serialize, ToSchema)]
pub struct GithubState {
    /// 不可猜测的随机字符串
    pub state: String,
}

/// 从github OAuth界面重定向回服务时github请求携带的请求体
#[derive(Debug, Deserialize, ToSchema)]
pub struct GithubCallback {
    /// 收到的作为对用户同意使用github进行登陆的响应的代码。
    pub code: String,
    /// 不可猜测的随机字符串，用于防止跨站请求伪造攻击。
    pub state: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(transparent)]
pub struct UserSchema(pub UserRepo);

#[derive(Debug, Serialize, Deserialize)]
#[serde(transparent)]
pub struct UserIdentitySchema(pub UserIdentityRepo);

impl UserSchema {
    pub fn into_innter(self) -> UserRepo {
        self.0
    }
}

impl UserIdentitySchema {
    pub fn into_inner(self) -> UserIdentityRepo {
        self.0
    }
}
