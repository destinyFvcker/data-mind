//! AKShare 股票数据
use clickhouse::Row;
use serde::{Deserialize, Deserializer, Serialize};
use serde_json::{Number, Value};
use utoipa::ToSchema;

use crate::utils::with_base_url;

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
pub struct AkStockZhAHist {
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
    /// 成交量，注意单位(手)
    #[serde(rename(deserialize = "成交量"))]
    pub trading_volume: f64,
    /// 成交额，注意单位(元)
    #[serde(rename(deserialize = "成交额"))]
    pub trading_value: f64,
    /// 振幅(%)
    #[serde(rename(deserialize = "振幅"))]
    pub amplitude: f64,
    /// 换手率(%)
    #[serde(rename(deserialize = "换手率"))]
    pub turnover_rate: f64,
    /// 涨跌幅(%)
    #[serde(rename(deserialize = "涨跌幅"))]
    pub change_percentage: f64,
    /// 涨跌额，注意单位(元)
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
pub struct AkStockHsgtHistEm {
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

/// 股票信息结构体
#[derive(Debug, Serialize, Deserialize)]
pub struct AkStockZtPoolEm {
    /// 股票代码
    #[serde(rename(deserialize = "代码"))]
    pub code: String,
    /// 股票名称
    #[serde(rename(deserialize = "名称"))]
    pub name: String,
    /// 封板所需资金（单位：元）
    #[serde(rename(deserialize = "封板资金"))]
    pub lockup_funds: f64,
    /// 序号
    #[serde(rename(deserialize = "序号"))]
    pub serial_number: u32,
    /// 总市值（单位：元）
    #[serde(rename(deserialize = "总市值"))]
    pub total_market_value: f64,
    /// 成交额（单位：元）
    #[serde(rename(deserialize = "成交额"))]
    pub turnover: f64,
    /// 所属行业
    #[serde(rename(deserialize = "所属行业"))]
    pub industry: String,
    /// 换手率（百分比）
    #[serde(rename(deserialize = "换手率"))]
    pub turnover_rate: f64,
    /// 最后封板时间（格式：HHMMSS）
    #[serde(rename(deserialize = "最后封板时间"))]
    pub last_lockup_time: String,
    /// 最新价格
    #[serde(rename(deserialize = "最新价"))]
    pub latest_price: f64,
    /// 流通市值（单位：元）
    #[serde(rename(deserialize = "流通市值"))]
    pub circulating_market_value: f64,
    /// 涨停统计（例如 "1/1"）
    #[serde(rename(deserialize = "涨停统计"))]
    pub limit_up_statistics: String,
    /// 涨跌幅（百分比）
    #[serde(rename(deserialize = "涨跌幅"))]
    pub price_change_percentage: f64,
    /// 炸板次数（封板失败次数）
    #[serde(rename(deserialize = "炸板次数"))]
    pub failed_lockup_count: u32,
    /// 连续涨停板数量
    #[serde(rename(deserialize = "连板数"))]
    pub consecutive_limit_ups: u32,
    /// 首次封板时间（格式：HHMMSS）
    #[serde(rename(deserialize = "首次封板时间"))]
    pub first_lockup_time: String,
}

/// 接口: stock_news_em  
/// 实际数据源地址: https://so.eastmoney.com/news/s  
/// 描述: 东方财富指定个股的新闻资讯数据  
/// 限量: 指定 symbol(股票id) 当日最近 100 条新闻资讯数据
#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct AkStockNewsEm {
    /// 关键词，例如股票代码（如 "300059"）
    #[serde(rename(deserialize = "关键词"))]
    pub keyword: String,
    /// 新闻发布时间（格式：YYYY-MM-DD HH:MM:SS）
    #[serde(rename(deserialize = "发布时间"))]
    pub publish_time: String,
    /// 文章来源，例如“人民财讯”
    #[serde(rename(deserialize = "文章来源"))]
    pub source: String,
    /// 新闻正文内容
    #[serde(rename(deserialize = "新闻内容"))]
    pub content: String,
    /// 新闻标题
    #[serde(rename(deserialize = "新闻标题"))]
    pub title: String,
    /// 新闻链接 URL
    #[serde(rename(deserialize = "新闻链接"))]
    pub url: String,
}

impl AkStockNewsEm {
    /// 从astock数据接口获取相关数据
    pub async fn from_astock_api(
        reqwest_client: &reqwest::Client,
        symbol: &str,
    ) -> anyhow::Result<Vec<Self>> {
        let res: Vec<Self> = reqwest_client
            .get(with_base_url("/stock_news_em"))
            .query(&[("symbol", symbol)])
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?;
        Ok(res)
    }
}

/// json schema -> 财经内容精选  
/// 目标地址: https://cxdata.caixin.com/pc/  
/// 描述: 财新网-财新数据通-内容精选  
/// 限量: 返回所有历史新闻数据
#[derive(Debug, Deserialize)]
pub struct AkStockNewsMainCx {
    /// 新闻被精选、推送或整理到内容库的时间。格式通常是 yyyy-MM-dd HH:mm。
    pub interval_time: String,
    /// 新闻的正式发布时间，即新闻内容原文在财新网等发布的时间。格式是完整的 yyyy-MM-dd HH:mm:ss.sss。
    pub pub_time: String,
    /// 新闻的摘要内容，对新闻正文的简要概括，便于快速了解新闻主旨。
    pub summary: String,
    /// 新闻的主题标签，通常由几个关键词组成，归纳了该新闻的主要话题或核心内容。
    pub tag: String,
    /// 新闻的详情链接，点击可以跳转到财新网对应的新闻完整正文页面。
    pub url: String,
}

/// 关键指标-同花顺 `stock_financial_abstract_ths`  
/// 目标地址: https://basic.10jqka.com.cn/new/000063/finance.html  
/// 描述: 同花顺-财务指标-主要指标  
/// 限量: 单次获取指定 symbol 的所有数据  
#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct AkStockFinancialAbstractThs {
    /// 报告期
    #[serde(deserialize_with = "always_string", rename(deserialize = "报告期"))]
    pub report_date: String,
    /// 净利润
    #[serde(
        deserialize_with = "false_or_null_as_none",
        rename(deserialize = "净利润")
    )]
    pub net_profit: Option<String>,
    /// 净利润同比增长率
    #[serde(
        deserialize_with = "false_or_null_as_none",
        rename(deserialize = "净利润同比增长率")
    )]
    pub net_profit_growth_rate: Option<String>,
    /// 扣非净利润
    #[serde(
        deserialize_with = "false_or_null_as_none",
        rename(deserialize = "扣非净利润")
    )]
    pub deducted_net_profit: Option<String>,
    /// 扣非净利润同比增长率
    #[serde(
        deserialize_with = "false_or_null_as_none",
        rename(deserialize = "扣非净利润同比增长率")
    )]
    pub deducted_net_profit_growth_rate: Option<String>,
    /// 营业总收入
    #[serde(
        deserialize_with = "false_or_null_as_none",
        rename(deserialize = "营业总收入")
    )]
    pub total_revenue: Option<String>,
    /// 营业总收入同比增长率
    #[serde(
        deserialize_with = "false_or_null_as_none",
        rename(deserialize = "营业总收入同比增长率")
    )]
    pub total_revenue_growth_rate: Option<String>,
    /// 基本每股收益
    #[serde(
        deserialize_with = "false_or_null_as_none",
        rename(deserialize = "基本每股收益")
    )]
    pub basic_eps: Option<String>,
    /// 每股净资产
    #[serde(rename(deserialize = "每股净资产"))]
    pub net_assets_per_share: Option<String>,
    /// 每股资本公积金
    #[serde(
        deserialize_with = "false_or_null_as_none",
        rename(deserialize = "每股资本公积金")
    )]
    pub capital_reserve_per_share: Option<String>,
    /// 每股未分配利润
    #[serde(
        deserialize_with = "false_or_null_as_none",
        rename(deserialize = "每股未分配利润")
    )]
    pub undistributed_profit_per_share: Option<String>,
    /// 每股经营现金流
    #[serde(
        deserialize_with = "false_or_null_as_none",
        rename(deserialize = "每股经营现金流")
    )]
    pub operating_cash_flow_per_share: Option<String>,
    /// 销售净利率
    #[serde(
        deserialize_with = "false_or_null_as_none",
        rename(deserialize = "销售净利率")
    )]
    pub net_profit_margin: Option<String>,
    /// 销售毛利率
    #[serde(
        deserialize_with = "false_or_null_as_none",
        rename(deserialize = "销售毛利率")
    )]
    pub gross_profit_margin: Option<String>,
    /// 净资产收益率
    #[serde(
        deserialize_with = "false_or_null_as_none",
        rename(deserialize = "净资产收益率")
    )]
    pub roe: Option<String>,
    /// 净资产收益率-摊薄
    #[serde(
        deserialize_with = "false_or_null_as_none",
        rename(deserialize = "净资产收益率-摊薄")
    )]
    pub roe_diluted: Option<String>,
    /// 营业周期
    #[serde(
        deserialize_with = "false_or_null_as_none",
        rename(deserialize = "营业周期")
    )]
    pub operating_cycle: Option<String>,
    /// 存货周转率
    #[serde(
        deserialize_with = "false_or_null_as_none",
        rename(deserialize = "存货周转率")
    )]
    pub inventory_turnover: Option<String>,
    /// 存货周转天数
    #[serde(
        deserialize_with = "false_or_null_as_none",
        rename(deserialize = "存货周转天数")
    )]
    pub inventory_turnover_days: Option<String>,
    /// 应收账款周转天数
    #[serde(
        deserialize_with = "false_or_null_as_none",
        rename(deserialize = "应收账款周转天数")
    )]
    pub accounts_receivable_turnover_days: Option<String>,
    /// 流动比率
    #[serde(
        deserialize_with = "false_or_null_as_none",
        rename(deserialize = "流动比率")
    )]
    pub current_ratio: Option<String>,
    /// 速动比率
    #[serde(
        deserialize_with = "false_or_null_as_none",
        rename(deserialize = "速动比率")
    )]
    pub quick_ratio: Option<String>,
    /// 保守速动比率
    #[serde(
        deserialize_with = "false_or_null_as_none",
        rename(deserialize = "保守速动比率")
    )]
    pub conservative_quick_ratio: Option<String>,
    /// 产权比率
    #[serde(
        deserialize_with = "false_or_null_as_none",
        rename(deserialize = "产权比率")
    )]
    pub equity_ratio: Option<String>,
    /// 资产负债率
    #[serde(
        deserialize_with = "false_or_null_as_none",
        rename(deserialize = "资产负债率")
    )]
    pub debt_asset_ratio: Option<String>,
}

