#!/bin/bash
# 定义颜色代码，使输出更易读
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m' # 无颜色

# 定义应用程序路径
APP_DIR="$HOME/projects/data-mind/web-server"
BINARY_PATH="$APP_DIR/web_server"
CONF_PATH="$APP_DIR/conf/web_server.toml"
PID_FILE="$APP_DIR/server.pid"
LOG_FILE="$APP_DIR/logs/server.log"

# 确保日志目录存在
mkdir -p "$APP_DIR/logs"

# 输出带有时间戳的消息
log_message() {
    echo -e "[$(date '+%Y-%m-%d %H:%M:%S')] $1"
}

# 检查服务是否正在运行
check_if_running() {
    if [ -f "$PID_FILE" ]; then
        PID=$(cat "$PID_FILE")
        if ps -p "$PID" >/dev/null; then
            return 0 # 进程正在运行
        fi
    fi
    return 1 # 进程未运行
}

# 停止服务
stop_service() {
    log_message "${YELLOW}正在停止 web_server 服务...${NC}"

    if [ -f "$PID_FILE" ]; then
        PID=$(cat "$PID_FILE")
        # 首先尝试正常终止进程
        if ps -p "$PID" >/dev/null; then
            log_message "发送终止信号到进程 $PID..."
            kill "$PID"

            # 等待进程结束
            WAIT_TIME=0
            while ps -p "$PID" >/dev/null && [ $WAIT_TIME -lt 10 ]; do
                sleep 1
                WAIT_TIME=$((WAIT_TIME + 1))
                log_message "等待进程终止... ($WAIT_TIME/10)"
            done

            # 如果进程仍在运行，强制终止
            if ps -p "$PID" >/dev/null; then
                log_message "${RED}进程未响应，强制终止...${NC}"
                kill -9 "$PID" || true
            fi
        else
            log_message "${YELLOW}进程 $PID 已不存在${NC}"
        fi

        # 删除 PID 文件
        rm -f "$PID_FILE"
        log_message "${GREEN}服务已停止${NC}"
    else
        log_message "${YELLOW}未找到 PID 文件，服务可能未运行${NC}"
    fi
}

# 启动服务
start_service() {
    log_message "${YELLOW}正在启动 web_server 服务...${NC}"

    # 检查二进制文件是否存在
    if [ ! -x "$BINARY_PATH" ]; then
        log_message "${RED}错误: $BINARY_PATH 不存在或不可执行${NC}"
        return 1
    fi

    # 切换到应用目录
    cd "$APP_DIR" || {
        log_message "${RED}错误: 无法切换到目录 $APP_DIR${NC}"
        return 1
    }

    # 启动服务并将输出重定向到日志文件
    nohup "$BINARY_PATH" --config-path "$CONF_PATH" >"$APP_DIR/logs/app.log" 2>"$APP_DIR/logs/server.log" &
    NEW_PID=$!

    # 保存 PID
    echo "$NEW_PID" >"$PID_FILE"

    # 验证服务是否成功启动
    sleep 2
    if ps -p "$NEW_PID" >/dev/null; then
        log_message "${GREEN}服务已成功启动，PID: $NEW_PID${NC}"
        return 0
    else
        log_message "${RED}服务启动失败，请检查日志文件: $LOG_FILE${NC}"
        return 1
    fi
}

# 重启服务的主函数
restart_service() {
    log_message "${YELLOW}开始重启 web_server 服务...${NC}"

    # 如果服务正在运行，停止它
    if check_if_running; then
        stop_service
    else
        log_message "${YELLOW}服务当前未运行${NC}"
    fi

    # 短暂等待以确保系统资源释放
    log_message "等待 2 秒以确保资源释放..."
    sleep 2

    # 启动服务
    if start_service; then
        log_message "${GREEN}服务重启成功完成!${NC}"
        return 0
    else
        log_message "${RED}服务重启失败!${NC}"
        return 1
    fi
}

# 显示服务状态
show_status() {
    if check_if_running; then
        PID=$(cat "$PID_FILE")
        UPTIME=$(ps -o etime= -p "$PID")
        log_message "${GREEN}web_server 服务正在运行${NC}"
        log_message "PID: $PID"
        log_message "运行时间: $UPTIME"
    else
        log_message "${YELLOW}web_server 服务未运行${NC}"
    fi
}

# 处理命令行参数
case "$1" in
status)
    show_status
    ;;
start)
    if check_if_running; then
        log_message "${YELLOW}服务已经在运行中${NC}"
    else
        start_service
    fi
    ;;
stop)
    stop_service
    ;;
restart | "") # 默认行为是重启
    restart_service
    ;;
*)
    echo "用法: $0 {start|stop|restart|status}"
    exit 1
    ;;
esac

exit 0
