//! grafana 报警消息定义
#![allow(unused)]

use chrono::{DateTime, Utc};
use serde::Deserialize;
use std::collections::HashMap;
use uuid::Uuid;

/// https://prometheus.io/docs/alerting/latest/configuration/#webhook_config  
/// Grafana报警消息响应体Json schema定义
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GrafanaWebhookMsg {
    pub title: String,
    pub version: String,
    pub group_key: String,
    pub truncated_alerts: u32,
    pub status: AlertStatus,
    pub receiver: String,
    pub group_labels: HashMap<String, String>,
    pub common_labels: HashMap<String, String>,
    pub common_annotations: HashMap<String, String>,
    #[serde(rename = "externalURL")]
    pub external_url: String,
    pub alerts: Vec<GrafanaAlert>,
    pub message: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GrafanaAlert {
    pub status: AlertStatus,
    pub labels: HashMap<String, String>,
    pub annotations: HashMap<String, String>,
    pub starts_at: DateTime<Utc>,
    pub ends_at: DateTime<Utc>,
    #[serde(rename = "generatorURL")]
    pub generator_url: String,
    pub fingerprint: String,
    pub value_string: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AlertStatus {
    Firing,
    Resolved,
}

impl GrafanaWebhookMsg {
    /// 从中提取出在proto之中定义的相关报警消息的简单摘要
    pub fn extract_proto_msg(mut self) -> Option<dm_proto::kafka_alarm::GrafanaAlert> {
        if self.alerts.is_empty() {
            return None;
        };

        let alerts = self
            .alerts
            .into_iter()
            .map(|alart| alart.value_string)
            .collect::<Vec<_>>();

        let summary = self.common_annotations.remove("summary")?;
        let description = self.common_annotations.remove("description")?;

        Some(dm_proto::kafka_alarm::GrafanaAlert {
            alertname: self.title,
            alerts,
            fingerprint: Uuid::new_v4().to_string(),
            summary,
            description,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use serde::Deserialize;

    #[derive(Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    struct MyStruct {
        #[serde(rename = "generatorURL")]
        generator_url: String,
        other_field: i32,
    }

    #[test]
    fn test_deser() {
        let json_data = r#"
    {
        "generatorURL": "https://example.com",
        "otherField": 42
    }
    "#;

        let parsed: MyStruct = serde_json::from_str(json_data).unwrap();
        println!("{:?}", parsed);
    }

    #[test]
    fn test_deser_grana() {
        let json_data = r#"
        {
	"receiver": "web-hook-test",
	"status": "resolved",
	"alerts": [
		{
			"status": "resolved",
			"labels": {
				"alertname": "指数大幅波动警告(日内涨跌幅\u003e=3%)",
				"code": "sz399020",
				"grafana_folder": "data-mind"
			},
			"annotations": {
				"description": "该报警用于检测A股主要市场指数在单个交易日内的显著波动。若某指数的收盘价与开盘价之间的变动幅度超过±3%，则表明市场情绪发生了快速变化，可能受到宏观经济消息、政策调整或突发事件的影响。此类波动通常意味着市场不确定性增强，需结合其他指标（如成交量、振幅）综合判断市场走向。\n\n触发条件：\n- (close - open) / open ≥ ±3%\n- 数据源：stock_zh_index_daily（日线指数行情表）\n- 粒度：每日",
				"grafana_state_reason": "Updated",
				"summary": "指数日内涨跌幅超过3%，可能存在重大市场情绪波动或宏观事件影响。"
			},
			"startsAt": "2025-05-06T15:57:10Z",
			"endsAt": "2025-05-06T16:11:30.01041952Z",
			"generatorURL": "https://destinyfvcker.cn/grafana/alerting/grafana/A1st0LxHk/view?orgId=1",
			"fingerprint": "bb8d3817791f1853",
			"silenceURL": "https://destinyfvcker.cn/grafana/alerting/silence/new?alertmanager=grafana\u0026matcher=__alert_rule_uid__%3DA1st0LxHk\u0026matcher=code%3Dsz399020\u0026orgId=1",
			"dashboardURL": "",
			"panelURL": "",
			"values": null,
			"valueString": "[ var='A' labels={code=sz399020} value=2.88 ]",
			"orgId": 1
		},
		{
			"status": "resolved",
			"labels": {
				"alertname": "指数大幅波动警告(日内涨跌幅\u003e=3%)",
				"code": "sz399239",
				"grafana_folder": "data-mind"
			},
			"annotations": {
				"description": "该报警用于检测A股主要市场指数在单个交易日内的显著波动。若某指数的收盘价与开盘价之间的变动幅度超过±3%，则表明市场情绪发生了快速变化，可能受到宏观经济消息、政策调整或突发事件的影响。此类波动通常意味着市场不确定性增强，需结合其他指标（如成交量、振幅）综合判断市场走向。\n\n触发条件：\n- (close - open) / open ≥ ±3%\n- 数据源：stock_zh_index_daily（日线指数行情表）\n- 粒度：每日",
				"grafana_state_reason": "Updated",
				"summary": "指数日内涨跌幅超过3%，可能存在重大市场情绪波动或宏观事件影响。"
			},
			"startsAt": "2025-05-06T15:57:10Z",
			"endsAt": "2025-05-06T16:11:30.01041952Z",
			"generatorURL": "https://destinyfvcker.cn/grafana/alerting/grafana/A1st0LxHk/view?orgId=1",
			"fingerprint": "f9f689ecf815b623",
			"silenceURL": "https://destinyfvcker.cn/grafana/alerting/silence/new?alertmanager=grafana\u0026matcher=__alert_rule_uid__%3DA1st0LxHk\u0026matcher=code%3Dsz399239\u0026orgId=1",
			"dashboardURL": "",
			"panelURL": "",
			"values": null,
			"valueString": "[ var='A' labels={code=sz399239} value=2.91 ]",
			"orgId": 1
		},
		{
			"status": "resolved",
			"labels": {
				"alertname": "指数大幅波动警告(日内涨跌幅\u003e=3%)",
				"code": "sz399242",
				"grafana_folder": "data-mind"
			},
			"annotations": {
				"description": "该报警用于检测A股主要市场指数在单个交易日内的显著波动。若某指数的收盘价与开盘价之间的变动幅度超过±3%，则表明市场情绪发生了快速变化，可能受到宏观经济消息、政策调整或突发事件的影响。此类波动通常意味着市场不确定性增强，需结合其他指标（如成交量、振幅）综合判断市场走向。\n\n触发条件：\n- (close - open) / open ≥ ±3%\n- 数据源：stock_zh_index_daily（日线指数行情表）\n- 粒度：每日",
				"grafana_state_reason": "Updated",
				"summary": "指数日内涨跌幅超过3%，可能存在重大市场情绪波动或宏观事件影响。"
			},
			"startsAt": "2025-05-06T15:57:10Z",
			"endsAt": "2025-05-06T16:11:30.01041952Z",
			"generatorURL": "https://destinyfvcker.cn/grafana/alerting/grafana/A1st0LxHk/view?orgId=1",
			"fingerprint": "874d33e77630439d",
			"silenceURL": "https://destinyfvcker.cn/grafana/alerting/silence/new?alertmanager=grafana\u0026matcher=__alert_rule_uid__%3DA1st0LxHk\u0026matcher=code%3Dsz399242\u0026orgId=1",
			"dashboardURL": "",
			"panelURL": "",
			"values": null,
			"valueString": "[ var='A' labels={code=sz399242} value=2.79 ]",
			"orgId": 1
		},
		{
			"status": "resolved",
			"labels": {
				"alertname": "指数大幅波动警告(日内涨跌幅\u003e=3%)",
				"code": "sz399264",
				"grafana_folder": "data-mind"
			},
			"annotations": {
				"description": "该报警用于检测A股主要市场指数在单个交易日内的显著波动。若某指数的收盘价与开盘价之间的变动幅度超过±3%，则表明市场情绪发生了快速变化，可能受到宏观经济消息、政策调整或突发事件的影响。此类波动通常意味着市场不确定性增强，需结合其他指标（如成交量、振幅）综合判断市场走向。\n\n触发条件：\n- (close - open) / open ≥ ±3%\n- 数据源：stock_zh_index_daily（日线指数行情表）\n- 粒度：每日",
				"grafana_state_reason": "Updated",
				"summary": "指数日内涨跌幅超过3%，可能存在重大市场情绪波动或宏观事件影响。"
			},
			"startsAt": "2025-05-06T15:57:10Z",
			"endsAt": "2025-05-06T16:11:30.01041952Z",
			"generatorURL": "https://destinyfvcker.cn/grafana/alerting/grafana/A1st0LxHk/view?orgId=1",
			"fingerprint": "ccd833bee9f57035",
			"silenceURL": "https://destinyfvcker.cn/grafana/alerting/silence/new?alertmanager=grafana\u0026matcher=__alert_rule_uid__%3DA1st0LxHk\u0026matcher=code%3Dsz399264\u0026orgId=1",
			"dashboardURL": "",
			"panelURL": "",
			"values": null,
			"valueString": "[ var='A' labels={code=sz399264} value=2.8 ]",
			"orgId": 1
		},
		{
			"status": "resolved",
			"labels": {
				"alertname": "指数大幅波动警告(日内涨跌幅\u003e=3%)",
				"code": "sz399675",
				"grafana_folder": "data-mind"
			},
			"annotations": {
				"description": "该报警用于检测A股主要市场指数在单个交易日内的显著波动。若某指数的收盘价与开盘价之间的变动幅度超过±3%，则表明市场情绪发生了快速变化，可能受到宏观经济消息、政策调整或突发事件的影响。此类波动通常意味着市场不确定性增强，需结合其他指标（如成交量、振幅）综合判断市场走向。\n\n触发条件：\n- (close - open) / open ≥ ±3%\n- 数据源：stock_zh_index_daily（日线指数行情表）\n- 粒度：每日",
				"grafana_state_reason": "Updated",
				"summary": "指数日内涨跌幅超过3%，可能存在重大市场情绪波动或宏观事件影响。"
			},
			"startsAt": "2025-05-06T15:57:10Z",
			"endsAt": "2025-05-06T16:11:30.01041952Z",
			"generatorURL": "https://destinyfvcker.cn/grafana/alerting/grafana/A1st0LxHk/view?orgId=1",
			"fingerprint": "773c9f8b75a72ea7",
			"silenceURL": "https://destinyfvcker.cn/grafana/alerting/silence/new?alertmanager=grafana\u0026matcher=__alert_rule_uid__%3DA1st0LxHk\u0026matcher=code%3Dsz399675\u0026orgId=1",
			"dashboardURL": "",
			"panelURL": "",
			"values": null,
			"valueString": "[ var='A' labels={code=sz399675} value=2.77 ]",
			"orgId": 1
		},
		{
			"status": "resolved",
			"labels": {
				"alertname": "指数大幅波动警告(日内涨跌幅\u003e=3%)",
				"code": "sz399677",
				"grafana_folder": "data-mind"
			},
			"annotations": {
				"description": "该报警用于检测A股主要市场指数在单个交易日内的显著波动。若某指数的收盘价与开盘价之间的变动幅度超过±3%，则表明市场情绪发生了快速变化，可能受到宏观经济消息、政策调整或突发事件的影响。此类波动通常意味着市场不确定性增强，需结合其他指标（如成交量、振幅）综合判断市场走向。\n\n触发条件：\n- (close - open) / open ≥ ±3%\n- 数据源：stock_zh_index_daily（日线指数行情表）\n- 粒度：每日",
				"grafana_state_reason": "Updated",
				"summary": "指数日内涨跌幅超过3%，可能存在重大市场情绪波动或宏观事件影响。"
			},
			"startsAt": "2025-05-06T15:57:10Z",
			"endsAt": "2025-05-06T16:11:30.01041952Z",
			"generatorURL": "https://destinyfvcker.cn/grafana/alerting/grafana/A1st0LxHk/view?orgId=1",
			"fingerprint": "1760ac489e090bc9",
			"silenceURL": "https://destinyfvcker.cn/grafana/alerting/silence/new?alertmanager=grafana\u0026matcher=__alert_rule_uid__%3DA1st0LxHk\u0026matcher=code%3Dsz399677\u0026orgId=1",
			"dashboardURL": "",
			"panelURL": "",
			"values": null,
			"valueString": "[ var='A' labels={code=sz399677} value=2.89 ]",
			"orgId": 1
		},
		{
			"status": "resolved",
			"labels": {
				"alertname": "指数大幅波动警告(日内涨跌幅\u003e=3%)",
				"code": "sz399694",
				"grafana_folder": "data-mind"
			},
			"annotations": {
				"description": "该报警用于检测A股主要市场指数在单个交易日内的显著波动。若某指数的收盘价与开盘价之间的变动幅度超过±3%，则表明市场情绪发生了快速变化，可能受到宏观经济消息、政策调整或突发事件的影响。此类波动通常意味着市场不确定性增强，需结合其他指标（如成交量、振幅）综合判断市场走向。\n\n触发条件：\n- (close - open) / open ≥ ±3%\n- 数据源：stock_zh_index_daily（日线指数行情表）\n- 粒度：每日",
				"grafana_state_reason": "Updated",
				"summary": "指数日内涨跌幅超过3%，可能存在重大市场情绪波动或宏观事件影响。"
			},
			"startsAt": "2025-05-06T15:57:10Z",
			"endsAt": "2025-05-06T16:11:30.01041952Z",
			"generatorURL": "https://destinyfvcker.cn/grafana/alerting/grafana/A1st0LxHk/view?orgId=1",
			"fingerprint": "9e70c96ea4a02dea",
			"silenceURL": "https://destinyfvcker.cn/grafana/alerting/silence/new?alertmanager=grafana\u0026matcher=__alert_rule_uid__%3DA1st0LxHk\u0026matcher=code%3Dsz399694\u0026orgId=1",
			"dashboardURL": "",
			"panelURL": "",
			"values": null,
			"valueString": "[ var='A' labels={code=sz399694} value=2.71 ]",
			"orgId": 1
		},
		{
			"status": "resolved",
			"labels": {
				"alertname": "指数大幅波动警告(日内涨跌幅\u003e=3%)",
				"code": "sz399805",
				"grafana_folder": "data-mind"
			},
			"annotations": {
				"description": "该报警用于检测A股主要市场指数在单个交易日内的显著波动。若某指数的收盘价与开盘价之间的变动幅度超过±3%，则表明市场情绪发生了快速变化，可能受到宏观经济消息、政策调整或突发事件的影响。此类波动通常意味着市场不确定性增强，需结合其他指标（如成交量、振幅）综合判断市场走向。\n\n触发条件：\n- (close - open) / open ≥ ±3%\n- 数据源：stock_zh_index_daily（日线指数行情表）\n- 粒度：每日",
				"grafana_state_reason": "Updated",
				"summary": "指数日内涨跌幅超过3%，可能存在重大市场情绪波动或宏观事件影响。"
			},
			"startsAt": "2025-05-06T15:57:10Z",
			"endsAt": "2025-05-06T16:11:30.01041952Z",
			"generatorURL": "https://destinyfvcker.cn/grafana/alerting/grafana/A1st0LxHk/view?orgId=1",
			"fingerprint": "8695f2497325865a",
			"silenceURL": "https://destinyfvcker.cn/grafana/alerting/silence/new?alertmanager=grafana\u0026matcher=__alert_rule_uid__%3DA1st0LxHk\u0026matcher=code%3Dsz399805\u0026orgId=1",
			"dashboardURL": "",
			"panelURL": "",
			"values": null,
			"valueString": "[ var='A' labels={code=sz399805} value=2.83 ]",
			"orgId": 1
		}
	],
	"groupLabels": { "alertname": "指数大幅波动警告(日内涨跌幅\u003e=3%)", "grafana_folder": "data-mind" },
	"commonLabels": { "alertname": "指数大幅波动警告(日内涨跌幅\u003e=3%)", "grafana_folder": "data-mind" },
	"commonAnnotations": {
		"description": "该报警用于检测A股主要市场指数在单个交易日内的显著波动。若某指数的收盘价与开盘价之间的变动幅度超过±3%，则表明市场情绪发生了快速变化，可能受到宏观经济消息、政策调整或突发事件的影响。此类波动通常意味着市场不确定性增强，需结合其他指标（如成交量、振幅）综合判断市场走向。\n\n触发条件：\n- (close - open) / open ≥ ±3%\n- 数据源：stock_zh_index_daily（日线指数行情表）\n- 粒度：每日",
		"grafana_state_reason": "Updated",
		"summary": "指数日内涨跌幅超过3%，可能存在重大市场情绪波动或宏观事件影响。"
	},
	"externalURL": "https://destinyfvcker.cn/grafana/",
	"version": "1",
	"groupKey": "{}/{__grafana_autogenerated__=\"true\"}/{__grafana_receiver__=\"web-hook-test\"}:{alertname=\"指数大幅波动警告(日内涨跌幅\u003e=3%)\", grafana_folder=\"data-mind\"}",
	"truncatedAlerts": 0,
	"orgId": 1,
	"title": "[RESOLVED] 指数大幅波动警告(日内涨跌幅\u003e=3%) data-mind ",
	"state": "ok",
	"message": "**Resolved**\n\nValue: [no value]\nLabels:\n - alertname = 指数大幅波动警告(日内涨跌幅\u003e=3%)\n - code = sz399020\n - grafana_folder = data-mind\nAnnotations:\n - description = 该报警用于检测A股主要市场指数在单个交易日内的显著波动。若某指数的收盘价与开盘价之间的变动幅度超过±3%，则表明市场情绪发生了快速变化，可能受到宏观经济消息、政策调整或突发事件的影响。此类波动通常意味着市场不确定性增强，需结合其他指标（如成交量、振幅）综合判断市场走向。\n\n触发条件：\n- (close - open) / open ≥ ±3%\n- 数据源：stock_zh_index_daily（日线指数行情表）\n- 粒度：每日\n - grafana_state_reason = Updated\n - summary = 指数日内涨跌幅超过3%，可能存在重大市场情绪波动或宏观事件影响。\nSource: https://destinyfvcker.cn/grafana/alerting/grafana/A1st0LxHk/view?orgId=1\nSilence: https://destinyfvcker.cn/grafana/alerting/silence/new?alertmanager=grafana\u0026matcher=__alert_rule_uid__%3DA1st0LxHk\u0026matcher=code%3Dsz399020\u0026orgId=1\n\nValue: [no value]\nLabels:\n - alertname = 指数大幅波动警告(日内涨跌幅\u003e=3%)\n - code = sz399239\n - grafana_folder = data-mind\nAnnotations:\n - description = 该报警用于检测A股主要市场指数在单个交易日内的显著波动。若某指数的收盘价与开盘价之间的变动幅度超过±3%，则表明市场情绪发生了快速变化，可能受到宏观经济消息、政策调整或突发事件的影响。此类波动通常意味着市场不确定性增强，需结合其他指标（如成交量、振幅）综合判断市场走向。\n\n触发条件：\n- (close - open) / open ≥ ±3%\n- 数据源：stock_zh_index_daily（日线指数行情表）\n- 粒度：每日\n - grafana_state_reason = Updated\n - summary = 指数日内涨跌幅超过3%，可能存在重大市场情绪波动或宏观事件影响。\nSource: https://destinyfvcker.cn/grafana/alerting/grafana/A1st0LxHk/view?orgId=1\nSilence: https://destinyfvcker.cn/grafana/alerting/silence/new?alertmanager=grafana\u0026matcher=__alert_rule_uid__%3DA1st0LxHk\u0026matcher=code%3Dsz399239\u0026orgId=1\n\nValue: [no value]\nLabels:\n - alertname = 指数大幅波动警告(日内涨跌幅\u003e=3%)\n - code = sz399242\n - grafana_folder = data-mind\nAnnotations:\n - description = 该报警用于检测A股主要市场指数在单个交易日内的显著波动。若某指数的收盘价与开盘价之间的变动幅度超过±3%，则表明市场情绪发生了快速变化，可能受到宏观经济消息、政策调整或突发事件的影响。此类波动通常意味着市场不确定性增强，需结合其他指标（如成交量、振幅）综合判断市场走向。\n\n触发条件：\n- (close - open) / open ≥ ±3%\n- 数据源：stock_zh_index_daily（日线指数行情表）\n- 粒度：每日\n - grafana_state_reason = Updated\n - summary = 指数日内涨跌幅超过3%，可能存在重大市场情绪波动或宏观事件影响。\nSource: https://destinyfvcker.cn/grafana/alerting/grafana/A1st0LxHk/view?orgId=1\nSilence: https://destinyfvcker.cn/grafana/alerting/silence/new?alertmanager=grafana\u0026matcher=__alert_rule_uid__%3DA1st0LxHk\u0026matcher=code%3Dsz399242\u0026orgId=1\n\nValue: [no value]\nLabels:\n - alertname = 指数大幅波动警告(日内涨跌幅\u003e=3%)\n - code = sz399264\n - grafana_folder = data-mind\nAnnotations:\n - description = 该报警用于检测A股主要市场指数在单个交易日内的显著波动。若某指数的收盘价与开盘价之间的变动幅度超过±3%，则表明市场情绪发生了快速变化，可能受到宏观经济消息、政策调整或突发事件的影响。此类波动通常意味着市场不确定性增强，需结合其他指标（如成交量、振幅）综合判断市场走向。\n\n触发条件：\n- (close - open) / open ≥ ±3%\n- 数据源：stock_zh_index_daily（日线指数行情表）\n- 粒度：每日\n - grafana_state_reason = Updated\n - summary = 指数日内涨跌幅超过3%，可能存在重大市场情绪波动或宏观事件影响。\nSource: https://destinyfvcker.cn/grafana/alerting/grafana/A1st0LxHk/view?orgId=1\nSilence: https://destinyfvcker.cn/grafana/alerting/silence/new?alertmanager=grafana\u0026matcher=__alert_rule_uid__%3DA1st0LxHk\u0026matcher=code%3Dsz399264\u0026orgId=1\n\nValue: [no value]\nLabels:\n - alertname = 指数大幅波动警告(日内涨跌幅\u003e=3%)\n - code = sz399675\n - grafana_folder = data-mind\nAnnotations:\n - description = 该报警用于检测A股主要市场指数在单个交易日内的显著波动。若某指数的收盘价与开盘价之间的变动幅度超过±3%，则表明市场情绪发生了快速变化，可能受到宏观经济消息、政策调整或突发事件的影响。此类波动通常意味着市场不确定性增强，需结合其他指标（如成交量、振幅）综合判断市场走向。\n\n触发条件：\n- (close - open) / open ≥ ±3%\n- 数据源：stock_zh_index_daily（日线指数行情表）\n- 粒度：每日\n - grafana_state_reason = Updated\n - summary = 指数日内涨跌幅超过3%，可能存在重大市场情绪波动或宏观事件影响。\nSource: https://destinyfvcker.cn/grafana/alerting/grafana/A1st0LxHk/view?orgId=1\nSilence: https://destinyfvcker.cn/grafana/alerting/silence/new?alertmanager=grafana\u0026matcher=__alert_rule_uid__%3DA1st0LxHk\u0026matcher=code%3Dsz399675\u0026orgId=1\n\nValue: [no value]\nLabels:\n - alertname = 指数大幅波动警告(日内涨跌幅\u003e=3%)\n - code = sz399677\n - grafana_folder = data-mind\nAnnotations:\n - description = 该报警用于检测A股主要市场指数在单个交易日内的显著波动。若某指数的收盘价与开盘价之间的变动幅度超过±3%，则表明市场情绪发生了快速变化，可能受到宏观经济消息、政策调整或突发事件的影响。此类波动通常意味着市场不确定性增强，需结合其他指标（如成交量、振幅）综合判断市场走向。\n\n触发条件：\n- (close - open) / open ≥ ±3%\n- 数据源：stock_zh_index_daily（日线指数行情表）\n- 粒度：每日\n - grafana_state_reason = Updated\n - summary = 指数日内涨跌幅超过3%，可能存在重大市场情绪波动或宏观事件影响。\nSource: https://destinyfvcker.cn/grafana/alerting/grafana/A1st0LxHk/view?orgId=1\nSilence: https://destinyfvcker.cn/grafana/alerting/silence/new?alertmanager=grafana\u0026matcher=__alert_rule_uid__%3DA1st0LxHk\u0026matcher=code%3Dsz399677\u0026orgId=1\n\nValue: [no value]\nLabels:\n - alertname = 指数大幅波动警告(日内涨跌幅\u003e=3%)\n - code = sz399694\n - grafana_folder = data-mind\nAnnotations:\n - description = 该报警用于检测A股主要市场指数在单个交易日内的显著波动。若某指数的收盘价与开盘价之间的变动幅度超过±3%，则表明市场情绪发生了快速变化，可能受到宏观经济消息、政策调整或突发事件的影响。此类波动通常意味着市场不确定性增强，需结合其他指标（如成交量、振幅）综合判断市场走向。\n\n触发条件：\n- (close - open) / open ≥ ±3%\n- 数据源：stock_zh_index_daily（日线指数行情表）\n- 粒度：每日\n - grafana_state_reason = Updated\n - summary = 指数日内涨跌幅超过3%，可能存在重大市场情绪波动或宏观事件影响。\nSource: https://destinyfvcker.cn/grafana/alerting/grafana/A1st0LxHk/view?orgId=1\nSilence: https://destinyfvcker.cn/grafana/alerting/silence/new?alertmanager=grafana\u0026matcher=__alert_rule_uid__%3DA1st0LxHk\u0026matcher=code%3Dsz399694\u0026orgId=1\n\nValue: [no value]\nLabels:\n - alertname = 指数大幅波动警告(日内涨跌幅\u003e=3%)\n - code = sz399805\n - grafana_folder = data-mind\nAnnotations:\n - description = 该报警用于检测A股主要市场指数在单个交易日内的显著波动。若某指数的收盘价与开盘价之间的变动幅度超过±3%，则表明市场情绪发生了快速变化，可能受到宏观经济消息、政策调整或突发事件的影响。此类波动通常意味着市场不确定性增强，需结合其他指标（如成交量、振幅）综合判断市场走向。\n\n触发条件：\n- (close - open) / open ≥ ±3%\n- 数据源：stock_zh_index_daily（日线指数行情表）\n- 粒度：每日\n - grafana_state_reason = Updated\n - summary = 指数日内涨跌幅超过3%，可能存在重大市场情绪波动或宏观事件影响。\nSource: https://destinyfvcker.cn/grafana/alerting/grafana/A1st0LxHk/view?orgId=1\nSilence: https://destinyfvcker.cn/grafana/alerting/silence/new?alertmanager=grafana\u0026matcher=__alert_rule_uid__%3DA1st0LxHk\u0026matcher=code%3Dsz399805\u0026orgId=1\n"
    }
        "#;

        let value: GrafanaWebhookMsg = serde_json::from_str(json_data).unwrap();
        println!("value = {:#?}", value);

        // let proto_msg = value.extract_proto_msg().unwrap();
        // println!("proto_msg = {:?}", proto_msg);
    }
}
