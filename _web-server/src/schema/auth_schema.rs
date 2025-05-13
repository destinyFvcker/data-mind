#![allow(unused)]
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

/// JWT的有效负载部分定义
#[derive(Debug, Serialize, Deserialize)]
pub struct JwtClaims {
    /// 主题，即用户标识
    pub sub: String,
    /// 过期时间（Expiry time），一个毫秒等级的时间戳，
    /// 用于设置 token 的过期时间。
    pub exp: i64,
}

/// 在从当前服务重定向到github OAuth界面需要的一个不可猜测的随机字符串，
/// 用于防止跨站请求伪造攻击
#[derive(Debug, Serialize, ToSchema)]
pub struct GithubState {
    /// 不可猜测的随机字符串
    #[schema(example = "VrEaJ191gmyuhB5CKq0x")]
    pub state: String,
}

/// 从github OAuth界面重定向回服务时github请求携带的请求体
#[derive(Debug, Deserialize, ToSchema, IntoParams)]
pub struct GithubCallback {
    /// 收到的作为对用户同意使用github进行登陆的响应的代码。
    #[schema(example = "A.u2r=n?N^Ea3Y5.?rLzF+U0ce")]
    pub code: String,
    /// 不可猜测的随机字符串，用于防止跨站请求伪造攻击。
    #[schema(example = "VrEaJ191gmyuhB5CKq0x")]
    pub state: String,
}

/// 从github接收到的访问令牌响应
#[derive(Debug, Deserialize)]
pub struct GithubTokenRes {
    /// 代表用户对github进行访问的token
    pub access_token: String,
    /// 令牌访问权限范围
    pub scope: String,
    /// 令牌类型
    pub token_type: String,
}

#[derive(Debug, Deserialize)]
pub struct UserSchema {
    pub email: String,
    pub password: String,
    pub mobile: Option<String>,
    pub nickname: String,
    pub avatar_url: String,
}

#[derive(Debug, Deserialize)]
pub struct UserIdentitySchema {
    pub provider: String,
    pub provider_user_id: String,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum GithubUserInfoRes {
    PrivateUser(GithubPrivateUserInfo),
    PublicUser(GithubPublicUserInfo),
}

#[allow(unused)]
#[derive(Debug, Deserialize)]
pub struct GithubPlan {
    pub collaborators: i32,
    pub name: String,
    pub space: i32,
    pub private_repos: i32,
}

#[allow(unused)]
#[derive(Debug, Deserialize)]
pub struct GithubPublicUserInfo {
    pub login: String,
    pub id: i64,
    pub user_view_type: Option<String>,
    pub node_id: String,
    pub avatar_url: String,
    pub gravatar_id: Option<String>,
    pub url: String,
    pub html_url: String,
    pub followers_url: String,
    pub following_url: String,
    pub gists_url: String,
    pub starred_url: String,
    pub subscriptions_url: String,
    pub organizations_url: String,
    pub repos_url: String,
    pub events_url: String,
    pub received_events_url: String,
    pub r#type: String,
    pub site_admin: bool,
    pub name: Option<String>,
    pub company: Option<String>,
    pub blog: Option<String>,
    pub location: Option<String>,
    pub email: Option<String>,
    pub notification_email: Option<String>,
    pub hireable: Option<bool>,
    pub bio: Option<String>,
    pub twitter_username: Option<String>,
    pub public_repos: i32,
    pub public_gists: i32,
    pub followers: i32,
    pub following: i32,
    pub created_at: String,
    pub updated_at: String,
    pub plan: Option<GithubPlan>,
    pub private_gists: Option<i32>,
    pub total_private_repos: Option<i32>,
    pub owned_private_repos: Option<i32>,
    pub disk_usage: Option<i32>,
    pub collaborators: Option<i32>,
}

#[allow(unused)]
#[derive(Debug, Deserialize)]
pub struct GithubPrivateUserInfo {
    #[serde(flatten)]
    pub base: GithubPublicUserInfo,
    pub collaborators: i32,
    pub disk_usage: i32,
    pub owned_private_repos: i32,
    pub private_gists: i32,
    pub total_private_repos: i32,
    pub two_factor_authentication: bool,
    pub business_plus: Option<bool>,
    pub ldap_dn: Option<String>,
}
