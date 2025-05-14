# data-mind

AI-powered Data Middleware

## 钉钉报警机器人接入

1. 关于自定义机器人发送群消息，见[文档a](https://open.dingtalk.com/document/orgapp/custom-robots-send-group-messages)
2. 关于自定义机器人安全设置，见[文档b](https://open.dingtalk.com/document/robots/customize-robot-security-settings)

## monitor

查看现有调度任务

```shell
curl -v "http://localhost:18803/scheduler?tag=All" | jq
```

手动触发一个调度任务内容

```shell
# task uuid可以通过上面的“查看现有调度任务获得”
curl -v localhost:18803/scheduler/:task_uuid
```

## for local dev

在使用`docker compose up`启动项目相关基础设置之前，(假如没有的话)请务必先在**项目根目录**下创建一个`.env`文件，并放入下面这些环境变量定义：

- `CLICKHOUSE_USER`: clickhouse root用户的用户名，一般为*default*
- `CLICKHOUSE_PASSWORD`: clickhouse root用户密码
- `MYSQL_ROOT_PASSWORD`: mysql root用户密码
- `REDIS_PASSWORD`: redis 用户密码

安全起见，这些环境变量在部署时都放在github action的"Secrets and varialbles"之中。

并且在相关可执行文件启动时，将这些环境变量通过github action注入到对应的进程之中。

本地开发时可以使用`local_start_up.sh`来快速启动相关项目的本地测试版本。

## utils

ch sql to check disk usage

```sql
SELECT
    database,
    table,
    formatReadableSize(sum(data_compressed_bytes) AS size) AS compressed,
    formatReadableSize(sum(data_uncompressed_bytes) AS usize) AS uncompressed,
    round(usize / size, 2) AS compr_rate,
    sum(rows) AS rows,
    count() AS part_count
FROM system.parts
WHERE (active = 1) AND (database LIKE '%') AND (table LIKE '%')
GROUP BY
    database,
    table
ORDER BY size DESC;
```

查看有关单个表的元数据

```sql
SELECT
    part_type,
    path,
    formatReadableQuantity(rows) AS rows,
    formatReadableSize(data_uncompressed_bytes) AS data_uncompressed_bytes,
    formatReadableSize(data_compressed_bytes) AS data_compressed_bytes,
    formatReadableSize(primary_key_bytes_in_memory) AS primary_key_bytes_in_memory,
    marks,
    formatReadableSize(bytes_on_disk) AS bytes_on_disk
FROM system.parts
WHERE (table = 'your_table_name') AND (active = 1)
FORMAT Vertical;
```
