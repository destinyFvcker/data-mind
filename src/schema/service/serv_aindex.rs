use chrono::NaiveDate;
use clickhouse::Row;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// 50ETF 期权波动率指数 QVIX; 又称中国版的恐慌指数 日频K线数据
#[derive(Debug, Deserialize, Serialize, Row, ToSchema)]
pub struct IndexOption50EtfQvixKline {
    /// 开盘价
    pub open: f64,
    /// 收盘价
    pub close: f64,
    /// 最高价
    pub high: f64,
    /// 最低价
    pub low: f64,
    /// 数据日期，格式为YYY-MM-DD
    #[serde(deserialize_with = "clickhouse::serde::chrono::date::deserialize")]
    pub date: NaiveDate,
}

/// 50ETF 期权波动率指数 QVIX; 日频移动平均线数据
#[derive(Debug, Serialize, Deserialize, Row, ToSchema)]
pub struct IndexOption50EtfQvixMA {
    /// 数据日期，格式为YYYY-DD-MM
    #[serde(deserialize_with = "clickhouse::serde::chrono::date::deserialize")]
    pub date: NaiveDate,
    /// 对应数据日期的5日平均线数据
    pub ma5: Option<f64>,
    /// 对应数据日期的10日平均线数据
    pub ma10: Option<f64>,
    /// 对应数据日期的20日平均线数据
    pub ma20: Option<f64>,
}

/// A股指数日频K线数据
#[derive(Debug, Deserialize, Serialize, Row, ToSchema)]
pub struct StockZhIndexDailyKline {
    /// 开盘价(元)
    pub open: f64,
    /// 收盘价(元)
    pub close: f64,
    /// 最高价(元)
    pub high: f64,
    /// 最低价(元)
    pub low: f64,
    /// 交易量(手)
    pub volume: f64,
    /// 数据日期，格式为YYYY-MM-DD
    #[serde(deserialize_with = "clickhouse::serde::chrono::date::deserialize")]
    pub date: NaiveDate,
}

/// A股指数日频交易量数据
#[derive(Debug, Deserialize, Serialize, Row, ToSchema)]
pub struct StockZhIndexDailyVolume {
    /// 交易日期
    #[serde(deserialize_with = "clickhouse::serde::chrono::date::deserialize")]
    pub date: NaiveDate,
    /// 交易量
    pub volume: f64,
}

/// A股指数日频移动平均线数据
#[derive(Debug, Serialize, Deserialize, Row, ToSchema)]
pub struct StockZhIndexDailyMA {
    /// 数据日期，格式为YYYY-MM-DD
    #[serde(deserialize_with = "clickhouse::serde::chrono::date::deserialize")]
    pub date: NaiveDate,
    /// 对应(指数代码, 数据日期)的5日移动平均线数据
    pub ma5: Option<f64>,
    /// 对应(指数代码, 数据日期)的10日移动平均线数据
    pub ma10: Option<f64>,
    /// 对应(指数代码, 数据日期)的20日移动平均线数据
    pub ma20: Option<f64>,
}

/// A股指数日频分页数据
#[derive(Debug, Deserialize, Serialize, Row, ToSchema)]
pub struct StockZhIndexDailyPagin {
    /// 指数代码
    pub code: String,
    /// 最新数据时间，格式为YYYY-MM-DD
    #[serde(deserialize_with = "clickhouse::serde::chrono::date::deserialize")]
    pub date: NaiveDate,
    /// 开盘价
    pub open: f64,
    /// 收盘价
    pub close: f64,
    /// 最高价
    pub high: f64,
    /// 最低价
    pub low: f64,
    // 最新的一组交易量(5日)
    // pub latest_volmes: Vec<f64>,
    /// 振幅(%)
    pub amplitude: f64,
    /// 涨跌幅(%)
    pub change_percentage: f64,
    /// 涨跌额(%)
    pub change_amount: f64,
}
