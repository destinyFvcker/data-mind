use crate::scheduler::{Schedulable, ScheduleTaskType, TaskMeta};

/// 每天定时清理数据库相关表格，避免磁盘爆炸
pub(super) struct CleanUp {
    ch_client: clickhouse::Client,
}

impl CleanUp {
    pub fn new(ch_client: clickhouse::Client) -> Self {
        Self { ch_client }
    }
}

impl Schedulable for CleanUp {
    fn gen_meta(&self) -> crate::scheduler::TaskMeta {
        TaskMeta {
            name: "clickhouse cleanup".to_owned(),
            desc: "每天定时清理数据库相关表格，避免磁盘超出容量".to_owned(),
            cron_expr: "0 0 6 * * *".to_owned(),
            tag: Some(ScheduleTaskType::System),
        }
    }

    fn execute(
        self: std::sync::Arc<Self>,
    ) -> Box<dyn Future<Output = anyhow::Result<()>> + Send + 'static> {
        Box::new(async move {
            cleanup_astock_realtime_data(&self.ch_client).await?;
            Ok(())
        })
    }
}

/// 定时清除a股实时行情数据
async fn cleanup_astock_realtime_data(_ch_client: &clickhouse::Client) -> anyhow::Result<()> {
    // let sql = r#"
    //     ALTER TABLE akshare.astock_realtime_data
    //     DELETE WHERE date < toDate(now() - INTERVAL 2 DAY)
    // "#;
    // ch_client.query(sql).execute().await?;
    Ok(())
}
