//! A股相关数据dto实体定义

use chrono::NaiveDate;
use clickhouse::Row;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

pub use crate::schema::akshare::StockIndividualInfoEm;

/// 移动平均线数据(MA5/MA10/MA20)
#[derive(Debug, Serialize, Deserialize, Row, ToSchema)]
struct MALines {
    date: NaiveDate,
    ma5: Option<f64>,
    ma10: Option<f64>,
    ma20: Option<f64>,
}
