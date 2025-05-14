#![allow(unused)]
use chrono::Utc;
use clickhouse::Row;
use serde::{Deserialize, Serialize};
use sqlx::MySqlPool;

use crate::schema::service::alarm::{DingTalkRobotReq, DingTalkRobotRes};

/// 报警历史消息，还是比较简陋的状态
#[derive(Debug, Deserialize, Serialize, Row)]
pub struct AlarmHist {
    /// 报警的uuid
    pub id: String,
    /// 报警的时间
    #[serde(with = "clickhouse::serde::chrono::datetime")]
    pub event_time: chrono::DateTime<Utc>,
    #[serde(with = "serde_bytes")]
    /// 报警的proto_data数据
    pub proto_data: Vec<u8>,
}

impl AlarmHist {
    pub async fn insert(
        ch_client: &clickhouse::Client,
        id: &str,
        proto_data: &[u8],
    ) -> clickhouse::error::Result<()> {
        todo!()
    }
}

impl DingTalkRobotRes {
    pub async fn fetch_option(
        mysql_pool: &MySqlPool,
        user_id: u64,
    ) -> anyhow::Result<Option<Self>> {
        let sql = r#"
SELECT 
    webhook_address, 
    CONVERT_TZ(created_at, @@session.time_zone, '+08:00') AS created_at,
    CONVERT_TZ(updated_at, @@session.time_zone, '+08:00') AS updated_at
FROM dingtalk_robots 
WHERE user_id = ? 
        "#;

        let robot_config = sqlx::query_as(sql)
            .bind(user_id)
            .fetch_optional(mysql_pool)
            .await?;
        Ok(robot_config)
    }
}

impl DingTalkRobotReq {
    pub async fn update_config(&self, mysql_pool: &MySqlPool, user_id: u64) -> anyhow::Result<()> {
        let sql = r#"
INSERT INTO dingtalk_robots (user_id, webhook_address, key_signature)
VALUES (?, ?, ?)
ON DUPLICATE KEY UPDATE
    webhook_address = VALUES(webhook_address),
    key_signature = VALUES(key_signature),
    updated_at = CURRENT_TIMESTAMP;
        "#;

        sqlx::query(sql)
            .bind(user_id)
            .bind(&self.webhook_address)
            .bind(&self.key_signature)
            .execute(mysql_pool)
            .await?;

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::utils::get_test_mysql_pool;

    use super::*;

    #[tokio::test]
    async fn test_dingtalk_robot() {
        let mysql_pool = get_test_mysql_pool().await;

        let update1 = DingTalkRobotReq {
            webhook_address: "todo".to_owned(),
            key_signature: "todo".to_owned(),
        };

        update1.update_config(&mysql_pool, 15).await.unwrap();

        let res1 = DingTalkRobotRes::fetch_option(&mysql_pool, 15)
            .await
            .unwrap()
            .unwrap();
        println!("{:?}", res1);

        assert_eq!(res1.webhook_address, update1.webhook_address);
    }
}
