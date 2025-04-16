use std::fmt::Debug;

use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct CommonResponse<T: Serialize + Debug> {
    pub code: u16,
    pub msg: String,
    pub data: T,
}

impl<T: Serialize + Debug> CommonResponse<T> {
    pub fn new(code: u16, msg: String, data: T) -> Self {
        Self { code, msg, data }
    }
}
