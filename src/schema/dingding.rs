//! 钉钉报警机器人接收的相关消息体结构定义

use serde::{Deserialize, Serialize};

/// 钉钉webhook报警机器人的消息类型
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum MessageType {
    /// 普通文本类型
    Text,
    /// 链接类型
    Link,
    /// Markdown类型
    Markdown,
    /// 整体/独立跳转ActionCard类型
    ActionCard,
    /// FeedCard类型
    FeedCard,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
/// 钉钉消息机器人post方法body参数
pub struct DingMessage {
    pub msgtype: MessageType,
    pub at: Option<DingAt>,
    pub text: Option<DingTextMessage>,
    pub link: Option<DingLinkMessage>,
    pub markdown: Option<DingMarkdownMessage>,
    pub action_card: Option<DingActionCardMessage>,
    pub feed_card: Option<DingFeedCardMessage>,
}

impl Default for DingMessage {
    fn default() -> Self {
        Self {
            msgtype: MessageType::Text,
            at: None,
            text: None,
            link: None,
            markdown: None,
            action_card: None,
            feed_card: None,
        }
    }
}

/// 钉钉机器人在群内@人依据的一些参数
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DingAt {
    /// 被@人的手机号
    pub at_mobiles: Option<Vec<String>>,
    /// 被@人的用户userId
    pub at_user_ids: Option<Vec<String>>,
    /// 是否@所有人监控
    pub is_at_all: Option<bool>,
}

/// 钉钉机器人普通文本类消息
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DingTextMessage {
    /// 文本消息的内容
    pub content: String,
}

/// 钉钉机器人链接类型消息
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DingLinkMessage {
    /// 点击消息跳转的URL
    pub message_url: String,
    /// 链接消息的内容
    pub text: String,
    /// 链接消息标题
    pub title: Option<String>,
    /// 链接消息内的图片地址
    pub pic_url: Option<String>,
}

/// 钉钉机器人markdown类型消息
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DingMarkdownMessage {
    /// markdown类型消息的文本内容
    pub text: String,
    /// 消息会话列表中展示的标题，非消息体的标题
    pub title: Option<String>,
}

/// 钉钉机器人feedCard单条消息类型
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DingFeedCardLink {
    pub title: String,
    pub message_url: String,
    pub pic_url: String,
}

/// 钉钉机器人feedCard类型消息
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DingFeedCardMessage {
    /// feedCard消息的内容列表
    links: Vec<DingFeedCardLink>,
}

/// 钉钉机器人actionCard类型消息
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DingActionCardMessage {
    action_card: DingAcionCardType,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum DingAcionCardType {
    OverAll(DingOverAllJump),
    Independent(DingIndeJump),
}

/// 钉钉机器人actionCard类型按钮排列方式
#[derive(Debug, Deserialize, Serialize)]
pub enum DingActionCardOrientation {
    /// 0：按钮竖直排列
    #[serde(rename = "0")]
    Vertical,
    /// 1：按钮横向排列
    #[serde(rename = "1")]
    Horizontal,
}

/// 钉钉机器人actionCard整体跳转类型
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DingOverAllJump {
    /// 首屏会话透出的展示内容
    title: String,
    /// markdown格式的消息
    text: String,
    /// 单个按钮的标题
    single_title: String,
    /// 点击消息跳转的URL
    #[serde(rename = "singleURL")]
    single_url: String,
    /// 按钮排列方式(0/1)
    btn_orientation: Option<DingActionCardOrientation>,
}

// 钉钉机器人actionCard独立跳转类型按钮子类型
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DingIndeJumpBtn {
    /// 按钮标题
    title: String,
    /// 点击按钮触发的URL
    #[serde(rename = "actionURL")]
    action_url: String,
}

/// 钉钉机器人actionCard独立跳转类型
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DingIndeJump {
    /// 首屏会话透出的展示内容
    title: String,
    /// markdown格式的消息
    text: String,
    /// 按钮(s)
    btns: Vec<DingIndeJumpBtn>,
    /// 按钮排列方式
    btn_orientation: Option<DingActionCardOrientation>,
}

#[cfg(test)]
mod test {
    use super::DingActionCardOrientation;

    #[test]
    fn test_ser_enum_rename() {
        let enum1 = DingActionCardOrientation::Vertical;
        let enum1_str = serde_json::to_string(&enum1).unwrap();
        assert_eq!(enum1_str, "\"0\"");
    }
}
