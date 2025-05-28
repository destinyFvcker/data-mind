use serde::Deserialize;
use sqlx::{prelude::FromRow, Executor, MySql, MySqlPool};

use crate::schema::user_config::UserConfigShow;

impl UserConfigShow {
    /// 通过user id找到对应的前端可显示信息配置
    pub async fn fetch_with_user_id(
        mysql_client: &MySqlPool,
        user_id: u64,
    ) -> anyhow::Result<Option<Self>> {
        let user_config = sqlx::query_as::<_, Self>(
            r#"
        SELECT 
            u.nickname,
            dr.webhook_address
        FROM users u
        LEFT JOIN dingtalk_robots dr
            on u.id = dr.user_id
        WHERE u.id = ?
        "#,
        )
        .bind(user_id)
        .fetch_optional(mysql_client)
        .await?;

        Ok(user_config)
    }
}

#[derive(Debug, FromRow, Deserialize)]
pub struct DingTalkRebotConfigRepo {
    pub webhook_address: Option<String>,
    pub key_signature: Option<String>,
}

impl DingTalkRebotConfigRepo {
    pub async fn fetch_with_user_id(
        mysql_client: &MySqlPool,
        user_id: u64,
    ) -> anyhow::Result<Option<Self>> {
        let ding_config = sqlx::query_as::<_, Self>(
            r#"
        SELECT
            webhook_address,
            key_signature
        FROM dingtalk_robots
        WHERE user_id = ?
        "#,
        )
        .bind(user_id)
        .fetch_optional(mysql_client)
        .await?;

        Ok(ding_config)
    }
}

/// 插入一个新用户对应的钉钉报警机器人配置
pub async fn insert_new_ding_robot<'e, E>(mysql_client: E, user_id: u64) -> Result<(), sqlx::Error>
where
    E: Executor<'e, Database = MySql>,
{
    sqlx::query("INSERT INTO dingtalk_robots (user_id) VALUES (?)")
        .bind(user_id)
        .fetch_optional(mysql_client)
        .await?;
    Ok(())
}

/// 更新用户名
pub async fn update_nick_name(
    mysql_client: &MySqlPool,
    user_id: u64,
    nick_name: &str,
) -> anyhow::Result<()> {
    sqlx::query("UPDATE users SET nickname = ? WHERE id = ?")
        .bind(nick_name)
        .bind(user_id)
        .execute(mysql_client)
        .await?;
    Ok(())
}

/// 更新钉钉报警机器人webhook地址
pub async fn update_webhook_addr(
    mysql_client: &MySqlPool,
    user_id: u64,
    webhook_addr: &str,
) -> anyhow::Result<()> {
    sqlx::query("UPDATE dingtalk_robots SET webhook_address = ? where user_id = ?")
        .bind(webhook_addr)
        .bind(user_id)
        .execute(mysql_client)
        .await?;
    Ok(())
}

/// 更新钉钉报警机器人密钥
pub async fn update_ding_webhook_secret(
    mysql_client: &MySqlPool,
    user_id: u64,
    secret: &str,
) -> anyhow::Result<()> {
    sqlx::query("UPDATE dingtalk_robots SET key_signature = ? where user_id = ?")
        .bind(secret)
        .bind(user_id)
        .execute(mysql_client)
        .await?;
    Ok(())
}

/// 修改密码
pub async fn update_user_password(
    mysql_client: &MySqlPool,
    user_id: u64,
    password: &str,
) -> anyhow::Result<()> {
    sqlx::query("UPDATE users SET password_hash = ? where id = ?")
        .bind(password)
        .bind(user_id)
        .execute(mysql_client)
        .await?;
    Ok(())
}

/// 验证用户密码是否匹配
pub async fn check_password_right(
    mysql_client: &MySqlPool,
    user_id: u64,
    password: &str,
) -> anyhow::Result<bool> {
    let correct: (bool,) = sqlx::query_as(
        r#"
SELECT EXISTS(
    SELECT 1 FROM users 
    WHERE id = ? AND (password_hash = ? OR password_hash IS NULL)
) AS password_correct;
    "#,
    )
    .bind(password)
    .bind(user_id)
    .fetch_one(mysql_client)
    .await?;

    Ok(correct.0)
}

// FIXME 这里处理的实际上并不是很好，应为无法区分用户是否存在的状况
