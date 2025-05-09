use std::{pin::Pin, sync::Arc};

use chrono::{Duration, Local, NaiveDate, Utc};
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
    repository::{self, FlowDirection, StockAdjustmentType},
    schema,
    utils::{config_backoff, with_base_url},
};

use super::{TRADE_TIME_CRON, in_trade_time};

/// 模块顶级方法，用于暴露给父模块调用将相关调度任务加入到全局调度器之中
pub(super) async fn start_a_stock_tasks(ext_res: ExternalResource) {
    // let realtime_stock_monitor = RealTimeStockMonitor {
    //     data_url: with_base_url("/stock_zh_a_spot_em"),
    //     data_table: "astock_realtime_data".to_owned(),
    //     ext_res: ext_res.clone(),
    // };
    // SCHEDULE_TASK_MANAGER.add_task(realtime_stock_monitor).await;

    let stock_zh_a_hist_monitor = StockZhAHistMonitor {
        data_url: with_base_url("/stock_zh_a_hist"),
        data_table: "stock_zh_a_hist".to_owned(),
        ext_res: ext_res.clone(),
    };
    SCHEDULE_TASK_MANAGER
        .add_task(stock_zh_a_hist_monitor)
        .await;

    let stock_hsgt_hist_em_monitor = StockHsgtHistEmMonitor {
        data_url: with_base_url("/stock_hsgt_hist_em"),
        data_table: "stock_hsgt_hist_em".to_owned(),
        ext_res: ext_res.clone(),
    };
    SCHEDULE_TASK_MANAGER
        .add_task(stock_hsgt_hist_em_monitor)
        .await;

    let stock_zt_pool_em_monitor = StockZtPoolEmMonitor {
        data_url: with_base_url("/stock_zt_pool_em"),
        data_table: "stock_zt_pool_em".to_owned(),
        ext_res: ext_res.clone(),
    };
    SCHEDULE_TASK_MANAGER
        .add_task(stock_zt_pool_em_monitor)
        .await;

    let stock_news_main_cx_monitor = StockNewsMainCxMonitor {
        data_url: with_base_url("/stock_news_main_cx"),
        data_table: "stock_news_main_cx".to_owned(),
        ext_res: ext_res.clone(),
    };
    SCHEDULE_TASK_MANAGER
        .add_task(stock_news_main_cx_monitor)
        .await;

    // StockRankLxszThsMonitor
    let stock_rank_lxsz_ths_monitor = StockRankLxszThsMonitor {
        data_table: "stock_rank_lxsz_ths".to_owned(),
        ext_res: ext_res.clone(),
    };
    SCHEDULE_TASK_MANAGER
        .add_task(stock_rank_lxsz_ths_monitor)
        .await;
}

/// 收集东方财富网-沪深京 A 股-实时行情数据
pub(super) struct RealTimeStockMonitor {
    data_url: String,
    data_table: String,
    ext_res: ExternalResource,
}

impl RealTimeStockMonitor {
    pub async fn collect_data(&self, ts: chrono::DateTime<Utc>) -> anyhow::Result<()> {
        let backoff_s = config_backoff(5, 20);

        let result: Vec<schema::akshare::RealtimeStockMarketRecord> =
            backoff::future::retry(backoff_s, || async {
                let api_data: Vec<schema::akshare::RealtimeStockMarketRecord> = self
                    .ext_res
                    .http_client
                    .get(&self.data_url)
                    .send()
                    .await?
                    .error_for_status()?
                    .json()
                    .await
                    .map_err(backoff::Error::Permanent)?;

                Ok(api_data)
            })
            .await?;

        let astock_realtime_data_row = result
            .into_iter()
            .map(|record| repository::RealtimeStockMarketRecord::from_with_ts(record, ts))
            .collect::<Vec<_>>();

        let mut inserter = self.ext_res.ch_client.inserter(&self.data_table)?;
        for row in astock_realtime_data_row {
            inserter.write(&row)?;
        }
        inserter.end().await?;

        Ok(())
    }
}

// ------------------------------------------------------------------------------------------

/// 收集东方财富-沪深京 A 股日频率数据;
/// 历史数据按日频率更新, 当日收盘价请在收盘后获取
pub(super) struct StockZhAHistMonitor {
    data_url: String,
    data_table: String,
    ext_res: ExternalResource,
}

