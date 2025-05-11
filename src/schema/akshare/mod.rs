//! 仅仅用于从akshare api之中获取数据，然后插入到clickhouse之中的一些
//! json schema

pub mod ak_aindex;
pub mod ak_astock;

pub use ak_aindex::*;
pub use ak_astock::*;
