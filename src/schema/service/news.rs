//! 财经新闻相关数据响应体

use chrono::{DateTime, Utc};
use clickhouse::Row;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// 财新网-财新数据通-内容精选
#[derive(Debug, Serialize, ToSchema, Deserialize, Row)]
pub struct StockNewsMainCx {
    /// 新闻的正式发布时间，即新闻内容原文在财新网等发布的时间。
    #[serde(with = "clickhouse::serde::chrono::datetime64::millis")]
    pub pub_time: DateTime<Utc>,
    /// 新闻的摘要内容，对新闻正文的简要概括，便于快速了解新闻主旨。
    pub summary: String,
    /// 新闻的主题标签，通常由几个关键词组成，归纳了该新闻的主要话题或核心内容。
    pub tag: String,
    /// 新闻的详情链接，点击可以跳转到财新网对应的新闻完整正文页面。
    pub url: String,
}

impl StockNewsMainCx {
    /// 获取最近的100条精选的财经信息
    pub async fn fetch_recent100(ch_client: &clickhouse::Client) -> anyhow::Result<Vec<Self>> {
        let data = ch_client
            .query(
                r#"
                SELECT
                    pub_time,
                    summary,
                    tag,
                    url
                FROM stock_news_main_cx
                ORDER BY pub_time DESC
                LIMIT 100
                "#,
            )
            .fetch_all()
            .await?;
        Ok(data)
    }

    /// 分页获取一定时间段内的财经新闻数据
    pub async fn fetch_range(
        ch_client: &clickhouse::Client,
        start_time: &str,
        end_time: &str,
        limit: u32,
        offset: u32,
    ) -> anyhow::Result<Vec<Self>> {
        let data = ch_client
            .query(
                r#"
                SELECT
                    pub_time,
                    summary,
                    tag,
                    url
                FROM stock_news_main_cx
                WHERE (pub_time >= parseDateTimeBestEffort( ? )) AND (pub_time <= parseDateTimeBestEffort( ? ))
                ORDER BY pub_time DESC
                LIMIT ?, ?
                "#,
            )
            .bind(start_time)
            .bind(end_time)
            .bind(offset)
            .bind(limit)
            .fetch_all()
            .await?;
        Ok(data)
    }
}

pub use crate::schema::akshare::AkStockNewsEm;

#[cfg(test)]
mod test {
    use crate::utils::TEST_CH_CLIENT;

    use super::*;

    #[tokio::test]
    async fn fetch_recent100() {
        let data = StockNewsMainCx::fetch_recent100(&TEST_CH_CLIENT)
            .await
            .unwrap();
        println!("{:?}", data);
    }

    #[tokio::test]
    async fn fetch_range() {
        let data = StockNewsMainCx::fetch_range(
            &TEST_CH_CLIENT,
            "2025-05-01 00:00:00",
            "2025-05-07 23:59:59",
            10,
            0,
        )
        .await
        .unwrap();
        println!("{:?}", data);
    }
}