impl StockZhAHistMonitor {
    async fn req_data_with_type(
        &self,
        code: &str,
        start_date: &str,
        end_date: &str,
        adj_type: StockAdjustmentType,
    ) -> anyhow::Result<Vec<repository::StockZhAHist>> {
        let backoff_s = config_backoff(5, 40);
        let ch_data: Vec<repository::StockZhAHist> = backoff::future::retry(backoff_s, || async {
            let api_data: Vec<schema::akshare::StockZhAHist> = self
                .ext_res
                .http_client
                .get(&self.data_url)
                .query(&[
                    ("symbol", code),
                    ("period", "daily"),
                    ("start_date", start_date),
                    ("end_date", end_date),
                    ("adjust", adj_type.to_str()),
                ])
                .send()
                .await?
                .error_for_status()?
                .json()
                .await
                .map_err(backoff::Error::Permanent)?;

            let now = Utc::now();
            let ch_data = api_data
                .into_iter()
                .map(|value| repository::StockZhAHist::from_with_type(value, adj_type, now))
                .collect::<Vec<repository::StockZhAHist>>();

            Ok(ch_data)
        })
        .await?;

        // use std::io::Write;
        // let mut file = std::fs::OpenOptions::new()
        //     .create(true)
        //     .append(true)
        //     .open("../tmp/output.log")?;
        // let now = Utc::now();
        // writeln!(
        //     file,
        //     "time = {:?}, code = {}, adj_type = {} processed",
        //     now,
        //     code,
        //     adj_type.to_str()
        // );

        Ok(ch_data)
    }

    /// 从akshare请求数据，返回clickhouse schema格式的Vec，包括对应code对应日期之中：`不复权 | 前复权 | 后复权`的整体数据
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
                .then(|adj_type| {
                    self.req_data_with_type(
                        code.as_str(),
                        start_date.as_str(),
                        end_date.as_str(),
                        adj_type,
                    )
                })
                .try_collect()
                .await?;

        Ok(hist_data.into_iter().flatten().collect())
    }

    /// 收集东方财富-沪深京 A 股日频率数据
    pub async fn collect_data(&self) -> anyhow::Result<()> {
        let codes = get_distinct_code(&self.ext_res.ch_client).await?;
        // println!("codes length = {}", codes.len());
        let now_date = Utc::now().with_timezone(&CST);
        let start_date = now_date - chrono::Duration::days(50);

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
            .map(|code| self.req_data(code, start, end))
            .buffer_unordered(64)
            .try_collect()
            .await?;

        let hist_data = hist_data.into_iter().flatten().collect::<Vec<_>>();

        let mut inserter = self.ext_res.ch_client.inserter(&self.data_table)?;
        // .with_option("insert_deduplicate", "1");
        for row in hist_data {
            inserter.write(&row)?;
        }
        inserter.end().await?;

        Ok(())
    }
}

// -----------------------------------------------------------------------------------------

pub struct StockHsgtHistEmMonitor {
    data_url: String,
    data_table: String,
    ext_res: ExternalResource,
}

impl StockHsgtHistEmMonitor {
    async fn get_dir_data(
        &self,
        flow_dir: FlowDirection,
    ) -> anyhow::Result<Vec<repository::StockHsgtHistEm>> {
        let backoff_s = config_backoff(5, 30);
        let api_data = backoff::future::retry(backoff_s, || async {
            let api_data: Vec<schema::akshare::StockHsgtHistEm> = self
                .ext_res
                .http_client
                .get(&self.data_url)
                .query(&[("symbol", flow_dir.as_str())])
                .send()
                .await?
                .error_for_status()?
                .json()
                .await
                .map_err(backoff::Error::Permanent)?;

            Ok(api_data)
        })
        .await?;

        let now = Utc::now();

        Ok(api_data
            .into_iter()
            .map(|value| repository::StockHsgtHistEm::from_with_dir_ts(value, flow_dir, now))
            .collect())
    }

    async fn get_api_data(&self) -> anyhow::Result<Vec<repository::StockHsgtHistEm>> {
        let api_data: Vec<Vec<repository::StockHsgtHistEm>> = stream::iter(FlowDirection::iter())
            .map(|flow_dir| self.get_dir_data(flow_dir))
            .buffer_unordered(2)
            .try_collect()
            .await?;

        Ok(api_data.into_iter().flatten().collect())
    }

