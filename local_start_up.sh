#!/bin/sh

# shellcheck disable=SC2046
export $(grep -v '^#' .env | xargs)
# 如果存在密钥文件路径，加载密钥内容
# shellcheck disable=SC3010
if [[ -n "$COZE_KEY_PATH" ]] && [[ -f "$COZE_KEY_PATH" ]]; then
    # shellcheck disable=SC2155
    export COZE_SIGNINGKEY="$(cat "$COZE_KEY_PATH")"
    echo "已加载私钥文件：$COZE_KEY_PATH"
fi
export RUST_BACKTRACE=1
export LOCAL_DEV=1
case "$1" in
"web")
    export SERVER_LOGDIR="./logs/web-server"
    export APP_NAME="web_server"
    ;;
"coze")
    export SERVER_LOGDIR="./logs/coze-plugin"
    export APP_NAME="coze_plugin"
    ;;
"monitor")
    export SERVER_LOGDIR="./logs/akshare-monitor"
    export APP_NAME="akshare_monitor"
    ;;
*)
    echo "Nothing matched ❌"
    exit 1
    ;;
esac

mkdir -p "$SERVER_LOGDIR"
cargo run -p "$APP_NAME" -r -- --config-path ./conf/${APP_NAME}.toml
