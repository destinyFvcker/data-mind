/// 从A股实时数据表之中获取所有的股票id
pub async fn get_distinct_code(ch_client: &clickhouse::Client) -> anyhow::Result<Vec<String>> {
    let res = ch_client
        .query(
            "SELECT DISTINCT code \
            FROM astock_realtime_data \
            WHERE is_suspended = false",
        )
        .fetch_all::<String>()
        .await?;

    Ok(res)
}

#[cfg(test)]
mod test {
    use crate::monitor_tasks::TEST_CH_CLIENT;

    use super::*;

    #[tokio::test]
    async fn test_distinct_code() {
        let res = get_distinct_code(&TEST_CH_CLIENT).await.unwrap();
        println!("{:?}", res);
    }
}
