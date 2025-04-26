use std::str::FromStr;

use chrono::{DateTime, NaiveDate, Utc};
use clickhouse::Row;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use strum::EnumIter;

use crate::{
    schema::{self, akshare::a_stock},
    utils::splite_date_naive,
};

// 定义交易状态枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TradingStatus {
    Active,     // 正常交易
    Suspended,  // 停牌
    LimitUp,    // 涨停
    LimitDown,  // 跌停
    NewListing, // 新上市
    Other,      // 其他状态
}

impl ToString for TradingStatus {
    fn to_string(&self) -> String {
        self.as_str().to_owned()
    }
}

impl TradingStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            TradingStatus::Active => "ACTIVE",
            TradingStatus::Suspended => "SUSPENDED",
            TradingStatus::LimitUp => "LIMIT_UP",
            TradingStatus::LimitDown => "LIMIT_DOWN",
            TradingStatus::NewListing => "NEW_LISTING",
            TradingStatus::Other => "OTHER",
        }
    }

    /// 判断股票是否停牌
    pub fn determine_status(data: &a_stock::RealtimeStockMarketRecord) -> Self {
        // 如果最新价为None，可能表示停牌
        if data.latest_price.is_none() {
            return TradingStatus::Suspended;
        }

        // 根据涨跌幅判断涨跌停
        // if let Some(change_percentage) = data.change_percentage {
        //     if change_percentage >= 9.9 {
        //         return TradingStatus::LimitUp;
        //     } else if change_percentage <= -9.9 {
        //         return TradingStatus::LimitDown;
        //     }
        // }

        // 其他情况视为正常交易
        TradingStatus::Active
    }
}

/// 与ClickHouse表结构对应的Rust结构体
#[derive(Debug, Clone, Row, Serialize, Deserialize)]
pub struct RealtimeStockMarketRecord {
    /// 时间戳，精确到毫秒
    #[serde(with = "clickhouse::serde::chrono::datetime64::millis")]
    pub timestamp: DateTime<Utc>,
    /// 股票代码
    pub code: String,
    /// 公司名称
    pub name: String,
    /// 序号
    pub idx: i32,
    /// 交易状态
    pub trading_status: String,
    /// 最新价
    pub latest_price: f64,
    /// 涨跌幅
    pub change_percentage: f64,
    /// 涨跌额
    pub change_amount: f64,
    /// 振幅
    pub amplitude: f64,
    /// 最高价
    pub high: f64,
    /// 最低价
    pub low: f64,
    /// 今开
    pub today_open: f64,
    /// 昨收
    pub previous_close: f64,
    /// 成交量
    pub trading_volume: f64,
    /// 成交额
    pub trading_value: f64,
    /// 量比
    pub volume_ratio: f64,
    /// 换手率
    pub turnover_rate: f64,
    /// 市盈率(动态)
    pub pe_ratio_ttm: f64,
    /// 市净率
    pub pb_ratio: f64,
    /// 总市值
    pub total_market_value: f64,
    /// 流通市值
    pub circulating_market_value: f64,
    /// 涨速
    pub change_speed: f64,
    /// 5分钟涨跌
    pub five_minute_change: f64,
    /// 60日涨跌幅
    pub sixty_day_change: f64,
    /// 年初至今涨跌幅
    pub ytd_change: f64,
}

