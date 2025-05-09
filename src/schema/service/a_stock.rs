//! A股相关数据dto实体定义

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::repository::{DailyIndicatorRepo, DailyKlineRepo, DailyTradingVolumeRepo, MALinesRepo};
pub use crate::schema::akshare::StockIndividualInfoEm;

/// 移动平均线数据(MA5/MA10/MA20)
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct MALines {
    /// 数据点日期，格式为YYYY-MM-DD
    #[schema(example = "2025-05-08")]
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

impl From<MALinesRepo> for MALines {
    fn from(value: MALinesRepo) -> Self {
        Self {
            date: value.date,
            ma5: value.ma5,
            ma10: value.ma10,
            ma20: value.ma20,
        }
    }
}

/// 日频K线数据
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct DailyKline {
    /// 数据日期，格式为YYYY-MM-DD
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

impl From<DailyKlineRepo> for DailyKline {
    fn from(value: DailyKlineRepo) -> Self {
        Self {
            date: value.date,
            open: value.open,
            close: value.low,
            high: value.high,
            low: value.low,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct DailyTradingVolume {
    /// 数据日期，格式为YYYY-MM-DD
    pub date: NaiveDate,
    /// 交易量(手)
    pub trading_volume: f64,
}

impl From<DailyTradingVolumeRepo> for DailyTradingVolume {
    fn from(value: DailyTradingVolumeRepo) -> Self {
        Self {
            date: value.date,
            trading_volume: value.trading_volume,
        }
    }
}

/// 日频其它指标数据
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct DailyIndicator {
    /// 数据日期，格式为YYYY-MM-DD
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

impl From<DailyIndicatorRepo> for DailyIndicator {
    fn from(value: DailyIndicatorRepo) -> Self {
        Self {
            date: value.date,
            trading_value: value.trading_value,
            amplitude: value.amplitude,
            turnover_rate: value.turnover_rate,
            change_percent: value.change_percent,
            change_amount: value.change_amount,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::utils::TEST_CH_CLIENT;

    use super::*;

    #[tokio::test]
    async fn test_fetch_malines() {
        let data: Vec<MALines> = MALinesRepo::fetch_with_limit(&TEST_CH_CLIENT, "603777", 90)
            .await
            .unwrap()
            .into_iter()
            .map(From::from)
            .collect();

        println!("{}", serde_json::to_string_pretty(&data).unwrap());
    }
}