    pub async fn collect_data(&self) -> anyhow::Result<()> {
        let rows = self.get_api_data().await?;

        let mut inserter = self.ext_res.ch_client.inserter(&self.data_table)?;
        for row in rows {
            inserter.write(&row)?;
        }
        inserter.end().await?;

        Ok(())
    }
}

// -----------------------------------------------------------------------------------------

pub struct StockZtPoolEmMonitor {
    data_url: String,
    data_table: String,
    ext_res: ExternalResource,
}

impl StockZtPoolEmMonitor {
    /// 获取某一天的所有数据
    async fn get_date_data(
        &self,
        date: NaiveDate,
    ) -> anyhow::Result<Vec<repository::StockZtPoolEm>> {
        let formatted = date.format("%Y%m%d").to_string();
        let backoff_s = config_backoff(5, 30);

        let api_data = backoff::future::retry(backoff_s, || async {
            let api_data: Vec<schema::akshare::StockZtPoolEm> = self
                .ext_res
                .http_client
                .get(&self.data_url)
                .query(&[("date", formatted.as_str())])
                .send()
                .await?
                .error_for_status()?
                .json()
                .await
                .map_err(backoff::Error::Permanent)?;

            Ok(api_data)
        })
        .await?;

        let now = Utc::now();

        Ok(api_data
            .into_iter()
            .map(|value| repository::StockZtPoolEm::from_with_time(value, date, now))
            .collect())
    }

    /// 获取过去两周的所有涨跌停板数据
    async fn get_api_data(&self) -> anyhow::Result<Vec<repository::StockZtPoolEm>> {
        // 获取当前本地时间
        let now = Local::now().date_naive();
        // 计算14天前的日期
        let start_date = now - Duration::weeks(2);
        let dates = (0..=14)
            .map(|diff| start_date + Duration::days(diff))
            .collect::<Vec<_>>();

        let api_data: Vec<Vec<repository::StockZtPoolEm>> = stream::iter(dates)
            .map(|date| self.get_date_data(date))
            .buffer_unordered(8)
            .try_collect()
            .await?;

        Ok(api_data.into_iter().flatten().collect())
    }

    /// 收集数据到clickhouse
    pub async fn collect_data(&self) -> anyhow::Result<()> {
        let rows = self.get_api_data().await?;

        let mut inserter = self.ext_res.ch_client.inserter(&self.data_table)?;
        for row in rows {
            inserter.write(&row)?;
        }
        inserter.end().await?;

        Ok(())
    }
}

// ------------------------------------------------------------------------------------

pub struct StockNewsMainCxMonitor {
    data_url: String,
    data_table: String,
    ext_res: ExternalResource,
}

impl StockNewsMainCxMonitor {
    async fn get_api_data(&self) -> anyhow::Result<Vec<repository::StockNewsMainCx>> {
        let res: Vec<schema::akshare::StockNewsMainCx> = self
            .ext_res
            .http_client
            .get(with_base_url("/stock_news_main_cx"))
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?;

        let now = Utc::now();

        Ok(res
            .into_iter()
            .map(|value| repository::StockNewsMainCx::from_with_ts(value, now))
            .collect())
    }

    pub async fn collect_data(&self) -> anyhow::Result<()> {
        let rows = self.get_api_data().await?;

        let mut inserter = self.ext_res.ch_client.inserter(&self.data_table)?;
        for row in rows {
            inserter.write(&row)?;
        }
        inserter.end().await?;

        Ok(())
    }
}

// ------------------------------------------------------------------------------------

pub struct StockRankLxszThsMonitor {
    data_table: String,
    ext_res: ExternalResource,
}

