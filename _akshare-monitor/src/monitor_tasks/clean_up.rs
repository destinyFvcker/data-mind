use crate::{
    ch::CH_CLIENT,
    scheduler::{Schedulable, ScheduleTaskType, TaskMeta},
};

/// 每天定时清理数据库相关表格，避免磁盘爆炸
pub(super) struct CleanUp;

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
            cleanup_astock_realtime_data().await?;
            Ok(())
        })
    }
}

async fn cleanup_astock_realtime_data() -> anyhow::Result<()> {
    let sql = r#"
        ALTER TABLE akshare.astock_realtime_data
        DELETE WHERE date < toDate(now() - INTERVAL 2 DAY)
    "#;
    CH_CLIENT.query(sql).execute().await?;
    Ok(())
}
