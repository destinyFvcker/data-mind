//! AKShare 股票数据
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

/// 东方财富-沪深京 A 股日频率数据; 历史数据按日频率更新, 当日收盘价在收盘后获取
///
/// akshare api数据模型
#[derive(Debug, Deserialize)]
pub struct StockZhAHist {
    /// 开盘价
    #[serde(rename(deserialize = "开盘"))]
    pub open: f64,
    /// 收盘价
    #[serde(rename(deserialize = "收盘"))]
    pub close: f64,
    /// 最低价
    #[serde(rename(deserialize = "最低"))]
    pub low: f64,
    /// 最高价
    #[serde(rename(deserialize = "最高"))]
    pub high: f64,
    /// 成交量
    #[serde(rename(deserialize = "成交量"))]
    pub trading_volume: f64,
    /// 成交额
    #[serde(rename(deserialize = "成交额"))]
    pub trading_value: f64,
    /// 振幅
    #[serde(rename(deserialize = "振幅"))]
    pub amplitude: f64,
    /// 换手率
    #[serde(rename(deserialize = "换手率"))]
    pub turnover_rate: f64,
    /// 涨跌幅
    #[serde(rename(deserialize = "涨跌幅"))]
    pub change_percentage: f64,
    /// 涨跌额
    #[serde(rename(deserialize = "涨跌额"))]
    pub change_amount: f64,
    /// 日期
    #[serde(rename(deserialize = "日期"))]
    pub date: String,
    /// 股票代码
    #[serde(rename(deserialize = "股票代码"))]
    pub code: String,
}

/// 沪深港通历史数据
///
/// 目标地址: https://data.eastmoney.com/hsgt/index.html  
/// 描述: 东方财富网-数据中心-资金流向-沪深港通资金流向-沪深港通历史数据
#[derive(Debug, Deserialize)]
pub struct StockHsgtHistEm {
    /// 买入成交额，单位：亿元
    #[serde(rename(deserialize = "买入成交额"))]
    pub buy_amount: Option<f64>,
    /// 卖出成交额，单位：亿元
    #[serde(rename(deserialize = "卖出成交额"))]
    pub sell_amount: Option<f64>,
    /// 历史累计净买额，单位：万亿元
    #[serde(rename(deserialize = "历史累计净买额"))]
    pub historical_net_buy_amount: Option<f64>,
    /// 当日余额，单位：亿元
    #[serde(rename(deserialize = "当日余额"))]
    pub daily_balance: Option<f64>,
    /// 当日成交净买额，单位：亿元
    #[serde(rename(deserialize = "当日成交净买额"))]
    pub daily_net_buy_amount: Option<f64>,
    /// 当日资金流入，单位：亿元
    #[serde(rename(deserialize = "单日资金流入"))]
    pub daily_inflow: Option<f64>,
    /// 持股市值，单位：元
    #[serde(rename(deserialize = "持股市值"))]
    pub holding_market_value: f64,
    /// 日期，格式："2023-09-28T00:00:00.000"
    #[serde(rename(deserialize = "日期"))]
    pub date: String,
    /// 沪深300指数点位
    #[serde(rename(deserialize = "沪深300"))]
    pub hs300_index: f64,
    /// 沪深300指数涨跌幅，单位：%
    #[serde(rename(deserialize = "沪深300-涨跌幅"))]
    pub hs300_change_percent: f64,
    /// 领涨股名称
    #[serde(rename(deserialize = "领涨股"))]
    pub leading_stock_name: String,
    /// 领涨股代码，例如 "600198.SH"
    #[serde(rename(deserialize = "领涨股-代码"))]
    pub leading_stock_code: String,
    /// 领涨股涨跌幅，单位：%
    #[serde(rename(deserialize = "领涨股-涨跌幅"))]
    pub leading_stock_change_percent: f64,
}
