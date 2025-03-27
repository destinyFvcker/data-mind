use chrono::Utc;
use data_mind::models::{akshare, ch_db};

use crate::{
    ch::CH_CLIENT,
    scheduler::{SCHEDULE_TASK_MANAGER, Schedulable, ScheduleTaskType, TaskMeta},
};

use super::{HTTP_CLIENT, TRADE_TIME_CRON, in_trade_time, with_base_url};

pub(super) async fn start_a_stock_tasks() {
    let realtime_stock_monitor = RealTimeStockMonitor {
        data_url: with_base_url("/stock_zh_a_spot_em"),
    };
    SCHEDULE_TASK_MANAGER.add_task(realtime_stock_monitor).await;
}

struct RealTimeStockMonitor {
    data_url: String,
}

impl Schedulable for RealTimeStockMonitor {
    fn gen_meta(&self) -> crate::scheduler::TaskMeta {
        TaskMeta {
            name: "stock_zh_a_spot_em".to_owned(),
            desc: "东方财富网-沪深京 A 股-实时行情数据".to_owned(),
            cron_expr: TRADE_TIME_CRON.to_owned(),
            tag: Some(ScheduleTaskType::AStock),
        }
    }

    fn execute(
        self: std::sync::Arc<Self>,
    ) -> Box<dyn Future<Output = anyhow::Result<()>> + Send + 'static> {
        Box::new(async move {
            let ts = Utc::now();
            if !in_trade_time(&ts) {
                return Ok(());
            }

            let result: Vec<akshare::RealtimeStockMarketRecord> =
                HTTP_CLIENT.get(&self.data_url).send().await?.json().await?;

            let astock_realtime_data_row = result
                .into_iter()
                .map(|record| ch_db::RealtimeStockMarketRecord::from_with_ts(record, ts))
                .collect::<Vec<_>>();

            let mut inserter = CH_CLIENT.inserter("astock_realtime_data")?;
            for row in astock_realtime_data_row {
                inserter.write(&row)?;
            }
            inserter.end().await?;

            Ok(())
        })
    }
}