// 通用的转换函数：false/null → None，其他有效值 → Some(对应类型)
fn false_or_null_as_none<'de, D, T>(deserializer: D) -> Result<Option<T>, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de>,
{
    let value = Value::deserialize(deserializer)?;
    match value {
        Value::Null => Ok(None),
        Value::Bool(false) => Ok(None),
        Value::Bool(true) => Err(serde::de::Error::custom("unexpected true")),
        other => {
            let t = T::deserialize(other)
                .map(Some)
                .map_err(serde::de::Error::custom)?; // 手动映射错误！
            Ok(t)
        }
    }
}

fn always_string<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let value = Value::deserialize(deserializer)?;
    let s = match value {
        Value::Null => String::new(),          // null → 空字符串
        Value::Bool(b) => b.to_string(),       // true/false → "true"/"false"
        Value::Number(num) => num.to_string(), // 数字 → "数字字符串"
        Value::String(s) => s,                 // 本来就是字符串
        Value::Array(_) | Value::Object(_) => {
            return Err(serde::de::Error::custom(
                "expected primitive, got complex type",
            ))
        }
    };
    Ok(s)
}

impl AkStockFinancialAbstractThs {
    /// 从aktool之中获取数据：  
    /// - symbol="000063"; 股票代码
    /// - indicator example = "按报告期"; choice of {"按报告期", "按年度", "按单季度"}
    pub async fn from_astock_api(
        reqwest_client: &reqwest::Client,
        symbol: &str,
        indicator: &str,
    ) -> anyhow::Result<Vec<Self>> {
        let res: Vec<Self> = reqwest_client
            .get(with_base_url("/stock_financial_abstract_ths"))
            .query(&[("symbol", symbol), ("indicator", indicator)])
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?;
        Ok(res)
    }
}