impl StockRankLxszThsMonitor {
    pub async fn collect_data(&self) -> anyhow::Result<()> {
        let rows =
            schema::akshare::StockRankLxszThs::from_astock_api(&self.ext_res.http_client).await?;

        // 首先删除当前表格之中的所有数据
        let sql = format!("TRUNCATE TABLE {}", self.data_table);
        self.ext_res.ch_client.query(&sql).execute().await?;

        let mut inserter = self.ext_res.ch_client.inserter(&self.data_table)?;
        for row in rows {
            inserter.write(&row)?;
        }
        inserter.end().await?;

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use std::{io::Read, os::fd, str::FromStr};

    use backoff::{Error, ExponentialBackoff, retry};
    use chrono::{Duration, Local, Utc};
    use data_mind::repository::StockAdjustmentType;
    use futures::{StreamExt, stream};
    use serde_json::Value;
    use strum::IntoEnumIterator;

    use crate::{
        init::ExternalResource,
        scheduler::CST,
        tasks::{TEST_CH_CLIENT, TEST_HTTP_CLIENT},
    };

    use super::*;

    #[test]
    fn test_cron() {
        println!(
            "{:?}",
            Utc::now().with_timezone(&CST).format("%Y%m%d").to_string()
        );
        let schedule = cron::Schedule::from_str("0 30 9 * * *").unwrap();
        for next in schedule.upcoming(CST).take(10) {
            println!("{:?}", next);
        }
    }

    #[tokio::test]
    async fn test_realtime_monitor() {
        let ext_res = ExternalResource {
            ch_client: TEST_CH_CLIENT.clone(),
            http_client: TEST_HTTP_CLIENT.clone(),
        };
        let reatime_stock_monitor = RealTimeStockMonitor {
            data_url: with_base_url("/stock_zh_a_spot_em"),
            data_table: "astock_realtime_data".to_owned(),
            ext_res,
        };

        reatime_stock_monitor
            .collect_data(Utc::now())
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn test_stock_zh_a_hist_monitor() {
        let ext_res = ExternalResource {
            ch_client: TEST_CH_CLIENT.clone(),
            http_client: TEST_HTTP_CLIENT.clone(),
        };
        let stock_zh_a_hist_monitor = StockZhAHistMonitor {
            data_url: with_base_url("/stock_zh_a_hist"),
            data_table: "stock_zh_a_hist".to_owned(),
            ext_res,
        };

        stock_zh_a_hist_monitor.collect_data().await.unwrap();
    }

    #[tokio::test]
    async fn test_stock_hsgt_hist_em_monitor() {
        let ext_res = ExternalResource {
            ch_client: TEST_CH_CLIENT.clone(),
            http_client: TEST_HTTP_CLIENT.clone(),
        };

        let stock_hsgt_hist_em_monitor = StockHsgtHistEmMonitor {
            data_url: with_base_url("/stock_hsgt_hist_em"),
            data_table: "stock_hsgt_hist_em".to_owned(),
            ext_res,
        };

        stock_hsgt_hist_em_monitor.collect_data().await.unwrap();
    }

    #[tokio::test]
    async fn test_stock_zt_pool_em_monitor() {
        let ext_res = ExternalResource {
            ch_client: TEST_CH_CLIENT.clone(),
            http_client: TEST_HTTP_CLIENT.clone(),
        };

        let stock_zt_pool_em_monitor = StockZtPoolEmMonitor {
            data_url: with_base_url("/stock_zt_pool_em"),
            data_table: "stock_zt_pool_em".to_owned(),
            ext_res,
        };

        stock_zt_pool_em_monitor.collect_data().await.unwrap();
    }

    #[tokio::test]
    async fn test_stock_news_main_cx_monitor() {
        let ext_res = ExternalResource {
            ch_client: TEST_CH_CLIENT.clone(),
            http_client: TEST_HTTP_CLIENT.clone(),
        };

        let stock_news_main_cx_monitor = StockNewsMainCxMonitor {
            data_url: with_base_url("/stock_news_main_cx"),
            data_table: "stock_news_main_cx".to_owned(),
            ext_res,
        };

        stock_news_main_cx_monitor.collect_data().await.unwrap();
    }

    #[tokio::test]
    async fn test_stock_rank_lxsz_ths_monitor() {
        let ext_res = ExternalResource {
            ch_client: TEST_CH_CLIENT.clone(),
            http_client: TEST_HTTP_CLIENT.clone(),
        };

        let stock_rank_lxsz_ths_monitor = StockRankLxszThsMonitor {
            data_table: "stock_rank_lxsz_ths".to_owned(),
            ext_res: ext_res.clone(),
        };

        stock_rank_lxsz_ths_monitor.collect_data().await.unwrap();
    }

    #[test]
    fn test_past_week() {
        // 获取当前本地时间
        let now = Local::now().date_naive();

        // 计算7天前的日期
        let start_date = now - Duration::weeks(2);

        // 从7天前到今天，每天循环
        for i in 0..=14 {
            let date = start_date + Duration::days(i);
            let formatted = date.format("%Y%m%d").to_string();
            println!("{}", formatted);
        }
    }
}
