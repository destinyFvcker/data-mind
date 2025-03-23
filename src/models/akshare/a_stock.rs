use serde::{Deserialize, Serialize};

/// Real-time market data 数据来源为东方财经
#[derive(Debug, Deserialize, Serialize)]
pub struct RealtimeStockMarketRecord {
    /// 序号 - 股票在列表中的位置索引
    #[serde(rename(deserialize = "序号"))]
    pub index: i64,
    /// 代码 - 股票代码
    #[serde(rename(deserialize = "代码"))]
    pub code: String,
    /// 名称 - 公司名称
    #[serde(rename(deserialize = "名称"))]
    pub name: String,
    /// 最新价 - 当前交易价格
    #[serde(rename(deserialize = "最新价"))]
    pub latest_price: Option<f64>,
    /// 涨跌幅 - 价格变动的百分比
    #[serde(rename(deserialize = "涨跌幅"))]
    pub change_percentage: Option<f64>,
    /// 涨跌额 - 价格变动的绝对值
    #[serde(rename(deserialize = "涨跌额"))]
    pub change_amount: Option<f64>,
    /// 成交量 - 交易的股票数量
    #[serde(rename(deserialize = "成交量"))]
    pub trading_volume: Option<f64>,
    /// 成交额 - 交易的总金额
    #[serde(rename(deserialize = "成交额"))]
    pub trading_value: Option<f64>,
    /// 振幅 - 当日最高价与最低价的差值占前一交易日收盘价的百分比
    #[serde(rename(deserialize = "振幅"))]
    pub amplitude: Option<f64>,
    /// 最高 - 当日最高交易价格
    #[serde(rename(deserialize = "最高"))]
    pub high: Option<f64>,
    /// 最低 - 当日最低交易价格
    #[serde(rename(deserialize = "最低"))]
    pub low: Option<f64>,
    /// 今开 - 当日开盘价格
    #[serde(rename(deserialize = "今开"))]
    pub today_open: Option<f64>,
    /// 昨收 - 前一交易日收盘价格
    #[serde(rename(deserialize = "昨收"))]
    pub previous_close: Option<f64>,
    /// 量比 - 当日成交量与过去一段时间平均成交量之比
    #[serde(rename(deserialize = "量比"))]
    pub volume_ratio: Option<f64>,
    /// 换手率 - 成交量占流通股本的百分比
    #[serde(rename(deserialize = "换手率"))]
    pub turnover_rate: Option<f64>,
    /// 市盈率(动态) - 股价与每股收益的比率(基于过去12个月数据)
    #[serde(rename(deserialize = "市盈率-动态"))]
    pub pe_ratio_ttm: Option<f64>,
    /// 市净率 - 股价与每股净资产的比率
    #[serde(rename(deserialize = "市净率"))]
    pub pb_ratio: Option<f64>,
    /// 总市值 - 公司所有流通与非流通股份的总价值
    #[serde(rename(deserialize = "总市值"))]
    pub total_market_value: Option<f64>,
    /// 流通市值 - 公司流通股份的总价值
    #[serde(rename(deserialize = "流通市值"))]
    pub circulating_market_value: Option<f64>,
    /// 涨速 - 最近一段时间内价格变动的速率
    #[serde(rename(deserialize = "涨速"))]
    pub change_speed: Option<f64>,
    /// 5分钟涨跌 - 最近5分钟的价格变动百分比
    #[serde(rename(deserialize = "5分钟涨跌"))]
    pub five_minute_change: Option<f64>,
    /// 60日涨跌幅 - 60个交易日内的价格变动百分比
    #[serde(rename(deserialize = "60日涨跌幅"))]
    pub sixty_day_change: Option<f64>,
    /// 年初至今涨跌幅 - 从年初到现在的价格变动百分比
    #[serde(rename(deserialize = "年初至今涨跌幅"))]
    pub ytd_change: Option<f64>,
}
