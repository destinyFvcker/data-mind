//! 钉钉自定义消息机器人客户端

use snafu::ResultExt;

use crate::{
    ding_robot::{
        error::{DingResult, NetReqSnafu, OtherSnafu},
        util::concat_sign_url,
    },
    schema::dingding::{DingMarkdownMessage, DingMessage, DingTalkRobotRes, DingTextMessage, MessageType},
};

/// 发送钉钉报警消息所需要的完整信息
#[derive(Debug)]
pub struct DingTalkRobotReq {
    pub webhook_address: String,
    pub key_signature: String,
}

impl DingTalkRobotReq {
    pub async fn send_text_msg(
        &self,
        reqwest_client: &reqwest::Client,
        msg: String,
    ) -> DingResult<()>{
        ding_text(
            reqwest_client,
            &self.webhook_address,
            &self.key_signature,
            msg,
        )
        .await?;
        Ok(())
    }

    pub async fn send_markdown_msg(
        &self,
        reqwest_client: &reqwest::Client,
        markdown_content: String,
        title: Option<String>,
    ) -> DingResult<()> {
        ding_markdown(
            reqwest_client,
            &self.webhook_address,
            &self.key_signature,
            markdown_content,
            title,
        )
        .await?;
        Ok(())
    }

    pub async fn ping(&self, reqwest_client: &reqwest::Client) -> DingResult<()> {
        ding_ping(
            reqwest_client,
            &self.webhook_address,
            &self.key_signature,
        )
        .await?;
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::utils::TEST_HTTP_CLIENT;

    use super::*;

    #[tokio::test]
    async fn test_hook_msg() {
        let ding_req = DingTalkRobotReq {
            webhook_address: "https://oapi.dingtalk.com/robot/send?access_token=2692ebf5fa7eea41c3c36496e73dfce34e81603b48996adf5bbeeded893633af".to_owned(),
            key_signature: "SECe02b21c62f1f77bfffb28b1f6fcc209813f0c63531cc1f56abb077098a659".to_owned()
        };

        ding_req.send_markdown_msg(
            &TEST_HTTP_CLIENT, 
            "**Resolved**\n\nValue: [no value]\nLabels:\n - alertname = 指数大幅波动警告(日内涨跌幅>=3%)\n - code = sz399020\n - grafana_folder = data-mind\nAnnotations:\n - description = 该报警用于检测A股主要市场指数在单个交易日内的显著波动。若某指数的收盘价与开盘价之间的变动幅度超过±3%，则表明市场情绪发生了快速变化，可能受到宏观经济消息、政策调整或突发事件的影响。此类波动通常意味着市场不确定性增强，需结合其他指标（如成交量、振幅）综合判断市场走向。\n\n触发条件：\n- (close - open) / open ≥ ±3%\n- 数据源：stock_zh_index_daily（日线指数行情表）\n- 粒度：每日\n - grafana_state_reason = Updated\n - summary = 指数日内涨跌幅超过3%，可能存在重大市场情绪波动或宏观事件影响。\nSource: https://destinyfvcker.cn/grafana/alerting/grafana/A1st0LxHk/view?orgId=1\nSilence: https://destinyfvcker.cn/grafana/alerting/silence/new?alertmanager=grafana&matcher=__alert_rule_uid__%3DA1st0LxHk&matcher=code%3Dsz399020&orgId=1\n\nValue: [no value]\nLabels:\n - alertname = 指数大幅波动警告(日内涨跌幅>=3%)\n - code = sz399239\n - grafana_folder = data-mind\nAnnotations:\n - description = 该报警用于检测A股主要市场指数在单个交易日内的显著波动。若某指数的收盘价与开盘价之间的变动幅度超过±3%，则表明市场情绪发生了快速变化，可能受到宏观经济消息、政策调整或突发事件的影响。此类波动通常意味着市场不确定性增强，需结合其他指标（如成交量、振幅）综合判断市场走向。\n\n触发条件：\n- (close - open) / open ≥ ±3%\n- 数据源：stock_zh_index_daily（日线指数行情表）\n- 粒度：每日\n - grafana_state_reason = Updated\n - summary = 指数日内涨跌幅超过3%，可能存在重大市场情绪波动或宏观事件影响。\nSource: https://destinyfvcker.cn/grafana/alerting/grafana/A1st0LxHk/view?orgId=1\nSilence: https://destinyfvcker.cn/grafana/alerting/silence/new?alertmanager=grafana&matcher=__alert_rule_uid__%3DA1st0LxHk&matcher=code%3Dsz399239&orgId=1\n\nValue: [no value]\nLabels:\n - alertname = 指数大幅波动警告(日内涨跌幅>=3%)\n - code = sz399242\n - grafana_folder = data-mind\nAnnotations:\n - description = 该报警用于检测A股主要市场指数在单个交易日内的显著波动。若某指数的收盘价与开盘价之间的变动幅度超过±3%，则表明市场情绪发生了快速变化，可能受到宏观经济消息、政策调整或突发事件的影响。此类波动通常意味着市场不确定性增强，需结合其他指标（如成交量、振幅）综合判断市场走向。\n\n触发条件：\n- (close - open) / open ≥ ±3%\n- 数据源：stock_zh_index_daily（日线指数行情表）\n- 粒度：每日\n - grafana_state_reason = Updated\n - summary = 指数日内涨跌幅超过3%，可能存在重大市场情绪波动或宏观事件影响。\nSource: https://destinyfvcker.cn/grafana/alerting/grafana/A1st0LxHk/view?orgId=1\nSilence: https://destinyfvcker.cn/grafana/alerting/silence/new?alertmanager=grafana&matcher=__alert_rule_uid__%3DA1st0LxHk&matcher=code%3Dsz399242&orgId=1\n\nValue: [no value]\nLabels:\n - alertname = 指数大幅波动警告(日内涨跌幅>=3%)\n - code = sz399264\n - grafana_folder = data-mind\nAnnotations:\n - description = 该报警用于检测A股主要市场指数在单个交易日内的显著波动。若某指数的收盘价与开盘价之间的变动幅度超过±3%，则表明市场情绪发生了快速变化，可能受到宏观经济消息、政策调整或突发事件的影响。此类波动通常意味着市场不确定性增强，需结合其他指标（如成交量、振幅）综合判断市场走向。\n\n触发条件：\n- (close - open) / open ≥ ±3%\n- 数据源：stock_zh_index_daily（日线指数行情表）\n- 粒度：每日\n - grafana_state_reason = Updated\n - summary = 指数日内涨跌幅超过3%，可能存在重大市场情绪波动或宏观事件影响。\nSource: https://destinyfvcker.cn/grafana/alerting/grafana/A1st0LxHk/view?orgId=1\nSilence: https://destinyfvcker.cn/grafana/alerting/silence/new?alertmanager=grafana&matcher=__alert_rule_uid__%3DA1st0LxHk&matcher=code%3Dsz399264&orgId=1\n\nValue: [no value]\nLabels:\n - alertname = 指数大幅波动警告(日内涨跌幅>=3%)\n - code = sz399675\n - grafana_folder = data-mind\nAnnotations:\n - description = 该报警用于检测A股主要市场指数在单个交易日内的显著波动。若某指数的收盘价与开盘价之间的变动幅度超过±3%，则表明市场情绪发生了快速变化，可能受到宏观经济消息、政策调整或突发事件的影响。此类波动通常意味着市场不确定性增强，需结合其他指标（如成交量、振幅）综合判断市场走向。\n\n触发条件：\n- (close - open) / open ≥ ±3%\n- 数据源：stock_zh_index_daily（日线指数行情表）\n- 粒度：每日\n - grafana_state_reason = Updated\n - summary = 指数日内涨跌幅超过3%，可能存在重大市场情绪波动或宏观事件影响。\nSource: https://destinyfvcker.cn/grafana/alerting/grafana/A1st0LxHk/view?orgId=1\nSilence: https://destinyfvcker.cn/grafana/alerting/silence/new?alertmanager=grafana&matcher=__alert_rule_uid__%3DA1st0LxHk&matcher=code%3Dsz399675&orgId=1\n\nValue: [no value]\nLabels:\n - alertname = 指数大幅波动警告(日内涨跌幅>=3%)\n - code = sz399677\n - grafana_folder = data-mind\nAnnotations:\n - description = 该报警用于检测A股主要市场指数在单个交易日内的显著波动。若某指数的收盘价与开盘价之间的变动幅度超过±3%，则表明市场情绪发生了快速变化，可能受到宏观经济消息、政策调整或突发事件的影响。此类波动通常意味着市场不确定性增强，需结合其他指标（如成交量、振幅）综合判断市场走向。\n\n触发条件：\n- (close - open) / open ≥ ±3%\n- 数据源：stock_zh_index_daily（日线指数行情表）\n- 粒度：每日\n - grafana_state_reason = Updated\n - summary = 指数日内涨跌幅超过3%，可能存在重大市场情绪波动或宏观事件影响。\nSource: https://destinyfvcker.cn/grafana/alerting/grafana/A1st0LxHk/view?orgId=1\nSilence: https://destinyfvcker.cn/grafana/alerting/silence/new?alertmanager=grafana&matcher=__alert_rule_uid__%3DA1st0LxHk&matcher=code%3Dsz399677&orgId=1\n\nValue: [no value]\nLabels:\n - alertname = 指数大幅波动警告(日内涨跌幅>=3%)\n - code = sz399694\n - grafana_folder = data-mind\nAnnotations:\n - description = 该报警用于检测A股主要市场指数在单个交易日内的显著波动。若某指数的收盘价与开盘价之间的变动幅度超过±3%，则表明市场情绪发生了快速变化，可能受到宏观经济消息、政策调整或突发事件的影响。此类波动通常意味着市场不确定性增强，需结合其他指标（如成交量、振幅）综合判断市场走向。\n\n触发条件：\n- (close - open) / open ≥ ±3%\n- 数据源：stock_zh_index_daily（日线指数行情表）\n- 粒度：每日\n - grafana_state_reason = Updated\n - summary = 指数日内涨跌幅超过3%，可能存在重大市场情绪波动或宏观事件影响。\nSource: https://destinyfvcker.cn/grafana/alerting/grafana/A1st0LxHk/view?orgId=1\nSilence: https://destinyfvcker.cn/grafana/alerting/silence/new?alertmanager=grafana&matcher=__alert_rule_uid__%3DA1st0LxHk&matcher=code%3Dsz399694&orgId=1\n\nValue: [no value]\nLabels:\n - alertname = 指数大幅波动警告(日内涨跌幅>=3%)\n - code = sz399805\n - grafana_folder = data-mind\nAnnotations:\n - description = 该报警用于检测A股主要市场指数在单个交易日内的显著波动。若某指数的收盘价与开盘价之间的变动幅度超过±3%，则表明市场情绪发生了快速变化，可能受到宏观经济消息、政策调整或突发事件的影响。此类波动通常意味着市场不确定性增强，需结合其他指标（如成交量、振幅）综合判断市场走向。\n\n触发条件：\n- (close - open) / open ≥ ±3%\n- 数据源：stock_zh_index_daily（日线指数行情表）\n- 粒度：每日\n - grafana_state_reason = Updated\n - summary = 指数日内涨跌幅超过3%，可能存在重大市场情绪波动或宏观事件影响。\nSource: https://destinyfvcker.cn/grafana/alerting/grafana/A1st0LxHk/view?orgId=1\nSilence: https://destinyfvcker.cn/grafana/alerting/silence/new?alertmanager=grafana&matcher=__alert_rule_uid__%3DA1st0LxHk&matcher=code%3Dsz399805&orgId=1\n".to_owned(), 
            Some("报警解除".to_owned())
        )
        .await
        .unwrap();
    }
}


/// 向指定的webhook地址发送ding测试消息
pub async fn ding_ping(
    reqwest_client: &reqwest::Client,
    hook_addr: &str,
    secret: &str,
) -> DingResult<()> {
    let hook_addr = concat_sign_url(hook_addr, secret);

    let mut body = DingMessage::default();
    body.text = Some(DingTextMessage {
        content: "ping".to_owned(),
    });
    let body_bytes = serde_json::to_string(&body).unwrap();

    let res: DingTalkRobotRes = reqwest_client
        .post(hook_addr)
        .body(body_bytes)
        .header(
            reqwest::header::CONTENT_TYPE,
            mime::APPLICATION_JSON.as_ref(),
        )
        .send()
        .await
        .context(NetReqSnafu)?
        .error_for_status()
        .context(NetReqSnafu)?
        .json()
        .await
        .map_err(|err| {
            OtherSnafu {
                reason: format!("报警机器人响应体解析错误: {}", err),
            }
            .build()
        })?;

    if let Some(err) = super::error::Error::from_error_res(res) {
        Err(err)
    } else {
        Ok(())
    }
}

/// 向指定的webhook地址发送一个纯文本消息
pub async fn ding_text(
    reqwest_client: &reqwest::Client,
    hook_addr: &str,
    secret: &str,
    text_content: String,
) -> DingResult<()> {
    let hook_addr = concat_sign_url(hook_addr, secret);

    let mut body = DingMessage::default();
    body.text = Some(DingTextMessage {
        content: text_content,
    });
    let body_bytes = serde_json::to_string(&body).unwrap();

    let res: DingTalkRobotRes = reqwest_client
        .post(hook_addr)
        .body(body_bytes)
        .header(
            reqwest::header::CONTENT_TYPE,
            mime::APPLICATION_JSON.as_ref(),
        )
        .send()
        .await
        .context(NetReqSnafu)?
        .error_for_status()
        .context(NetReqSnafu)?
        .json()
        .await
        .map_err(|err| {
            OtherSnafu {
                reason: format!("报警机器人响应体解析错误: {}", err),
            }
            .build()
        })?;

    if let Some(err) = super::error::Error::from_error_res(res) {
        Err(err)
    } else {
        Ok(())
    }
}

/// 向指定的webhook地址发送一个markdown格式的消息
pub async fn ding_markdown(
    reqwest_client: &reqwest::Client,
    hook_addr: &str,
    secret: &str,
    markdown_content: String,
    title: Option<String>,
) -> DingResult<()> {
    let hook_addr = concat_sign_url(hook_addr, secret);

    let mut body = DingMessage::default();
    body.msgtype = MessageType::Markdown;
    body.markdown = Some(DingMarkdownMessage {
        text: markdown_content,
        title,
    });
    let body_bytes = serde_json::to_string(&body).unwrap();

    let res: DingTalkRobotRes = reqwest_client
        .post(hook_addr)
        .body(body_bytes)
        .header(
            reqwest::header::CONTENT_TYPE,
            mime::APPLICATION_JSON.as_ref(),
        )
        .send()
        .await
        .context(NetReqSnafu)?
        .error_for_status()
        .context(NetReqSnafu)?
        .json()
        .await
        .map_err(|err| {
            OtherSnafu {
                reason: format!("报警机器人响应体解析错误: {}", err),
            }
            .build()
        })?;

    if let Some(err) = super::error::Error::from_error_res(res) {
        Err(err)
    } else {
        Ok(())
    }
}
