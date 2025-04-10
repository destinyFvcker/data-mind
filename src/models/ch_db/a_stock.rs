use chrono::{DateTime, Utc};
use clickhouse::Row;
use serde::{Deserialize, Serialize};

use crate::models::akshare;

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
    pub fn determine_status(data: &akshare::RealtimeStockMarketRecord) -> Self {
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
#[derive(Debug, Clone, Row, Serialize)]
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
        source: akshare::RealtimeStockMarketRecord,
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
