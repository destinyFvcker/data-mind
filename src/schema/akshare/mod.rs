//! 仅仅用于从akshare api之中获取数据，然后插入到clickhouse之中的一些
//! json schema

pub mod a_index;
pub mod a_stock;

pub use a_index::*;
pub use a_stock::*;