/// akshare json schema 技术指标-创新高  
/// 接口：stock_rank_cxg_ths
/// 目标地址：https://data.10jqka.com.cn/rank/cxg/  
/// 描述：同花顺-数据中心-技术选股-创新高  
/// 限量：单次指定 symbol 的所有数据
#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct AkStockRankCxgThs {
    /// 前期高点
    #[serde(rename(deserialize = "前期高点"))]
    pub previous_high: f64,
    /// 前期高点日期, 格式为`%Y-%m-%dT%H:%M:%S%.3f`
    #[serde(rename(deserialize = "前期高点日期"))]
    pub previous_high_date: String,
    /// 序号
    #[serde(rename(deserialize = "序号"))]
    pub index: u32,
    /// 换手率，注意单位%
    #[serde(rename(deserialize = "换手率"))]
    pub turnover_rate: f64,
    /// 最新价，注意单位：元
    #[serde(rename(deserialize = "最新价"))]
    pub latest_price: f64,
    /// 涨跌幅，注意单位%
    #[serde(rename(deserialize = "涨跌幅"))]
    pub change_percentage: f64,
    /// 股票代码
    #[serde(rename(deserialize = "股票代码"))]
    pub stock_code: String,
    /// 股票简称
    #[serde(rename(deserialize = "股票简称"))]
    pub stock_name: String,
}

