//! Coze API相关数据实体定义

use serde::{Deserialize, Serialize};

/// 向智能体提供的表格描述
#[derive(Debug, Serialize, Deserialize)]
pub struct AgentTableDesc {
    /// 表名
    pub table_name: String,
    /// 表的定义ddl
    pub ddl: String,
}

/// 智能体提示词
#[derive(Debug, Serialize, Deserialize)]
pub struct AgentDesc {
    pub admin_input: String,
    pub available_table_ddls: Vec<AgentTableDesc>,
}

/// Coze工作流输入
#[derive(Debug, Serialize, Deserialize)]
pub struct CozeFlowInput {
    pub input: String,
}

/// Coze工作流请求体
#[derive(Debug, Serialize)]
pub struct CozeReqBody<'a, T: Serialize> {
    pub parameters: T,
    pub is_async: bool,
    pub workflow_id: &'a str,
}

impl<'a, T: Serialize> CozeReqBody<'a, T> {
    pub fn new(value: T, is_async: bool, workflow_id: &'a str) -> Self {
        Self {
            parameters: value,
            is_async,
            workflow_id,
        }
    }
}
