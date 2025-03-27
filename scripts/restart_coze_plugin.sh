#!/bin/bash
APP_PID_PATH="./PID"
APP_NAME="coze_plugin"
CONF="conf/${APP_NAME}.toml"

cd ~/projects/data-mind/"$APP_NAME" || exit

if [[ -f $APP_PID_PATH ]]; then
    PID=$(cat $APP_PID_PATH)
    kill "$PID" || true
    sleep 5
else
    echo "No PID file found."
fi

nohup ./"$APP_NAME" --config-path="$CONF" >>logs/app.log 2>>logs/server.log &
echo $! >"$APP_PID_PATH"