impl AkStockRankCxgThs {
    /// 从akshare之中获取数据  
    /// symbol example = "创月新高"; choice of {"创月新高", "半年新高", "一年新高", "历史新高"}
    pub async fn from_astock_api(
        reqwest_client: &reqwest::Client,
        symbol: &str,
    ) -> anyhow::Result<Vec<Self>> {
        let res: Vec<Self> = reqwest_client
            .get(with_base_url("/stock_rank_cxg_ths"))
            .query(&[("symbol", symbol)])
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?;
        Ok(res)
    }
}

/// akshare json schema 技术指标-创新低  
/// 接口：stock_rank_cxd_ths  
/// 目标地址：https://data.10jqka.com.cn/rank/cxd/  
/// 描述：同花顺-数据中心-技术选股-创新低  
/// 限量：单次指定 symbol 的所有数据
#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct AkStockRankCxdThs {
    /// 前期低点价格
    #[serde(rename(deserialize = "前期低点"))]
    pub previous_low: f64,
    /// 前期低点对应的日期，格式为 "YYYY-MM-DDTHH:MM:SS.sss"
    #[serde(rename(deserialize = "前期低点日期"))]
    pub previous_low_date: String,
    /// 序号
    #[serde(rename(deserialize = "序号"))]
    pub index: i32,
    /// 换手率，单位：百分比 (%)
    #[serde(rename(deserialize = "换手率"))]
    pub turnover_rate: f64,
    /// 最新价格，注意单位：(元)
    #[serde(rename(deserialize = "最新价"))]
    pub latest_price: f64,
    /// 涨跌幅，单位：百分比 (%)
    #[serde(rename(deserialize = "涨跌幅"))]
    pub change_percentage: f64,
    /// 股票代码
    #[serde(rename(deserialize = "股票代码"))]
    pub stock_code: String,
    /// 股票简称
    #[serde(rename(deserialize = "股票简称"))]
    pub stock_name: String,
}

