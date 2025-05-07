-- 报警数据存储 v1 版本

CREATE TABLE IF NOT EXISTS alarm_hist
(
    `id` String,
    `event_time` DateTime('Asia/Shanghai') DEFAULT now(),
    -- protobuf数据，报警消息可能变化比较大，所以存储时使用protobuf
    `proto_data` String
)
ENGINE = MergeTree
ORDER BY (event_time, id)