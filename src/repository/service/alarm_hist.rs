#![allow(unused)]
use chrono::Utc;
use clickhouse::Row;
use serde::{Deserialize, Serialize};

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