impl AkStockRankCxdThs {
    /// 从akshare获取数据  
    /// symbol example="创月新低"; choice of {"创月新低", "半年新低", "一年新低", "历史新低"}
    pub async fn from_astock_api(
        reqwest_client: &reqwest::Client,
        symbol: &str,
    ) -> anyhow::Result<Vec<Self>> {
        let res: Vec<Self> = reqwest_client
            .get(with_base_url("/stock_rank_cxd_ths"))
            .query(&[("symbol", symbol)])
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?;
        Ok(res)
    }
}

/// akshare json schema 技术指标-连续上涨  
/// 接口：stock_rank_lxsz_ths  
/// 目标地址：https://data.10jqka.com.cn/rank/lxsz/  
/// 描述：同花顺-数据中心-技术选股-连续上涨  
/// 限量：单次返回所有数据
#[derive(Debug, Deserialize, Serialize, ToSchema, Row)]
pub struct AkStockRankLxszThs {
    /// 序号
    #[serde(rename(deserialize = "序号"))]
    pub index: i32,
    /// 所属行业
    #[serde(rename(deserialize = "所属行业"))]
    pub industry: String,
    /// 收盘价(元)
    #[serde(rename(deserialize = "收盘价"))]
    pub closing_price: f64,
    /// 最低价(元)
    #[serde(rename(deserialize = "最低价"))]
    pub lowest_price: f64,
    /// 最高价(元)
    #[serde(rename(deserialize = "最高价"))]
    pub highest_price: f64,
    /// 累计换手率，单位：百分比 (%)
    #[serde(rename(deserialize = "累计换手率"))]
    pub cumulative_turnover_rate: f64,
    /// 股票代码
    #[serde(rename(deserialize = "股票代码"))]
    pub stock_code: String,
    /// 股票简称
    #[serde(rename(deserialize = "股票简称"))]
    pub stock_name: String,
    /// 连续上涨天数
    #[serde(rename(deserialize = "连涨天数"))]
    pub consecutive_rising_days: i32,
    /// 连续涨跌幅，单位：百分比 (%)
    #[serde(rename(deserialize = "连续涨跌幅"))]
    pub consecutive_change_percentage: f64,
}

impl AkStockRankLxszThs {
    /// 从akshare获取对应的一组数据
    pub async fn from_astock_api(reqwest_client: &reqwest::Client) -> anyhow::Result<Vec<Self>> {
        let res: Vec<Self> = reqwest_client
            .get(with_base_url("/stock_rank_lxsz_ths"))
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?;
        Ok(res)
    }
}

/// akshare json schema 技术指标-连续下跌  
/// 接口：stock_rank_lxxd_ths  
/// 目标地址：https://data.10jqka.com.cn/rank/lxxd/  
/// 描述：同花顺-数据中心-技术选股-连续下跌  
/// 限量：单次返回所有数据
#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct AkStockRankLxxdThs {
    /// 序号
    #[serde(rename(deserialize = "序号"))]
    pub index: i32,
    /// 所属行业
    #[serde(rename(deserialize = "所属行业"))]
    pub industry: String,
    /// 收盘价(元)
    #[serde(rename(deserialize = "收盘价"))]
    pub closing_price: f64,
    /// 最低价(元)
    #[serde(rename(deserialize = "最低价"))]
    pub lowest_price: f64,
    /// 最高价(元)
    #[serde(rename(deserialize = "最高价"))]
    pub highest_price: f64,
    /// 累计换手率，单位：百分比 (%)
    #[serde(rename(deserialize = "累计换手率"))]
    pub cumulative_turnover_rate: f64,
    /// 股票代码
    #[serde(rename(deserialize = "股票代码"))]
    pub stock_code: String,
    /// 股票简称
    #[serde(rename(deserialize = "股票简称"))]
    pub stock_name: String,
    /// 连续上涨天数
    #[serde(rename(deserialize = "连涨天数"))]
    pub consecutive_rising_days: i32,
    /// 连续涨跌幅，单位：百分比 (%)
    #[serde(rename(deserialize = "连续涨跌幅"))]
    pub consecutive_change_percentage: f64,
}

