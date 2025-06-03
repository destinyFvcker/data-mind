//! 普通输入密码进行鉴权的相关handler定义

use actix_web::{
    post,
    web::{self, Json},
    HttpResponse,
};
use data_mind::schema::common::EmptyOkRes;
use serde::Deserialize;
use snafu::ResultExt;
use sqlx::MySqlPool;
use utoipa::ToSchema;
use utoipa_actix_web::{scope, service_config::ServiceConfig};

use crate::{
    handler::auth::{
        error::{DbErrSnafu, EncodeJwtSnafu, UserExistSnafu, UserPasswordMismatchSnafu},
        jwt_mw::gen_jwt,
    },
    init_config::InitConfig,
    repository::{auth_repo, user_config_repo},
};

use super::error::AuthError;

pub(super) fn mount_plain_auth_scope(config: &mut ServiceConfig) {
    config.service(
        scope("/plain_auth")
            .service(plain_sign_in)
            .service(plain_sign_up),
    );
}

/// 注册/登录data-mind账号请求体结构
#[derive(Debug, Deserialize, ToSchema)]
struct SignBody {
    /// 邮箱账号
    #[schema(example = "destinyfunk@163.com")]
    email: String,
    /// 密码
    #[schema(example = "12345678")]
    password: String,
}

/// 普通账号注册
#[utoipa::path(
    tag = super::API_TAG,
    responses(
        (status = 200, description = "成功创建用户", body = EmptyOkRes),
        (status = 409, description = "用户已存在，请进行登录", body = AuthError),
        (status = 500, description = "服务器内部错误", body = AuthError)
    )
)]
#[post("/signup")]
async fn plain_sign_up(
    mysql_client: web::Data<MySqlPool>,
    body: Json<SignBody>,
) -> Result<Json<EmptyOkRes>, AuthError> {
    // HACK 下面这段代码处理得不好，主要是之前从来都没有用sqlx启动过事务，错误处理方面也是一坨大的
    let mut tx = mysql_client.begin().await.context(DbErrSnafu)?;
    let user_id = auth_repo::UserRepo::insert_part(&mut *tx, &body.email, &body.password).await;

    use sqlx::{mysql::MySqlDatabaseError, Error as SqlxError};
    let user_id = match user_id {
        Ok(user_id) => user_id,
        Err(SqlxError::Database(db_err)) => {
            let mysql_err = db_err.downcast_ref::<MySqlDatabaseError>();
            if mysql_err.number() == 1062 {
                return Err(UserExistSnafu.build().into());
            } else {
                Err(SqlxError::Database(db_err)).context(DbErrSnafu)?;
                unreachable!()
            }
        }
        Err(err) => {
            Err(err).context(DbErrSnafu)?;
            unreachable!()
        }
    };

    user_config_repo::insert_new_ding_robot(&mut *tx, user_id)
        .await
        .context(DbErrSnafu)?;
    tx.commit().await.context(DbErrSnafu)?;

    Ok(Json(EmptyOkRes::from_msg("成功创建用户✔".to_owned())))
}

/// 普通账号登录
#[utoipa::path(
    tag = super::API_TAG,
    responses(
        (
            status = 200,
            description = "empty body with jwt token in the header",
            headers(
                ("Authorization" = String, description = "New jwt token")
            ),
            body = EmptyOkRes
        ),
        (
            status = 500,
            description = "服务器内部错误❌",
            body = AuthError
        )
    )
)]
#[post("/signin")]
async fn plain_sign_in(
    mysql_client: web::Data<MySqlPool>,
    init_config: web::Data<InitConfig>,
    body: Json<SignBody>,
) -> Result<HttpResponse, AuthError> {
    let sign_body = body.into_inner();
    let user_id =
        auth_repo::UserRepo::verify_by_plain(&mysql_client, &sign_body.email, &sign_body.password)
            .await
            .context(DbErrSnafu)?
            .ok_or(())
            .map_err(|_| {
                UserPasswordMismatchSnafu {
                    email_address: sign_body.email,
                }
                .build()
            })?;

    let jwt = gen_jwt(user_id, &init_config.jwt_secret_key).context(EncodeJwtSnafu)?;

    // 创建响应并添加自定义头部
    Ok(HttpResponse::Ok()
        .append_header(("Authorization", jwt))
        // .append_header(("X-User-ID", user_id.to_string()))
        .json(EmptyOkRes::from_msg("登录成功 🚀".to_owned()))) // 假设你有一个默认实现
}

#[cfg(test)]
mod test {
    #[tokio::test]
    async fn test_sign_up() {
        todo!()
    }

    #[tokio::test]
    async fn test_sign_in() {
        todo!()
    }
}
