use chrono::{DateTime, NaiveDate, Utc};
use clickhouse::Row;
use serde::{Deserialize, Serialize};

use crate::schema::akshare::{AkIndexOption50EtfQvix, AkStockZhIndexDaily};

pub use crate::schema::akshare::AkIndexStockInfo as IndexStockInfoInsert;

/// 指数日频历史行情数据-新浪
///
/// clickhouse数据模型
#[derive(Debug, Serialize, Deserialize, Row)]
pub struct StockZhIndexDailyInsert {
    /// 指数代码
    pub code: String,
    /// 开盘
    pub open: f64,
    /// 收盘
    pub close: f64,
    /// 最高
    pub high: f64,
    /// 最低
    pub low: f64,
    /// 交易量
    pub volume: f64,
    /// 数据生成时间戳
    #[serde(with = "clickhouse::serde::chrono::date")]
    pub date: NaiveDate,
    /// 数据收集时间戳
    #[serde(with = "clickhouse::serde::chrono::datetime64::millis")]
    pub ts: DateTime<Utc>,
}

impl StockZhIndexDailyInsert {
    pub fn from_with_ts(mut value: AkStockZhIndexDaily, code: &str, ts: DateTime<Utc>) -> Self {
        value.date.push('Z');
        let date = value
            .date
            .parse::<DateTime<Utc>>()
            .unwrap()
            .naive_utc()
            .date();

        Self {
            code: code.to_owned(),
            high: value.high,
            low: value.low,
            open: value.open,
            close: value.close,
            volume: value.volume,
            date,
            ts,
        }
    }
}

// --------------

/// 50ETF 期权波动率指数 *clickhouse* schema
///
/// 50ETF 期权波动率指数 QVIX; 又称中国版的恐慌指数
#[derive(Debug, Deserialize, Serialize, Row)]
pub struct IndexOption50EtfQvixInsert {
    /// 开盘
    pub open: f64,
    /// 收盘
    pub close: f64,
    /// 最高
    pub high: f64,
    /// 最低
    pub low: f64,
    /// 数据生成时间戳
    #[serde(with = "clickhouse::serde::chrono::date")]
    pub date: NaiveDate,
    /// 数据收集时间戳
    #[serde(with = "clickhouse::serde::chrono::datetime64::millis")]
    pub ts: DateTime<Utc>,
}

impl IndexOption50EtfQvixInsert {
    pub fn from_with_ts(mut value: AkIndexOption50EtfQvix, ts: DateTime<Utc>) -> Option<Self> {
        if value.open.is_none()
            || value.close.is_none()
            || value.high.is_none()
            || value.low.is_none()
        {
            return None;
        }

        value.date.push('Z');
        let date = value
            .date
            .parse::<DateTime<Utc>>()
            .unwrap()
            .naive_utc()
            .date();

        Some(Self {
            open: value.open.unwrap(),
            close: value.close.unwrap(),
            high: value.high.unwrap(),
            low: value.low.unwrap(),
            date,
            ts,
        })
    }
}

#[cfg(test)]
mod test {
    #![allow(unused)]
    use chrono::{DateTime, FixedOffset, Utc};

    #[test]
    fn test_parse_time() {
        let date_str = "2025-04-21T00:00:00.000Z";
        let date = date_str.parse::<DateTime<Utc>>().unwrap();

        let date1 = date;
        let date2 = date;
        println!("date = {date:?}");
    }
}
