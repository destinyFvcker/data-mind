use chrono::Utc;
use data_mind::{
    repository::{self, IndexStockInfo},
    schema,
    utils::{config_backoff, with_base_url},
};
use futures::{StreamExt, TryStreamExt, stream};

use crate::{init::ExternalResource, scheduler::SCHEDULE_TASK_MANAGER};

pub async fn start_a_index_tasks(ext_res: ExternalResource) {
    let stock_zh_index_daily_monitor = StockZhIndexDailyMonitor {
        codes_url: with_base_url("/stock_zh_index_spot_sina"),
        data_url: with_base_url("/stock_zh_index_daily"),
        data_table: "stock_zh_index_daily".to_string(),
        ext_res: ext_res.clone(),
    };
    SCHEDULE_TASK_MANAGER
        .add_task(stock_zh_index_daily_monitor)
        .await;

    let index_option_50etf_qvix = IndexOption50EtfQvixMonitor {
        data_url: with_base_url("/index_option_50etf_qvix"),
        data_table: "index_option_50etf_qvix".to_owned(),
        ext_res: ext_res.clone(),
    };
    SCHEDULE_TASK_MANAGER
        .add_task(index_option_50etf_qvix)
        .await;

    let index_stock_info_monitor = IndexStockInfoMonitor {
        data_url: with_base_url("/index_stock_info"),
        data_table: "index_stock_info".to_owned(),
        ext_res: ext_res.clone(),
    };
    SCHEDULE_TASK_MANAGER
        .add_task(index_stock_info_monitor)
        .await;
}

/// 收集历史行情数据-新浪
pub struct StockZhIndexDailyMonitor {
    /// 获取指数数据代码的url
    codes_url: String,
    /// 获取历史数据的url
    data_url: String,
    /// 对接的数据库表名
    data_table: String,
    /// 相关外部资源（数据库连接池等等）
    ext_res: ExternalResource,
}

impl StockZhIndexDailyMonitor {
    /// 获取所有指数数据代码
    async fn get_codes(&self) -> anyhow::Result<Vec<String>> {
        let values: Vec<schema::akshare::StockZhIndexSpotSina> = self
            .ext_res
            .http_client
            .get(&self.codes_url)
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?;

        Ok(values.into_iter().map(|value| value.code).collect())
    }

    /// 获取单个code对应的指数的日频历史行情数据
    async fn get_daily_hist(
        &self,
        code: String,
    ) -> anyhow::Result<Vec<repository::StockZhIndexDaily>> {
        let backoff_s = config_backoff(5, 30);
        let api_data = backoff::future::retry(backoff_s, || async {
            let api_data: Vec<schema::akshare::StockZhIndexDaily> = self
                .ext_res
                .http_client
                .get(&self.data_url)
                .query(&[("symbol", code.as_str())])
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
            .map(|data| repository::StockZhIndexDaily::from_with_ts(data, &code, now))
            .collect())
    }

    /// 通过股票代码获取所有指数历史数据
    async fn get_daily_hists(
        &self,
        codes: Vec<String>,
    ) -> anyhow::Result<Vec<repository::StockZhIndexDaily>> {
        let res: Vec<Vec<repository::StockZhIndexDaily>> = stream::iter(codes)
            .map(|code| self.get_daily_hist(code))
            .buffer_unordered(50)
            .try_collect()
            .await?;

        Ok(res.into_iter().flatten().collect())
    }

    pub async fn collect_data(&self) -> anyhow::Result<()> {
        let codes = self.get_codes().await?;
        let daily_hists = self.get_daily_hists(codes).await?;

        let mut inserter = self.ext_res.ch_client.inserter(&self.data_table)?;
        for row in daily_hists {
            inserter.write(&row)?;
        }
        inserter.end().await?;

        Ok(())
    }
}

// ------------------------------------------------------------------------

pub struct IndexOption50EtfQvixMonitor {
    data_url: String,
    data_table: String,
    ext_res: ExternalResource,
}

impl IndexOption50EtfQvixMonitor {
    async fn get_data(&self) -> anyhow::Result<Vec<Option<repository::IndexOption50EtfQvix>>> {
        let backoff_s = config_backoff(5, 30);
        let api_data = backoff::future::retry(backoff_s, || async {
            let api_data: Vec<schema::akshare::IndexOption50EtfQvix> = self
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

        let now = Utc::now();
        let repo_data = api_data
            .into_iter()
            .map(|data| repository::IndexOption50EtfQvix::from_with_ts(data, now))
            .collect::<Vec<_>>();

        Ok(repo_data)
    }

    pub async fn collect_data(&self) -> anyhow::Result<()> {
        let rows_with_none = self.get_data().await?;
        let rows = rows_with_none
            .into_iter()
            .filter_map(|row| row)
            .collect::<Vec<_>>();

        let mut inserter = self.ext_res.ch_client.inserter(&self.data_table)?;
        for row in rows {
            inserter.write(&row)?;
        }
        inserter.end().await?;

        Ok(())
    }
}

pub struct IndexStockInfoMonitor {
    data_url: String,
    data_table: String,
    ext_res: ExternalResource,
}

impl IndexStockInfoMonitor {
    pub async fn collect_data(&self) -> anyhow::Result<()> {
        let rows = IndexStockInfo::from_astock_api(&self.ext_res.http_client).await?;

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
    use super::*;
    use crate::{
        init::ExternalResource,
        tasks::{TEST_CH_CLIENT, TEST_HTTP_CLIENT},
    };

    #[tokio::test]
    async fn test_stock_zh_index_daily_monitor() {
        let ext_res = ExternalResource {
            ch_client: TEST_CH_CLIENT.clone(),
            http_client: TEST_HTTP_CLIENT.clone(),
        };

        let stock_zh_index_daily_monitor = StockZhIndexDailyMonitor {
            codes_url: with_base_url("/stock_zh_index_spot_sina"),
            data_url: with_base_url("/stock_zh_index_daily"),
            data_table: "stock_zh_index_daily".to_string(),
            ext_res,
        };

        stock_zh_index_daily_monitor.collect_data().await.unwrap()
    }

    #[tokio::test]
    async fn test_index_option_50etf_qvix_monitor() {
        let ext_res = ExternalResource {
            ch_client: TEST_CH_CLIENT.clone(),
            http_client: TEST_HTTP_CLIENT.clone(),
        };

        let index_option_50etf_qvix = IndexOption50EtfQvixMonitor {
            data_url: with_base_url("/index_option_50etf_qvix"),
            data_table: "index_option_50etf_qvix".to_owned(),
            ext_res,
        };

        index_option_50etf_qvix.collect_data().await.unwrap()
    }
}
