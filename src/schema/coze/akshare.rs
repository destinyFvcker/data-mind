use serde::{Deserialize, Deserializer};
use serde_json::Value;

/// 关键指标-同花顺 `stock_financial_abstract_ths`  
/// 目标地址: https://basic.10jqka.com.cn/new/000063/finance.html  
/// 描述: 同花顺-财务指标-主要指标  
/// 限量: 单次获取指定 symbol 的所有数据  
#[derive(Debug, Deserialize)]
pub struct StockFinancialAbstractThs {
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

// ---------------------------------------------------------------------------------

/// 财经内容精选 `stock_news_main_cx`
/// 目标地址: https://cxdata.caixin.com/pc/
/// 描述: 财新网-财新数据通-内容精选
/// 限量: 返回所有历史新闻数据
pub struct StockNewsMainCx {
    //
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

#[cfg(test)]
mod test {
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
}
