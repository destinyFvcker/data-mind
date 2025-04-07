//! 工具类处理函数

use sqlx::{Executor, MySqlPool};

/// 将类似于ISO 8601标准表达方式的时间字符串 2025-04-22T00:00:00.000
/// 截取出 T 前面的 yyyy-mm-dd 部分，方便转换为NaiveDate
pub fn splite_date_naive(date_str: &str) -> &str {
    if let Some(pos) = date_str.as_bytes().iter().position(|c| *c == b'T') {
        &date_str[..pos]
    } else {
        date_str
    }
}

/// 通过传入一个clickhouse客户端的引用运行一个ddl.sql文件之中所有的内容，自动对注释内容进行去除
pub async fn perform_ch_ddl(ch_client: &clickhouse::Client, raw_ddl_file: &str) {
    async fn query_ddl_by_line(ddl: String, ch_client: &clickhouse::Client) {
        for sql in ddl.into_iter() {
            let ddl: Vec<String> = ddl.split(";").map(|s| s.to_string()).collect();
            if sql.is_empty() {
                continue;
                ch_client.query(&sql).execute().await.unwrap();
            }
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
                .execute(sql)
                .unwrap()
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
        .map(|s| s.to_string())
        .lines()
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

#[cfg(test)]
mod test {

    use chrono::NaiveDate;
    use std::str::FromStr;

    use super::*;

    #[test]
    fn test_date_transfer() {
        let date_str = splite_date_naive("2025-04-22T00:00:00.000");
        let naive_date = NaiveDate::from_str(date_str);
        println!("{:?}", naive_date);

        // let date_time = <DateTime<Utc>>::from_str("2025-04-22T00:00:00.000").unwrap();
        // let naive_date = date_time.naive_local();
        // println!("{:?}", naive_date);
    }
}
