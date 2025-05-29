#![allow(unused)]

mod manager;
use std::sync::Arc;

pub use manager::*;
use serde::{Deserialize, Serialize};

use crate::{init::ExternalResource, tasks};

/// 当前调度任务管理器之中的所有调度任务类型
#[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq)]
pub enum ScheduleTaskType {
    /// 调度器自省任务
    System,
    /// A股数据监控任务
    AStock,
    /// 指数监控任务
    Index,
    /// 所有类型
    All,
}

impl ToString for ScheduleTaskType {
    fn to_string(&self) -> String {
        match self {
            Self::System => "System".to_owned(),
            Self::AStock => "AStock".to_owned(),
            Self::Index => "Index".to_owned(),
            Self::All => "All".to_owned(),
        }
    }
}

/// 在服务器启动时开启一些默认启动的调度任务
pub async fn scheduler_start_up(ext_res: ExternalResource) -> anyhow::Result<()> {
    tasks::start_up_monitor_tasks(ext_res).await;

    let snap_shot = SCHEDULE_TASK_MANAGER.inspect(Some(ScheduleTaskType::All));
    let message = format!("当前调度器内部状态：{:#?}", snap_shot);
    ftlog::info!(target: "scheduler::info", "{}", message);

    Ok(())
}
