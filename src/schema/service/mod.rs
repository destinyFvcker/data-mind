//! 和akshare独立的，相关服务service自己需要的一些数据实体定义
//! 包含了一些对akshare之中数据的导出

use serde::Deserialize;

pub mod news;
pub mod serv_aindex;
pub mod serv_astock;
pub mod webhook;

/// 排序类型
#[derive(Debug, Deserialize)]
pub enum SortType {
    /// 正序排序
    Asc,
    /// 倒序排序
    Desc,
}