impl RealtimeStockMarketRecord {
    pub fn from_with_ts(
        source: a_stock::RealtimeStockMarketRecord,
        timestamp: DateTime<Utc>,
    ) -> Self {
        // 确定交易状态
        let trading_status = TradingStatus::determine_status(&source);

        // 对于停牌股票，使用特殊处理逻辑
        let is_suspended = trading_status == TradingStatus::Suspended;

        // 安全地提取Option<f64>值或使用默认值
        let extract_value = |opt: Option<f64>, default: f64| -> f64 {
            if is_suspended {
                // 停牌时可以使用特殊的默认值策略
                opt.unwrap_or(default)
            } else {
                // 非停牌但数据缺失，使用-1.0作为默认值
                opt.unwrap_or(-1.0)
            }
        };

        // 为停牌股票提供合理的默认值（这里使用昨收价，如果有的话）
        let previous_close = source.previous_close.unwrap_or(0.0);

        RealtimeStockMarketRecord {
            timestamp,
            code: source.code,
            name: source.name,
            idx: source.index as i32, // 转换为i32类型以匹配表结构
            trading_status: trading_status.to_string(),

            // 价格信息 - 对于停牌股票使用昨收价作为默认值
            latest_price: extract_value(source.latest_price, previous_close),
            change_percentage: extract_value(source.change_percentage, 0.0),
            change_amount: extract_value(source.change_amount, 0.0),
            amplitude: extract_value(source.amplitude, 0.0),
            high: extract_value(source.high, previous_close),
            low: extract_value(source.low, previous_close),
            today_open: extract_value(source.today_open, previous_close),
            previous_close,

            // 交易信息 - 停牌时设为0
            trading_volume: extract_value(source.trading_volume, 0.0),
            trading_value: extract_value(source.trading_value, 0.0),
            volume_ratio: extract_value(source.volume_ratio, 0.0),
            turnover_rate: extract_value(source.turnover_rate, 0.0),

            // 估值信息
            pe_ratio_ttm: extract_value(source.pe_ratio_ttm, 0.0),
            pb_ratio: extract_value(source.pb_ratio, 0.0),
            total_market_value: extract_value(source.total_market_value, 0.0),
            circulating_market_value: extract_value(source.circulating_market_value, 0.0),

            // 动态变化信息 - 停牌时设为0
            change_speed: extract_value(source.change_speed, 0.0),
            five_minute_change: extract_value(source.five_minute_change, 0.0),
            sixty_day_change: extract_value(source.sixty_day_change, 0.0),
            ytd_change: extract_value(source.ytd_change, 0.0),
        }
    }
}

/// 日频A股数据复权方式
#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug, EnumIter, Clone, Copy)]
#[repr(u8)]
pub enum StockAdjustmentType {
    None,     // 不复权
    Forward,  // 前复权
    Backward, // 后复权
}

impl StockAdjustmentType {
    pub fn to_str(&self) -> &'static str {
        match self {
            Self::None => "",
            Self::Forward => "qfq",
            Self::Backward => "hfq",
        }
    }
}

/// 东方财富-沪深京 A 股日频率数据; 历史数据按日频率更新, 当日收盘价在收盘后获取
///
/// clickhouse数据模型
#[derive(Debug, Deserialize, Serialize, Row)]
pub struct StockZhAHist {
    /// 股票代码
    pub code: String,
    /// 开盘价
    pub open: f64,
    /// 收盘价
    pub close: f64,
    /// 最低价
    pub low: f64,
    /// 最高价
    pub high: f64,
    /// 成交量
    pub trading_volume: f64,
    /// 成交额
    pub trading_value: f64,
    /// 振幅
    pub amplitude: f64,
    /// 换手率
    pub turnover_rate: f64,
    /// 涨跌幅
    pub change_percentage: f64,
    /// 涨跌额
    pub change_amount: f64,
    /// 数据产生日期
    #[serde(with = "clickhouse::serde::chrono::date")]
    pub date: NaiveDate,
    /// 复权方式枚举
    pub adj_type: StockAdjustmentType,
}

impl StockZhAHist {
    pub fn from_with_type(
        value: schema::akshare::StockZhAHist,
        adj_type: StockAdjustmentType,
    ) -> Self {
        let date = NaiveDate::from_str(splite_date_naive(&value.date))
            .expect("date formet should be ISO 8601");

        Self {
            code: value.code,
            open: value.open,
            close: value.close,
            low: value.low,
            high: value.high,
            trading_volume: value.trading_volume,
            trading_value: value.trading_value,
            amplitude: value.amplitude,
            turnover_rate: value.turnover_rate,
            change_percentage: value.change_percentage,
            change_amount: value.change_amount,
            date,
            adj_type,
        }
    }
}

// ------------------------------------------------------------------------------

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug, EnumIter, Clone, Copy)]
#[repr(u8)]
pub enum FlowDirection {
    Northbound, // 北向资金
    Southbound, // 南向资金
}

impl FlowDirection {
    pub fn as_str(&self) -> &'static str {
        match self {
            FlowDirection::Northbound => "北向资金",
            FlowDirection::Southbound => "南向资金",
        }
    }
}

// TODO 数据结构不同，下面想要的话只能新定义一个schema
// ShanghaiConnect,  // 沪股通
// ShenzhenConnect,  // 深股通
// HongKongConnectSh, // 港股通沪
// HongKongConnectSz, // 港股通深

