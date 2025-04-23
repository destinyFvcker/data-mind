use chrono::Utc;

use crate::{
    init::ExternalResource,
    monitor_tasks::utils::get_distinct_code,
    scheduler::{SCHEDULE_TASK_MANAGER, Schedulable, ScheduleTaskType, TaskMeta},
};
use data_mind::{
    repository::{self, StockAdjustmentType},
    schema,
};

use super::{TRADE_TIME_CRON, in_trade_time, with_base_url};

/// 模块顶级方法，用于暴露给父模块调用将相关调度任务加入到全局调度器之中
pub(super) async fn start_a_stock_tasks(ext_res: ExternalResource) {
    let realtime_stock_monitor = RealTimeStockMonitor {
        data_url: with_base_url("/stock_zh_a_spot_em"),
        ext_res: ext_res.clone(),
    };
    SCHEDULE_TASK_MANAGER.add_task(realtime_stock_monitor).await;
}

/// 收集东方财富网-沪深京 A 股-实时行情数据
struct RealTimeStockMonitor {
    data_url: String,
    ext_res: ExternalResource,
}

/// 收集东方财富-沪深京 A 股日频率数据;
/// 历史数据按日频率更新, 当日收盘价请在收盘后获取
struct StockZhAHistMonitor {
    data_url: String,
    ext_res: ExternalResource,
}

impl RealTimeStockMonitor {
    async fn collect_data(&self, ts: chrono::DateTime<Utc>) -> anyhow::Result<()> {
        let result: Vec<schema::RealtimeStockMarketRecord> = self
            .ext_res
            .http_client
            .get(&self.data_url)
            .send()
            .await?
            .json()
            .await?;

        let astock_realtime_data_row = result
            .into_iter()
            .map(|record| repository::RealtimeStockMarketRecord::from_with_ts(record, ts))
            .collect::<Vec<_>>();

        let mut inserter = self.ext_res.ch_client.inserter("astock_realtime_data")?;
        for row in astock_realtime_data_row {
            inserter.write(&row)?;
        }
        inserter.end().await?;

        Ok(())
    }
}

impl StockZhAHistMonitor {
    /// 从akshare请求数据，注意date_str是一个 yyyymmdd 格式的时间字符串
    async fn req_data(
        &self,
        code: &str,
        adj_type: StockAdjustmentType,
        date_str: &str,
    ) -> anyhow::Result<Vec<schema::StockZhAHist>> {
        let api_data: Vec<schema::StockZhAHist> = self
            .ext_res
            .http_client
            .get(&self.data_url)
            .query(&[
                ("symbol", code),
                ("period", "daily"),
                ("start_date", "00000000"),
                ("end_date", date_str),
                ("adjust", adj_type.to_str()),
            ])
            .send()
            .await?
            .json()
            .await?;

        Ok(api_data)
    }

    /// 收集东方财富-沪深京 A 股日频率数据
    async fn collect_data(&self) -> anyhow::Result<()> {
        let codes = get_distinct_code(&self.ext_res.ch_client).await?;

        // let futs = codes
        //     .into_iter()
        //     .map(|code| async {
        //         let api_data: Vec<schema::StockZhAHist> = self
        //             .ext_res
        //             .http_client
        //             .get(&self.data_url)
        //             .send()
        //             .await?
        //             .json()
        //             .await?;

        //         return api_data;
        //     })
        //     .collect::<Vec<_>>();

        // let stock_zh_a_hist: Vec<repository::StockZhAHist>

        todo!()
    }
}

impl Schedulable for StockZhAHistMonitor {
    fn gen_meta(&self) -> TaskMeta {
        TaskMeta {
            name: "stock_zh_a_hist".to_owned(),
            desc:
                "收集东方财富-沪深京 A 股日频率数据; 历史数据按日频率更新, 当日收盘价请在收盘后获取"
                    .to_owned(),
            cron_expr: "0 30 15 * * MON-FRI".to_owned(),
            tag: Some(ScheduleTaskType::AStock),
        }
    }

    fn execute(
        self: std::sync::Arc<Self>,
    ) -> Box<dyn Future<Output = anyhow::Result<()>> + Send + 'static> {
        todo!()
    }
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
            self.collect_data(ts).await?;
            Ok(())
        })
    }
}

#[cfg(test)]
mod test {
    use std::str::FromStr;

    use chrono::Utc;

    use crate::{
        init::ExternalResource,
        monitor_tasks::{TEST_CH_CLIENT, TEST_HTTP_CLIENT, with_base_url},
        scheduler::CST,
    };

    use super::RealTimeStockMonitor;

    #[test]
    fn test_cron() {
        let schedule = cron::Schedule::from_str("0 0 17 * * MON-FRI").unwrap();
        for next in schedule.upcoming(CST).take(10) {
            println!("{:?}", next);
        }
    }

    #[tokio::test]
    async fn test_realtime_collect_data() {
        let ext_res = ExternalResource {
            ch_client: TEST_CH_CLIENT.clone(),
            http_client: TEST_HTTP_CLIENT.clone(),
        };
        let reatime_stock_monitor = RealTimeStockMonitor {
            data_url: with_base_url("/stock_zh_a_spot_em"),
            ext_res,
        };

        reatime_stock_monitor
            .collect_data(Utc::now())
            .await
            .unwrap();
    }
}
