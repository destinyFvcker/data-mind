//! A股相关数据dto实体定义

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::repository::MALinesRepo;
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
