use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use chrono::Utc;
use hmac::{Hmac, Mac};
use sha2::Sha256;
use urlencoding::encode;

type HmacSha256 = Hmac<Sha256>;

/// 生成钉钉签名  
/// - `secret`: 钉钉提供的密钥
/// - `timestamp`: 当前时间戳
///
/// 返回签名后的字符串（已进行 URL 编码）
pub(super) fn generate_dingtalk_signature(secret: &str, timestamp: i64) -> String {
    // 具体流程见https://open.dingtalk.com/document/robots/customize-robot-security-settings
    //️ 拼接待签名字符串：`timestamp\nsecret`
    let string_to_sign = format!("{}\n{}", timestamp, secret);

    let mut mac =
        HmacSha256::new_from_slice(secret.as_bytes()).expect("HMAC can take key of any size");
    mac.update(string_to_sign.as_bytes());
    let result = mac.finalize().into_bytes();
    let base64_signature = STANDARD.encode(result);
    encode(&base64_signature).to_string()
}

/// 拿到开发服务内当前系统 timestamp 和加密 sign 签名值，将 timestamp 和 sign 拼接到 URL 中
#[inline]
pub(super) fn concat_sign_url(hook_addr: &str, secret: &str) -> String {
    let timestamp = Utc::now().timestamp_millis();
    let signature = generate_dingtalk_signature(secret, timestamp);

    format!("{}&timestamp={}&sign={}", hook_addr, timestamp, signature)
}
