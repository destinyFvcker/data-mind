//! akshare 指数数据

use clickhouse::Row;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::utils::with_base_url;

/// 实时行情数据-新浪
///
/// stock_zh_index_spot_sina 新浪财经-中国股票指数数据数据接口
#[derive(Debug, Deserialize)]
pub struct AkStockZhIndexSpotSina {
    /// 代码
    #[serde(rename(deserialize = "代码"))]
    pub code: String,
    /// 名称
    #[serde(rename(deserialize = "名称"))]
    pub name: String,
    /// 最新价
    #[serde(rename(deserialize = "最新价"))]
    pub latest_price: f64,
    /// 涨跌额
    #[serde(rename(deserialize = "涨跌额"))]
    pub change_amount: f64,
    /// 涨跌幅
    #[serde(rename(deserialize = "涨跌幅"))]
    pub change_percentage: f64,
    /// 昨收
    #[serde(rename(deserialize = "昨收"))]
    pub previous_close: f64,
    /// 今开
    #[serde(rename(deserialize = "今开"))]
    pub open_price: f64,
    /// 最高
    #[serde(rename(deserialize = "最高"))]
    pub high_price: f64,
    /// 最低
    #[serde(rename(deserialize = "最低"))]
    pub low_price: f64,
    /// 成交量
    #[serde(rename(deserialize = "成交量"))]
    pub volume: f64,
    /// 成交额
    #[serde(rename(deserialize = "成交额"))]
    pub value: f64,
}

/// 历史行情数据-新浪
///
/// 接口: stock_zh_index_daily 股票指数的历史数据按日频率更新
#[derive(Debug, Deserialize)]
pub struct AkStockZhIndexDaily {
    /// 时间戳
    pub date: String,
    /// 最高
    pub high: f64,
    /// 最低
    pub low: f64,
    /// 开盘
    pub open: f64,
    /// 收盘
    pub close: f64,
    /// 交易量
    pub volume: f64,
}

/// 50ETF 期权波动率指数
///
/// 50ETF 期权波动率指数 QVIX; 又称中国版的恐慌指数
#[derive(Debug, Deserialize)]
pub struct AkIndexOption50EtfQvix {
    /// 时间戳
    pub date: String,
    /// 开盘
    pub open: Option<f64>,
    /// 收盘
    pub high: Option<f64>,
    /// 最高
    pub low: Option<f64>,
    /// 最低
    pub close: Option<f64>,
}

/// 股票指数信息一览表表项
#[derive(Debug, Deserialize, Serialize, ToSchema, Row)]
pub struct AkIndexStockInfo {
    /// 指数名称
    pub display_name: String,
    /// 指数代码
    pub index_code: String,
    // 指数发布日期,格式为%Y/%m/%d
    pub publish_date: String,
}

impl AkIndexStockInfo {
    pub async fn from_astock_api(reqwest_client: &reqwest::Client) -> anyhow::Result<Vec<Self>> {
        let data: Vec<Self> = reqwest_client
            .get(with_base_url("/index_stock_info"))
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?;
        Ok(data)
    }
}
