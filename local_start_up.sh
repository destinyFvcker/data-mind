#!/bin/sh

# shellcheck disable=SC2046
export $(grep -v '^#' .env | xargs)
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
