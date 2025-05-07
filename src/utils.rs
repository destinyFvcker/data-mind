//! 工具类处理函数

use std::{collections::HashSet, sync::Arc, time::Duration};

use backoff::ExponentialBackoff;
use chrono::DateTime;
use rskafka::{
    client::{
        consumer::{StreamConsumer, StreamConsumerBuilder},
        partition::{OffsetAt, PartitionClient},
    },
    BackoffConfig,
};
use sqlx::{Executor, MySqlPool};

pub const AK_TOOLS_BASE_URL: &'static str = "http://127.0.0.1:8080/api/public";

/// 通过指定的数据字典项拼接出实际的aktools目标数据url
pub fn with_base_url(path: &str) -> String {
    format!("{}{}", AK_TOOLS_BASE_URL, path)
}

/// 获取一个比较合理的指数退避重拾策略
#[inline]
pub fn config_backoff(max_interval: u64, max_elapsed_time: u64) -> ExponentialBackoff {
    ExponentialBackoff {
        initial_interval: Duration::from_millis(100), // 第一次失败后100ms重试
        randomization_factor: 0.5,                    // 加入一定的抖动，避免雪崩
        multiplier: 2.0,                              // 每次间隔翻倍
        max_interval: Duration::from_secs(max_interval), // 单次最大间隔1秒
        max_elapsed_time: Some(Duration::from_secs(max_elapsed_time)), // 总最大重试时间12秒
        ..Default::default()
    }
}

/// 将类似于ISO 8601标准表达方式的时间字符串 2025-04-22T00:00:00.000
/// 截取出 T 前面的 yyyy-mm-dd 部分，方便转换为NaiveDate
pub fn splite_date_naive(date_str: &str) -> &str {
    if let Some(pos) = date_str.as_bytes().iter().position(|c| *c == b'T') {
        &date_str[..pos]
    } else {
        date_str
    }
}

/// 通过传入一个clickhouse客户端的引用运行一个ddl.sql文件之中所有的内容，自动对注释内容进行去除
pub async fn perform_ch_ddl(ch_client: &clickhouse::Client, raw_ddl_file: &str) {
    async fn query_ddl_by_line(ddl: String, ch_client: &clickhouse::Client) {
        let ddl: Vec<String> = ddl.split(";").map(|s| s.to_string()).collect();
        for sql in ddl.into_iter() {
            if sql.is_empty() {
                continue;
            }
            ch_client.query(&sql).execute().await.unwrap();
        }
    }

    query_ddl_by_line(clean_up(raw_ddl_file), ch_client).await;
}

/// 通过传入一个mysql客户端的引用运行一个ddl.sql文件之中的所有内容，自动对注释内容进行去除
pub async fn perform_mysql_ddl(mysql_client: &MySqlPool, raw_ddl_file: &str) {
    async fn query_ddl_by_line(ddl: String, mysql_client: &MySqlPool) {
        let ddl: Vec<&str> = ddl.split(";").collect();
        for sql in ddl {
            if sql.is_empty() {
                continue;
            }
            mysql_client
                .acquire()
                .await
                .unwrap()
                .execute(sql)
                .await
                .unwrap();
        }
    }

    query_ddl_by_line(clean_up(raw_ddl_file), mysql_client).await;
}

/// 清理传入的ddl.sql文件的内容，删除空行以及注释，返回一个单行的纯sql字符串
fn clean_up(raw_ddl_file: &str) -> String {
    raw_ddl_file
        .to_string()
        .trim()
        .lines()
        .map(|s| s.to_string())
        .filter(|line| {
            !(line.trim().starts_with("/*") || line.trim().starts_with("--") || line.is_empty())
        })
        .map(|line| match line.find("--") {
            Some(pos) => line[..pos].trim().to_owned(),
            None => line.trim().to_owned(),
        })
        .reduce(|s, line| s + " " + &line)
        .map(|str| str.trim().to_owned())
        .unwrap_or("".to_string())
}

/// 获取从一个固定的时间戳开始消耗的kafka stream
pub async fn get_kafka_stream(
    client: Arc<PartitionClient>,
    start_ts: i64,
) -> (i64, StreamConsumer) {
    let start_offset = client
        .get_offset(OffsetAt::Timestamp(DateTime::from_timestamp_nanos(
            start_ts,
        )))
        .await
        .unwrap();
    let latest_offset = client.get_offset(OffsetAt::Latest).await.unwrap();

    let kafka_stream = StreamConsumerBuilder::new(
        client,
        rskafka::client::consumer::StartOffset::At(start_offset),
    )
    .with_max_batch_size(10_000_000)
    .build();

    (latest_offset, kafka_stream)
}

/// 通过指定`broker`、`topic`和`partition`获取一个指定分区对应的client
pub async fn connect_kafka(
    broker: &str,
    topic: &str,
    partition: i32,
) -> rskafka::client::partition::PartitionClient {
    let client = rskafka::client::ClientBuilder::new(vec![broker.to_owned()])
        .backoff_config(BackoffConfig {
            deadline: Some(Duration::from_secs(15)),
            ..Default::default()
        })
        .build()
        .await
        .unwrap();

    // 假如对应的topic还没有被创建的话，就先进行创建
    let exist_topics = client
        .list_topics()
        .await
        .unwrap()
        .into_iter()
        .map(|t| t.name)
        .collect::<HashSet<_>>();
    if !exist_topics.contains(topic) {
        client
            .controller_client()
            .unwrap()
            .create_topic(topic, 1, 1, 300)
            .await
            .unwrap();
    }

    client
        .partition_client(
            topic,
            partition,
            rskafka::client::partition::UnknownTopicHandling::Error,
        )
        .await
        .unwrap()
}

#[cfg(test)]
mod test {

    use chrono::NaiveDate;
    use std::str::FromStr;

    use super::*;

    #[test]
    fn test_date_transfer() {
        let date_str = splite_date_naive("2025-04-22T00:00:00.000");
        let naive_date = NaiveDate::from_str(date_str);
        println!("{:?}", naive_date);

        // let date_time = <DateTime<Utc>>::from_str("2025-04-22T00:00:00.000").unwrap();
        // let naive_date = date_time.naive_local();
        // println!("{:?}", naive_date);
    }
}