impl AkStockRankLxxdThs {
    /// 从akshare获取对应的一组数据
    pub async fn from_astock_api(reqwest_client: &reqwest::Client) -> anyhow::Result<Vec<Self>> {
        let res: Vec<Self> = reqwest_client
            .get(with_base_url("/stock_rank_lxxd_ths"))
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?;
        Ok(res)
    }
}

/// akshare json schema 技术指标-持续放量  
/// 接口: stock_rank_cxfl_ths  
/// 目标地址: https://data.10jqka.com.cn/rank/cxfl/  
/// 描述: 同花顺-数据中心-技术选股-持续放量  
/// 限量: 单次返回所有数据
#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct AkStockRankCxflThs {
    /// 基准日成交量，格式示例："371.17万(04月18日)" (股)
    #[serde(rename(deserialize = "基准日成交量"))]
    pub base_day_volume: String,
    /// 序号
    #[serde(rename(deserialize = "序号"))]
    pub index: i32,
    /// 成交量，格式示例："861.91万" (股)
    #[serde(rename(deserialize = "成交量"))]
    pub volume: String,
    /// 所属行业
    #[serde(rename(deserialize = "所属行业"))]
    pub industry: String,
    /// 放量天数
    #[serde(rename(deserialize = "放量天数"))]
    pub volume_increase_days: i32,
    /// 最新价格(元)
    #[serde(rename(deserialize = "最新价"))]
    pub latest_price: f64,
    /// 涨跌幅，单位：百分比 (%)
    #[serde(rename(deserialize = "涨跌幅"))]
    pub change_percentage: f64,
    /// 股票代码
    #[serde(rename(deserialize = "股票代码"))]
    pub stock_code: String,
    /// 股票简称
    #[serde(rename(deserialize = "股票简称"))]
    pub stock_name: String,
    /// 阶段涨跌幅，单位：百分比 (%)
    #[serde(rename(deserialize = "阶段涨跌幅"))]
    pub stage_change_percentage: f64,
}

impl AkStockRankCxflThs {
    /// 从akshare获取到对应的数据
    pub async fn from_astock_api(reqwest_client: &reqwest::Client) -> anyhow::Result<Vec<Self>> {
        let res: Vec<Self> = reqwest_client
            .get(with_base_url("/stock_rank_cxfl_ths"))
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?;
        Ok(res)
    }
}

/// akshare json schema 技术指标-持续缩量  
/// 接口: stock_rank_cxsl_ths
/// 目标地址: https://data.10jqka.com.cn/rank/cxsl/
/// 描述: 同花顺-数据中心-技术选股-持续缩量
/// 限量: 单次返回所有数据
#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct AkStockRankCxslThs {
    /// 基准日成交量，格式示例："371.17万(04月18日)" (股)
    #[serde(rename(deserialize = "基准日成交量"))]
    pub base_day_volume: String,
    /// 序号
    #[serde(rename(deserialize = "序号"))]
    pub index: i32,
    /// 成交量，格式示例："861.91万" (股)
    #[serde(rename(deserialize = "成交量"))]
    pub volume: String,
    /// 所属行业
    #[serde(rename(deserialize = "所属行业"))]
    pub industry: String,
    /// 放量天数
    #[serde(rename(deserialize = "缩量天数"))]
    pub volume_decrease_days: i32,
    /// 最新价格(元)
    #[serde(rename(deserialize = "最新价"))]
    pub latest_price: f64,
    /// 涨跌幅，单位：百分比 (%)
    #[serde(rename(deserialize = "涨跌幅"))]
    pub change_percentage: f64,
    /// 股票代码
    #[serde(rename(deserialize = "股票代码"))]
    pub stock_code: String,
    /// 股票简称
    #[serde(rename(deserialize = "股票简称"))]
    pub stock_name: String,
    /// 阶段涨跌幅，单位：百分比 (%)
    #[serde(rename(deserialize = "阶段涨跌幅"))]
    pub stage_change_percentage: f64,
}

