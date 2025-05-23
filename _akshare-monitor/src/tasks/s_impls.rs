//! 任务的schedulable trait实现

use chrono::Utc;

use crate::scheduler::{Schedulable, ScheduleTaskType, TaskMeta};

use super::{
    TRADE_TIME_CRON,
    a_index::{IndexOption50EtfQvixMonitor, IndexStockInfoMonitor, StockZhIndexDailyMonitor},
    a_stock::{
        RealTimeStockMonitor, StockHsgtHistEmMonitor, StockNewsMainCxMonitor,
        StockRankLxszThsMonitor, StockZhAHistMonitor, StockZtPoolEmMonitor,
    },
    in_trade_time,
};

impl Schedulable for StockZhAHistMonitor {
    fn gen_meta(&self) -> TaskMeta {
        TaskMeta {
            name: "stock_zh_a_hist".to_owned(),
            desc:
                "收集东方财富-沪深京 A 股日频率数据; 历史数据按日频率更新, 当日收盘价请在收盘后获取"
                    .to_owned(),
            cron_expr: "0 0 16 * * MON-FRI".to_owned(),
            tag: Some(ScheduleTaskType::AStock),
        }
    }

    fn execute(
        self: std::sync::Arc<Self>,
    ) -> Box<dyn Future<Output = anyhow::Result<()>> + Send + 'static> {
        Box::new(async move {
            self.collect_data().await?;
            Ok(())
        })
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

impl Schedulable for StockZhIndexDailyMonitor {
    fn gen_meta(&self) -> TaskMeta {
        TaskMeta {
            name: "stock_zh_index_daily".to_owned(),
            desc: "历史行情数据-新浪, 股票指数的历史数据按日频率更新".to_owned(),
            cron_expr: "0 50 18 * * MON-FRI".to_owned(),
            tag: Some(ScheduleTaskType::Index),
        }
    }

    fn execute(
        self: std::sync::Arc<Self>,
    ) -> Box<dyn Future<Output = anyhow::Result<()>> + Send + 'static> {
        Box::new(async move {
            self.collect_data().await?;
            Ok(())
        })
    }
}

impl Schedulable for IndexOption50EtfQvixMonitor {
    fn gen_meta(&self) -> TaskMeta {
        TaskMeta {
            name: "index_option_50etf_qvix".to_owned(),
            desc: "50ETF 期权波动率指数 QVIX; 又称中国版的恐慌指数".to_owned(),
            cron_expr: "0 30 15 * * MON-FRI".to_owned(),
            tag: Some(ScheduleTaskType::Index),
        }
    }

    fn execute(
        self: std::sync::Arc<Self>,
    ) -> Box<dyn Future<Output = anyhow::Result<()>> + Send + 'static> {
        Box::new(async move {
            self.collect_data().await?;
            Ok(())
        })
    }
}

impl Schedulable for StockHsgtHistEmMonitor {
    fn gen_meta(&self) -> TaskMeta {
        TaskMeta {
            name: "stock_hsgt_hist_em".to_owned(),
            desc: "东方财富网-数据中心-资金流向-沪深港通资金流向-沪深港通历史数据".to_owned(),
            cron_expr: "0 30 15 * * MON-FRI".to_owned(),
            tag: Some(ScheduleTaskType::AStock),
        }
    }

    fn execute(
        self: std::sync::Arc<Self>,
    ) -> Box<dyn Future<Output = anyhow::Result<()>> + Send + 'static> {
        Box::new(async move {
            self.collect_data().await?;
            Ok(())
        })
    }
}

impl Schedulable for StockZtPoolEmMonitor {
    fn gen_meta(&self) -> TaskMeta {
        TaskMeta {
            name: "stock_zt_pool_em".to_owned(),
            desc: "东方财富网-行情中心-涨停板行情-涨停股池".to_owned(),
            cron_expr: "0 30 15 * * MON-FRI".to_owned(),
            tag: Some(ScheduleTaskType::AStock),
        }
    }

    fn execute(
        self: std::sync::Arc<Self>,
    ) -> Box<dyn Future<Output = anyhow::Result<()>> + Send + 'static> {
        Box::new(async move {
            self.collect_data().await?;
            Ok(())
        })
    }
}

impl Schedulable for StockNewsMainCxMonitor {
    fn gen_meta(&self) -> TaskMeta {
        TaskMeta {
            name: "stock_news_main_cx".to_owned(),
            desc: "财新网-财新数据通-内容精选".to_owned(),
            cron_expr: "0 52 21 * * *".to_owned(),
            tag: Some(ScheduleTaskType::AStock),
        }
    }

    fn execute(
        self: std::sync::Arc<Self>,
    ) -> Box<dyn Future<Output = anyhow::Result<()>> + Send + 'static> {
        Box::new(async move {
            self.collect_data().await?;
            Ok(())
        })
    }
}

impl Schedulable for StockRankLxszThsMonitor {
    fn gen_meta(&self) -> TaskMeta {
        TaskMeta {
            name: "stock_rank_lxsz_ths".to_owned(),
            desc: "同花顺-数据中心-技术选股-连续上涨".to_owned(),
            cron_expr: "0 3 17 * * *".to_owned(),
            tag: Some(ScheduleTaskType::AStock),
        }
    }

    fn execute(
        self: std::sync::Arc<Self>,
    ) -> Box<dyn Future<Output = anyhow::Result<()>> + Send + 'static> {
        Box::new(async move {
            self.collect_data().await?;
            Ok(())
        })
    }
}

impl Schedulable for IndexStockInfoMonitor {
    fn gen_meta(&self) -> TaskMeta {
        TaskMeta {
            name: "index_stock_info".to_owned(),
            desc: "收集股票指数信息一览表".to_owned(),
            cron_expr: "0 0 12 * * SAT".to_owned(),
            tag: Some(ScheduleTaskType::Index),
        }
    }

    fn execute(
        self: std::sync::Arc<Self>,
    ) -> Box<dyn Future<Output = anyhow::Result<()>> + Send + 'static> {
        Box::new(async move {
            self.collect_data().await?;
            Ok(())
        })
    }
}
