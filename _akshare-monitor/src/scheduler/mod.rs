#![allow(unused)]

mod manager;
use std::sync::Arc;

pub use manager::*;
use serde::{Deserialize, Serialize};

use crate::{init::ExternalResource, monitor_tasks};

/// 当前调度任务管理器之中的所有调度任务类型
#[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq)]
pub enum ScheduleTaskType {
    /// 调度器自省任务
    System,
    /// A股数据监控任务
    AStock,
    /// 所有类型
    All,
}

impl ToString for ScheduleTaskType {
    fn to_string(&self) -> String {
        match self {
            Self::System => "System".to_owned(),
            Self::AStock => "AStock".to_owned(),
            Self::All => "All".to_owned(),
        }
    }
}

/// 在服务器启动时开启一些默认启动的调度任务
pub async fn scheduler_start_up(ext_res: ExternalResource) -> anyhow::Result<()> {
    // 加入心跳检测任务和清除zombie task任务
    SCHEDULE_TASK_MANAGER.add_task(SchedHeartBeat).await;
    SCHEDULE_TASK_MANAGER.add_task(SchedWaitZombie).await;

    monitor_tasks::start_up_monitor_tasks(ext_res).await;

    let snap_shot = SCHEDULE_TASK_MANAGER.inspect(Some(ScheduleTaskType::All));
    let message = format!("当前调度器内部状态：{:#?}", snap_shot);
    ftlog::info!("{}", message);

    Ok(())
}

/// 调度器心跳，每10分钟发回当前调度器内部状态
#[derive(Debug)]
struct SchedHeartBeat;

impl Schedulable for SchedHeartBeat {
    fn gen_meta(&self) -> manager::TaskMeta {
        TaskMeta {
            name: "调度器心跳任务".to_string(),
            desc: "心跳任务，每10分钟将调度器状态写入日志".to_owned(),
            cron_expr: "0 */10 * * * * *".to_string(),
            tag: Some(ScheduleTaskType::System),
        }
    }

    fn execute(
        self: Arc<Self>,
    ) -> Box<dyn std::future::Future<Output = anyhow::Result<()>> + Send + 'static> {
        Box::new(async move {
            let snap_shot = SCHEDULE_TASK_MANAGER.inspect(Some(ScheduleTaskType::All));
            let message = format!("当前调度器内部状态：{:#?}", snap_shot);
            ftlog::info!("{}", message);
            Ok(())
        })
    }
}

/// 清除僵尸任务，比调度器心跳要快一些，一分钟执行一次
struct SchedWaitZombie;

impl Schedulable for SchedWaitZombie {
    fn gen_meta(&self) -> TaskMeta {
        // 现在时间最短的调度任务是每分钟第0秒执行一次，这里稍微等一下，每分钟第5秒执行一次
        TaskMeta {
            name: "定时清除僵尸任务".to_string(),
            desc: "定时清除僵尸任务，每分钟的第5秒执行一次，其余的调度任务最好都设置为每xx的第0秒执行一次，方便此定时任务及时进行清理".to_owned(),
            cron_expr: "5 * * * * * *".to_string(),
            tag: Some(ScheduleTaskType::System),
        }
    }

    fn execute(
        self: Arc<Self>,
    ) -> Box<dyn std::future::Future<Output = anyhow::Result<()>> + Send + 'static> {
        Box::new(async move {
            SCHEDULE_TASK_MANAGER.wait_tasks();
            Ok(())
        })
    }
}