impl AkStockRankCxslThs {
    pub async fn from_astock_api(reqwest_client: &reqwest::Client) -> anyhow::Result<Vec<Self>> {
        let res: Vec<Self> = reqwest_client
            .get(with_base_url("/stock_rank_cxsl_ths"))
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?;
        Ok(res)
    }
}

/// stock_individual_info_em  
/// 目标地址: http://quote.eastmoney.com/concept/sh603777.html?from=classic  
/// 描述: 东方财富-个股-股票信息  
/// 限量: 单次返回指定 symbol 的个股信息
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct AkStockIndividualInfoEm {
    /// 最新价(元)
    #[serde(rename(deserialize = "最新"))]
    pub latest_price: f64,
    /// 股票代码
    #[serde(rename(deserialize = "股票代码"))]
    pub stock_code: String,
    /// 股票简称
    #[serde(rename(deserialize = "股票简称"))]
    pub stock_name: String,
    /// 总股本(元)
    #[serde(rename(deserialize = "总股本"))]
    pub total_shares: f64,
    /// 流通股
    #[serde(rename(deserialize = "流通股"))]
    pub circulating_shares: f64,
    /// 总市值(元)
    #[serde(rename(deserialize = "总市值"))]
    pub total_market_cap: f64,
    /// 流通市值(元)
    #[serde(rename(deserialize = "流通市值"))]
    pub circulating_market_cap: f64,
    /// 行业
    #[serde(rename(deserialize = "行业"))]
    pub industry: String,
    /// 上市时间
    #[serde(rename(deserialize = "上市时间"))]
    pub listing_date: i64,
}

impl AkStockIndividualInfoEm {
    pub async fn from_astock_api(
        reqwest_client: &reqwest::Client,
        stock_code: &str,
    ) -> anyhow::Result<Self> {
        #[derive(Debug, Deserialize)]
        struct ItemValue {
            item: String,
            value: Value,
        }

        let raw_json = reqwest_client
            .get(with_base_url("/stock_individual_info_em"))
            .query(&[("symbol", stock_code)])
            .send()
            .await?
            .error_for_status()?
            .text()
            .await?;

        // 1️⃣ 反序列化为 Vec<ItemValue>
        let list: Vec<ItemValue> = serde_json::from_str(&raw_json)?;

        // 2️⃣ 转换为 Map<String, Value>
        let mut map = serde_json::Map::new();
        for entry in list {
            if entry.item == "最新" {
                if let Value::String(_) = entry.value {
                    map.insert(entry.item, Value::Number(Number::from_f64(-1f64).unwrap()));
                    continue;
                }
            }
            map.insert(entry.item, entry.value);
        }

        let stock_info: Self = serde_json::from_value(Value::Object(map))?;
        Ok(stock_info)
    }
}

