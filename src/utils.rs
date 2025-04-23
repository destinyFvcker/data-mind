//! 工具类处理函数

/// 将类似于ISO 8601标准表达方式的时间字符串 2025-04-22T00:00:00.000
/// 截取出 T 前面的 yyyy-mm-dd 部分，方便转换为NaiveDate
pub fn splite_date_naive(date_str: &str) -> &str {
    if let Some(pos) = date_str.as_bytes().iter().position(|c| *c == b'T') {
        &date_str[..pos]
    } else {
        date_str
    }
}

#[cfg(test)]
mod test {
    use std::str::FromStr;

    use chrono::NaiveDate;

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