/// 东方财富网-数据中心-资金流向-沪深港通资金流向-沪深港通历史数据，  
/// 分为南向北向
#[derive(Debug, Serialize, Deserialize, Row)]
pub struct StockHsgtHistEm {
    /// 资金流动方向
    pub flow_dir: FlowDirection,
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
    /// 日期，格式："2023-09-28T00:00:00.000"
    #[serde(with = "clickhouse::serde::chrono::date")]
    pub date: NaiveDate,
    /// 收集数据时间戳，毫秒等级
    #[serde(with = "clickhouse::serde::chrono::datetime64::millis")]
    pub ts: DateTime<Utc>,
}

impl StockHsgtHistEm {
    pub fn from_with_dir_ts(
        mut value: schema::akshare::StockHsgtHistEm,
        flow_dir: FlowDirection,
        ts: DateTime<Utc>,
    ) -> Self {
        value.date.push('Z');
        let date = value
            .date
            .parse::<DateTime<Utc>>()
            .unwrap()
            .naive_utc()
            .date();

        Self {
            flow_dir,
            buy_amount: value.buy_amount.unwrap_or_default(),
            sell_amount: value.sell_amount.unwrap_or_default(),
            historical_net_buy_amount: value.historical_net_buy_amount.unwrap_or_default(),
            daily_balance: value.daily_balance.unwrap_or_default(),
            daily_net_buy_amount: value.daily_net_buy_amount.unwrap_or_default(),
            daily_inflow: value.daily_inflow.unwrap_or_default(),
            holding_market_value: value.holding_market_value,
            hs300_index: value.hs300_index,
            hs300_change_percent: value.hs300_change_percent,
            leading_stock_name: value.leading_stock_name,
            leading_stock_code: value.leading_stock_code,
            leading_stock_change_percent: value.leading_stock_change_percent,
            date,
            ts,
        }
    }
}

// --------------------------------------------------------------------------

#[derive(Debug, Serialize, Deserialize, Row)]
pub struct StockZtPoolEm {
    /// 股票代码
    pub code: String,
    /// 股票名称
    pub name: String,
    /// 封板所需资金（单位：元）
    pub lockup_funds: f64,
    /// 序号
    pub serial_number: u32,
    /// 总市值（单位：元）
    pub total_market_value: f64,
    /// 成交额（单位：元）
    pub turnover: f64,
    /// 所属行业
    pub industry: String,
    /// 换手率（百分比）
    pub turnover_rate: f64,
    /// 最后封板时间（格式：HHMMSS）
    pub last_lockup_time: String,
    /// 最新价格
    pub latest_price: f64,
    /// 流通市值（单位：元）
    pub circulating_market_value: f64,
    /// 涨停统计（例如 "1/1"）
    pub limit_up_statistics: String,
    /// 涨跌幅（百分比）
    pub price_change_percentage: f64,
    /// 炸板次数（封板失败次数）
    pub failed_lockup_count: u32,
    /// 连续涨停板数量
    pub consecutive_limit_ups: u32,
    /// 首次封板时间（格式：HHMMSS）
    pub first_lockup_time: String,
    /// 数据生成时间
    #[serde(with = "clickhouse::serde::chrono::date")]
    pub date: NaiveDate,
    /// 数据收集时间
    #[serde(with = "clickhouse::serde::chrono::datetime64::millis")]
    pub ts: DateTime<Utc>,
}

impl StockZtPoolEm {
    pub fn from_with_time(
        value: schema::akshare::StockZtPoolEm,
        date: NaiveDate,
        ts: DateTime<Utc>,
    ) -> Self {
        Self {
            code: value.code,
            name: value.name,
            lockup_funds: value.lockup_funds,
            serial_number: value.serial_number,
            total_market_value: value.total_market_value,
            turnover: value.turnover,
            industry: value.industry,
            turnover_rate: value.turnover_rate,
            last_lockup_time: value.last_lockup_time,
            latest_price: value.latest_price,
            circulating_market_value: value.circulating_market_value,
            limit_up_statistics: value.limit_up_statistics,
            price_change_percentage: value.price_change_percentage,
            failed_lockup_count: value.failed_lockup_count,
            consecutive_limit_ups: value.consecutive_limit_ups,
            first_lockup_time: value.first_lockup_time,
            date,
            ts,
        }
    }
}
