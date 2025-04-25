use std::{pin::Pin, sync::Arc};

use chrono::Utc;
use futures::{
    FutureExt, StreamExt, TryStreamExt,
    stream::{self, FuturesUnordered},
};
use strum::IntoEnumIterator;

use crate::{
    init::ExternalResource,
    scheduler::{CST, SCHEDULE_TASK_MANAGER, Schedulable, ScheduleTaskType, TaskMeta},
    tasks::utils::get_distinct_code,
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
pub(super) struct RealTimeStockMonitor {
    data_url: String,
    ext_res: ExternalResource,
}

/// 收集东方财富-沪深京 A 股日频率数据;
/// 历史数据按日频率更新, 当日收盘价请在收盘后获取
pub(super) struct StockZhAHistMonitor {
    data_url: String,
    ext_res: ExternalResource,
}

impl RealTimeStockMonitor {
    pub async fn collect_data(&self, ts: chrono::DateTime<Utc>) -> anyhow::Result<()> {
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
    /// 从akshare请求数据，返回clickhouse schema格式的Vec，
    /// 包括对应code对应日期之中：`不复权 | 前复权 | 后复权`的整体数据
    /// 注意date_str是一个 yyyymmdd 格式的时间字符串
    async fn req_data(
        &self,
        code: String,
        start_date: u32,
        end_date: u32,
    ) -> anyhow::Result<Vec<repository::StockZhAHist>> {
        let start_date = start_date.to_string();
        let end_date = end_date.to_string();

        let hist_data: Vec<Vec<repository::StockZhAHist>> =
            stream::iter(StockAdjustmentType::iter())
                .then(async |adj_type| {
                    let res = self
                        .ext_res
                        .http_client
                        .get(&self.data_url)
                        .query(&[
                            ("symbol", code.as_str()),
                            ("period", "daily"),
                            ("start_date", start_date.as_str()),
                            ("end_date", end_date.as_str()),
                            ("adjust", adj_type.to_str()),
                        ])
                        .send()
                        .await
                        .inspect_err(|err| println!("error await send = {:?}", err))?
                        .error_for_status()?;

                    let api_data: Vec<schema::StockZhAHist> = res
                        .json()
                        .await
                        .inspect_err(|err| println!("error deser json data = {:?}", err))?;
                    let ch_data = api_data
                        .into_iter()
                        .map(|value| repository::StockZhAHist::from_with_type(value, adj_type))
                        .collect::<Vec<repository::StockZhAHist>>();

                    Ok::<Vec<data_mind::repository::StockZhAHist>, anyhow::Error>(ch_data)
                })
                .try_collect()
                .await?;

        Ok(hist_data.into_iter().flatten().collect())
    }

    /// 收集东方财富-沪深京 A 股日频率数据
    pub async fn collect_data(&self) -> anyhow::Result<()> {
        let codes = get_distinct_code(&self.ext_res.ch_client).await?;
        println!("codes length = {}", codes.len());
        let now_date = Utc::now().with_timezone(&CST);
        let start_date = now_date - chrono::Duration::days(7);

        let end = now_date
            .format("%Y%m%d")
            .to_string()
            .parse::<u32>()
            .unwrap();
        let start = start_date
            .format("%Y%m%d")
            .to_string()
            .parse::<u32>()
            .unwrap();

        let hist_data: Vec<Vec<repository::StockZhAHist>> = stream::iter(codes)
            .take(3)
            .map(|code| self.req_data(code, start, end))
            .buffer_unordered(10)
            .try_collect()
            .await?;

        let hist_data = hist_data.into_iter().flatten().collect::<Vec<_>>();

        let mut inserter = self
            .ext_res
            .ch_client
            .inserter("stock_zh_a_hist")?
            .with_option("insert_deduplicate", "1");
        for row in hist_data {
            inserter.write(&row)?;
        }
        inserter.end().await?;

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use std::str::FromStr;

    use chrono::Utc;
    use data_mind::repository::StockAdjustmentType;
    use futures::{StreamExt, stream};
    use strum::IntoEnumIterator;

    use crate::{
        init::ExternalResource,
        scheduler::CST,
        tasks::{TEST_CH_CLIENT, TEST_HTTP_CLIENT, a_stock::StockZhAHistMonitor, with_base_url},
    };

    use super::RealTimeStockMonitor;

    #[test]
    fn test_cron() {
        println!(
            "{:?}",
            Utc::now().with_timezone(&CST).format("%Y%m%d").to_string()
        );
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

    #[tokio::test]
    async fn test_stock_zh_a_hist_collect_data() {
        let ext_res = ExternalResource {
            ch_client: TEST_CH_CLIENT.clone(),
            http_client: TEST_HTTP_CLIENT.clone(),
        };
        let stock_zh_a_hist_monitor = StockZhAHistMonitor {
            data_url: with_base_url("/stock_zh_a_hist"),
            ext_res,
        };

        stock_zh_a_hist_monitor.collect_data().await.unwrap();
    }

    #[tokio::test]
    async fn test_steam1() {
        let stream = stream::iter(StockAdjustmentType::iter())
            .collect::<Vec<StockAdjustmentType>>()
            .await;

        println!("{:?}", stream);
    }

    #[tokio::test]
    async fn test_try_fold() {
        use futures::channel::mpsc;
        use futures::stream::{StreamExt, TryStreamExt};
        use std::thread;

        let (tx1, rx1) = mpsc::unbounded();
        let (tx2, rx2) = mpsc::unbounded();
        let (tx3, rx3) = mpsc::unbounded();

        thread::spawn(move || {
            tx1.unbounded_send(Ok(1)).unwrap();
        });
        thread::spawn(move || {
            tx2.unbounded_send(Ok(2)).unwrap();
            tx2.unbounded_send(Err(3)).unwrap();
            tx2.unbounded_send(Ok(4)).unwrap();
        });
        thread::spawn(move || {
            tx3.unbounded_send(Ok(rx1)).unwrap();
            tx3.unbounded_send(Ok(rx2)).unwrap();
            tx3.unbounded_send(Err(5)).unwrap();
        });

        let mut stream = rx3.try_flatten();
        assert_eq!(stream.next().await, Some(Ok(1)));
        assert_eq!(stream.next().await, Some(Ok(2)));
        assert_eq!(stream.next().await, Some(Err(3)));
        assert_eq!(stream.next().await, Some(Ok(4)));
        assert_eq!(stream.next().await, Some(Err(5)));
        assert_eq!(stream.next().await, None);
    }
}
