//! 钉钉自定义消息机器人客户端

use snafu::ResultExt;

use crate::{
    ding_robot::{
        error::{DingResult, NetReqSnafu, OtherSnafu},
        util::concat_sign_url,
    },
    schema::dingding::{
        DingMarkdownMessage, DingMessage, DingTalkRobotRes, DingTextMessage, MessageType,
    },
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
    ) -> DingResult<()> {
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
        ding_ping(reqwest_client, &self.webhook_address, &self.key_signature).await?;
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
            key_signature: "SECe02b21c62f1f77bfffb28b1f6fcc209813f0c63531cc1f56abb077098a659e01".to_owned()
        };

        ding_req
            .send_markdown_msg(
                &TEST_HTTP_CLIENT,
                r#"# Grafana 股票指数波动报警摘要

## 报警类型
**指数大幅波动警告(日内涨跌幅>=3%)**

## 触发条件
- 公式：`(close - open) / open ≥ ±3%`
- 数据源：`stock_zh_index_daily`（日线指数行情表）
- 监控粒度：每日

## 报警描述
检测A股主要市场指数单日显著波动，当收盘价与开盘价变动幅度超过±3%时触发。表明市场情绪快速变化，可能受宏观经济消息、政策调整或突发事件影响。

## 触发的指数及波动幅度

### 高波动（≥4%）
| 指数代码 | 波动幅度 |
|---------|---------|
| sz399805 | 4.92% |
| sz399699 | 4.19% |
| sh000683 | 4.08% |

### 中等波动（3%-4%）
| 指数代码 | 波动幅度 |
|---------|---------|
| sz399677 | 3.87% |
| sz399243 | 3.72% |
| sz399239 | 3.53% |
| sz399265 | 3.53% |
| sz399993 | 3.43% |
| sz399675 | 3.41% |
| sz399693 | 3.36% |
| sh000131 | 3.31% |
| sz399291 | 3.19% |
| sz399441 | 3.17% |
| sz399264 | 3.14% |
| sz399994 | 3.11% |

### 接近阈值（2.8%-3%）
| 指数代码 | 波动幅度 |
|---------|---------|
| sz399275 | 2.98% |
| sz399282 | 2.97% |
| sz399019 | 2.92% |
| sz399676 | 2.88% |
| sh000690 | 2.82% |
| sz980076 | 2.82% |
| sz399697 | 2.81% |

## 关键统计
- **总计触发数量**：23个指数
- **最高波动**：sz399805 (4.92%)
- **平均波动**：约3.4%
- **涉及交易所**：上海证券交易所(sh)、深圳证券交易所(sz)

## 操作链接
每个报警都包含：
- **查看详情**：Grafana报警详情页面
- **静默报警**：临时关闭特定指数的报警通知

## 风险提示
此类大幅波动通常意味着市场不确定性增强，建议结合成交量、振幅等其他指标综合判断市场走向。"#.to_owned(),
                Some("指数振幅超额报警".to_owned()),
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

    let res = reqwest_client
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
