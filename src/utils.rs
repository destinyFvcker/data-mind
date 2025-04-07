use sqlx::{Executor, MySqlPool};

/// 通过传入一个clickhouse客户端的引用运行一个ddl.sql文件之中所有的内容，自动对注释内容进行去除
pub async fn perform_ch_ddl(ch_client: &clickhouse::Client, raw_ddl_file: &str) {
    async fn query_ddl_by_line(ddl: String, ch_client: &clickhouse::Client) {
        let ddl: Vec<String> = ddl.split(";").map(|s| s.to_string()).collect();
        for sql in ddl.into_iter() {
            if sql.is_empty() {
                continue;
            }
            ch_client.query(&sql).execute().await.unwrap();
        }
    }

    query_ddl_by_line(clean_up(raw_ddl_file), ch_client).await;
}

/// 通过传入一个mysql客户端的引用运行一个ddl.sql文件之中的所有内容，自动对注释内容进行去除
pub async fn perform_mysql_ddl(mysql_client: &MySqlPool, raw_ddl_file: &str) {
    async fn query_ddl_by_line(ddl: String, mysql_client: &MySqlPool) {
        let ddl: Vec<&str> = ddl.split(";").collect();
        for sql in ddl {
            if sql.is_empty() {
                continue;
            }
            mysql_client
                .acquire()
                .await
                .unwrap()
                .execute(sql)
                .await
                .unwrap();
        }
    }

    query_ddl_by_line(clean_up(raw_ddl_file), mysql_client).await;
}

/// 清理传入的ddl.sql文件的内容，删除空行以及注释，返回一个单行的纯sql字符串
fn clean_up(raw_ddl_file: &str) -> String {
    raw_ddl_file
        .to_string()
        .trim()
        .lines()
        .map(|s| s.to_string())
        .filter(|line| {
            !(line.trim().starts_with("/*") || line.trim().starts_with("--") || line.is_empty())
        })
        .map(|line| match line.find("--") {
            Some(pos) => line[..pos].trim().to_owned(),
            None => line.trim().to_owned(),
        })
        .reduce(|s, line| s + " " + &line)
        .map(|str| str.trim().to_owned())
        .unwrap_or("".to_string())
}
