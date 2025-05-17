//! A股相关数据dto实体定义

use chrono::NaiveDate;
use clickhouse::Row;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// A股移动平均线数据(MA5/MA10/MA20)
#[derive(Debug, Serialize, Deserialize, Row, ToSchema)]
pub struct StockMALines {
    /// 数据点日期，格式为YYYY-MM-DD
    #[schema(example = "2025-05-08")]
    #[serde(deserialize_with = "clickhouse::serde::chrono::date::deserialize")]
    pub date: NaiveDate,
    /// 数据点日期对应的MA5值，注意单位(元)
    #[schema(example = 13.29)]
    pub ma5: Option<f64>,
    /// 数据点日期对应的MA10值。注意单位(元)
    #[schema(example = 13.481)]
    pub ma10: Option<f64>,
    /// 数据点日期对应的MA20值，注意单位(元)
    #[schema(example = 13.8955)]
    pub ma20: Option<f64>,
}

/// A股日频K线数据
#[derive(Debug, Serialize, Deserialize, Row, ToSchema)]
pub struct StockDailyKline {
    /// 数据日期，格式为YYYY-MM-DD
    #[serde(deserialize_with = "clickhouse::serde::chrono::date::deserialize")]
    pub date: NaiveDate,
    /// 开盘价(元)
    pub open: f64,
    /// 收盘价(元)
    pub close: f64,
    /// 最高价(元)
    pub high: f64,
    /// 最低价(元)
    pub low: f64,
}

/// A股日频成交量数据
#[derive(Debug, Serialize, Deserialize, Row, ToSchema)]
pub struct StockDailyTradingVolume {
    /// 数据日期，格式为YYYY-MM-DD
    #[serde(deserialize_with = "clickhouse::serde::chrono::date::deserialize")]
    pub date: NaiveDate,
    /// 交易量(手)
    pub trading_volume: f64,
}

/// A股日频其它指标数据
#[derive(Debug, Serialize, Deserialize, Row, ToSchema)]
pub struct StockDailyIndicator {
    /// 数据日期，格式为YYYY-MM-DD
    #[serde(deserialize_with = "clickhouse::serde::chrono::date::deserialize")]
    pub date: NaiveDate,
    /// 成交额,注意单位(元)
    pub trading_value: f64,
    /// 振幅(%)
    pub amplitude: f64,
    /// 换手率(%)
    pub turnover_rate: f64,
    /// 涨跌幅(%)
    pub change_percent: f64,
    /// 涨跌额,注意单位(元)
    pub change_amount: f64,
}

/// A股日频分页数据
#[derive(Debug, Serialize, Deserialize, Row, ToSchema)]
pub struct StockDailyPagin {
    /// 股票代码
    pub code: String,
    /// 开盘价(元)
    pub open: f64,
    /// 收盘价(元)
    pub close: f64,
    /// 最低价(元)
    pub low: f64,
    /// 最高价(元)
    pub high: f64,
    /// 成交量,注意单位(手)
    pub trading_volume: f64,
    /// 成交额,注意单位(元)
    pub trading_value: f64,
    /// 振幅(%)
    pub amplitude: f64,
    /// 换手率(%)
    pub turnover_rate: f64,
    /// 涨跌幅(%)
    pub change_percentage: f64,
    /// 涨跌额,注意单位(元)
    pub change_amount: f64,
    /// 最近的数据更新日期，格式为YYYY-MM-DD
    #[serde(deserialize_with = "clickhouse::serde::chrono::date::deserialize")]
    pub date: NaiveDate,
}

impl StockDailyPagin {
    #[allow(unused)]
    fn is_sortable_filed(filed: &str) -> bool {
        let sortable_fileds = [
            "open",
            "close",
            "low",
            "high",
            "trading_volume",
            "trading_value",
            "amplitude",
            "turnover_rate",
            "change_percentage",
            "change_amount",
        ];

        sortable_fileds
            .iter()
            .position(|&sortable| sortable == filed)
            .is_some()
    }
}

/// 东方财富网-数据中心-资金流向-沪深港通资金流向-沪深港通历史数据，  
/// 分为南向北向
#[derive(Debug, Serialize, Deserialize, Row, ToSchema)]
pub struct StockHsgtHistEm {
    /// 资金流动方向(0代表北向，1代表南向)
    // pub flow_dir: FlowDirection,
    /// 买入成交额，单位：亿元
    pub buy_amount: f64,
    /// 卖出成交额，单位：亿元
    pub sell_amount: f64,
    /// 历史累计净买额，单位：万亿元
    pub historical_net_buy_amount: f64,
    /// 当日余额，单位：亿元
    pub daily_balance: f64,
    /// 当日成交净买额，单位：亿元
    pub daily_net_buy_amount: f64,
    /// 当日资金流入，单位：亿元
    pub daily_inflow: f64,
    /// 持股市值，单位：元
    pub holding_market_value: f64,
    /// 沪深300指数点位
    pub hs300_index: f64,
    /// 沪深300指数涨跌幅，单位：%
    pub hs300_change_percent: f64,
    /// 领涨股名称
    pub leading_stock_name: String,
    /// 领涨股代码，例如 "600198.SH"
    pub leading_stock_code: String,
    /// 领涨股涨跌幅，单位：%
    pub leading_stock_change_percent: f64,
    /// 日期，格式："YYYY-MM-DD"
    #[serde(deserialize_with = "clickhouse::serde::chrono::date::deserialize")]
    pub date: NaiveDate,
}

#[cfg(test)]
mod test {
    use crate::utils::TEST_CH_CLIENT;

    use super::*;

    #[tokio::test]
    async fn test_fetch_malines() {
        let data: Vec<StockMALines> = StockMALines::fetch_with_limit(&TEST_CH_CLIENT, "603777", 90)
            .await
            .unwrap()
            .into_iter()
            .map(From::from)
            .collect();

        println!("{}", serde_json::to_string_pretty(&data).unwrap());
    }
}
