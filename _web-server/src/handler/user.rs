//! 处理用户信息web服务模块
use actix_web::{get, web::Json};
use utoipa_actix_web::{scope, service_config::ServiceConfig};

use crate::schema::user_schema::User;

/// Run external configuration as part of the application building process
pub fn config() -> impl FnOnce(&mut ServiceConfig) {
    |config: &mut ServiceConfig| {
        config.service(scope("/user").service(get_user));
    }
}

pub const API_TAG: &'static str = "user";
pub const API_DESC: &'static str =
    "提供用户相关功能的标准化接口，包括用户注册、登录、信息查询与更新等操作";

#[utoipa::path(
    tag = API_TAG,
    responses((status = OK, body = User)),
)]
#[get("/")]
async fn get_user() -> Json<User> {
    Json(User {
        username: "Ryan Gosling".to_owned(),
        password: "Idrive".to_owned(),
    })
}