/// 风险警示版 - 接口: stock_zh_a_st_em  
/// 目标地址: https://quote.eastmoney.com/center/gridlist.html#st_board  
/// 描述: 东方财富网-行情中心-沪深个股-风险警示板  
/// 限量: 单次返回当前交易日风险警示板的所有股票的行情数据
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct AkStockZhAStEm {
    /// 序号
    #[serde(rename(deserialize = "序号"))]
    pub index: i64,
    /// 股票代码
    #[serde(rename(deserialize = "代码"))]
    pub code: String,
    /// 股票名称
    #[serde(rename(deserialize = "名称"))]
    pub name: String,
    /// 最新价
    #[serde(rename(deserialize = "最新价"))]
    pub latest_price: Option<f64>,
    /// 涨跌幅 (单位: %)
    #[serde(rename(deserialize = "涨跌幅"))]
    pub change_percent: Option<f64>,
    /// 涨跌额
    #[serde(rename(deserialize = "涨跌额"))]
    pub change_amount: Option<f64>,
    /// 成交量
    #[serde(rename(deserialize = "成交量"))]
    pub trading_volume: Option<f64>,
    /// 成交额
    #[serde(rename(deserialize = "成交额"))]
    pub trading_value: Option<f64>,
    /// 振幅 (单位: %)
    #[serde(rename(deserialize = "振幅"))]
    pub amplitude: Option<f64>,
    /// 最高价
    #[serde(rename(deserialize = "最高"))]
    pub highest_price: Option<f64>,
    /// 最低价
    #[serde(rename(deserialize = "最低"))]
    pub lowest_price: Option<f64>,
    /// 今日开盘价
    #[serde(rename(deserialize = "今开"))]
    pub open_price: Option<f64>,
    /// 昨日收盘价
    #[serde(rename(deserialize = "昨收"))]
    pub previous_close_price: f64,
    /// 量比
    #[serde(rename(deserialize = "量比"))]
    pub volume_ratio: Option<f64>,
    /// 换手率 (单位: %)
    #[serde(rename(deserialize = "换手率"))]
    pub turnover_rate: f64,
    /// 市盈率-动态
    #[serde(rename(deserialize = "市盈率-动态"))]
    pub pe_ratio: f64,
    /// 市净率
    #[serde(rename(deserialize = "市净率"))]
    pub pb_ratio: f64,
}

impl AkStockZhAStEm {
    pub async fn from_astock_api(reqwest_client: &reqwest::Client) -> anyhow::Result<Vec<Self>> {
        let data: Vec<Self> = reqwest_client
            .get(with_base_url("/stock_zh_a_st_em"))
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?;

        Ok(data)
    }
}

#[cfg(test)]
mod test {
    use crate::utils::TEST_HTTP_CLIENT;

    use super::*;

    #[test]
    fn test_deserde_with_fn1() {
        #[derive(Debug, Deserialize, PartialEq)]
        struct TestDeTarget {
            #[serde(deserialize_with = "false_or_null_as_none")]
            some_item: Option<f64>,
        }

        let target_json1 = r#"{"some_itme": true}"#;
        let target_json2 = r#"{"some_item": false}"#;
        let target_json3 = r#"{"some_item": null}"#;
        let target_json4 = r#"{"some_item": 1.6}"#;

        let result1 = serde_json::from_str::<TestDeTarget>(target_json1);
        assert!(result1.is_err());
        let result2 = serde_json::from_str::<TestDeTarget>(target_json2);
        assert_eq!(TestDeTarget { some_item: None }, result2.unwrap());
        let result3 = serde_json::from_str::<TestDeTarget>(target_json3);
        assert_eq!(TestDeTarget { some_item: None }, result3.unwrap());
        let result4 = serde_json::from_str::<TestDeTarget>(target_json4);
        assert_eq!(
            TestDeTarget {
                some_item: Some(1.6)
            },
            result4.unwrap()
        );
    }

    #[test]
    fn test_deserde_with_fn2() {
        #[derive(Debug, Deserialize, PartialEq)]
        struct TestDeTarget {
            #[serde(deserialize_with = "always_string")]
            some_item: String,
        }

        let target_json1 = r#"{"some_item": 1994}"#;
        let target_json2 = r#"{"some_item": "1994-12-31"}"#;

        let result1 = serde_json::from_str(target_json1);
        assert_eq!(
            TestDeTarget {
                some_item: "1994".to_owned()
            },
            result1.unwrap()
        );
        let result2 = serde_json::from_str(target_json2);
        assert_eq!(
            TestDeTarget {
                some_item: "1994-12-31".to_owned()
            },
            result2.unwrap()
        );
    }

    #[tokio::test]
    async fn test_stock_individual_info_em() {
        let data = AkStockIndividualInfoEm::from_astock_api(&TEST_HTTP_CLIENT, "603777")
            .await
            .unwrap();
        println!("{:#?}", data);
    }
}
