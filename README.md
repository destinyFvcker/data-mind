# data-mind

AI-powered Data Middleware

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
