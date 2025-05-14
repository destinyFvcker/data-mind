//! 钉钉自定义消息机器人客户端

use crate::{
    ding_robot::util::concat_sign_url,
    schema::dingding::{DingMarkdownMessage, DingMessage, DingTextMessage, MessageType},
};

pub async fn ding_ping(
    reqwest_client: &reqwest::Client,
    hook_addr: &str,
    secret: &str,
) -> anyhow::Result<()> {
    let hook_addr = concat_sign_url(hook_addr, secret);

    let mut body = DingMessage::default();
    body.text = Some(DingTextMessage {
        content: "ping".to_owned(),
    });
    let body_bytes = serde_json::to_string(&body).unwrap();

    let text = reqwest_client
        .post(hook_addr)
        .body(body_bytes)
        .header(
            reqwest::header::CONTENT_TYPE,
            mime::APPLICATION_JSON.as_ref(),
        )
        .send()
        .await?
        .error_for_status()?
        .text()
        .await?;

    ftlog::info!("[dingding message robot] ping res = {text}");

    Ok(())
}

pub async fn ding_text(
    reqwest_client: &reqwest::Client,
    hook_addr: &str,
    secret: &str,
    text_content: String,
) -> anyhow::Result<()> {
    let hook_addr = concat_sign_url(hook_addr, secret);

    let mut body = DingMessage::default();
    body.text = Some(DingTextMessage {
        content: text_content,
    });
    let body_bytes = serde_json::to_string(&body).unwrap();

    let text = reqwest_client
        .post(hook_addr)
        .body(body_bytes)
        .header(
            reqwest::header::CONTENT_TYPE,
            mime::APPLICATION_JSON.as_ref(),
        )
        .send()
        .await?
        .error_for_status()?
        .text()
        .await?;

    ftlog::info!("[dingding message robot] ding text res = {text}");

    Ok(())
}

pub async fn ding_markdown(
    reqwest_client: &reqwest::Client,
    hook_addr: &str,
    secret: &str,
    markdown_content: String,
    title: Option<String>,
) -> anyhow::Result<()> {
    let hook_addr = concat_sign_url(hook_addr, secret);

    let mut body = DingMessage::default();
    body.msgtype = MessageType::Markdown;
    body.markdown = Some(DingMarkdownMessage {
        text: markdown_content,
        title,
    });
    let body_bytes = serde_json::to_string(&body).unwrap();

    let text = reqwest_client
        .post(hook_addr)
        .body(body_bytes)
        .header(
            reqwest::header::CONTENT_TYPE,
            mime::APPLICATION_JSON.as_ref(),
        )
        .send()
        .await?
        .error_for_status()?
        .text()
        .await?;

    println!("markdown res = {text}");
    ftlog::info!("[dingding message robot] ding markdown res = {text}");

    Ok(())
}
