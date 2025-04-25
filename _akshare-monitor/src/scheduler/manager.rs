//! 本模块提供了一个在运行时工作的调度任务管理器，支持CRUD，对相关定时任务进行持久化的功能并不包含在本模块之中
use chrono::{DateTime, FixedOffset, Utc};
use serde::Serialize;
use std::{
    collections::HashMap,
    future::Future,
    str::FromStr,
    sync::{Arc, LazyLock, Mutex},
};
use tokio::sync::{mpsc, oneshot};
use uuid::Uuid;

use super::ScheduleTaskType;

pub const CST: FixedOffset = FixedOffset::east_opt(8 * 3600).unwrap();
pub static SCHEDULE_TASK_MANAGER: LazyLock<TaskManager> = LazyLock::new(|| TaskManager::new());

/// 指令类型定义(CRUD)
enum TaskCommand {
    Add(ScheduleTask),
    Update(ScheduleTask), // new task with the same key
    Remove(String),       // remove with the key of task
}

/// 调度任务描述性元信息
#[derive(Debug, Clone, Serialize)]
pub struct TaskMeta {
    pub name: String,
    pub desc: String,
    pub cron_expr: String,
    pub tag: Option<ScheduleTaskType>,
}

/// 调度任务展示信息
#[derive(Debug, Serialize)]
pub struct TaskMetaShow {
    pub name: String,
    pub desc: String,
    pub cron_expr: String,
    pub next_time: DateTime<FixedOffset>,
    pub is_alive: bool,
    pub tag: String,
    pub uuid: String,
}

/// 定义在调度任务之中应该做什么
pub trait Schedulable: Sync + Send + 'static {
    fn gen_key(&self) -> String {
        Uuid::new_v4().to_string()
    }
    /// 调度任务相关描述性字段
    fn gen_meta(&self) -> TaskMeta;
    /// 在一次调度之中做什么
    fn execute(self: Arc<Self>) -> Box<dyn Future<Output = anyhow::Result<()>> + Send + 'static>;
    /// 是否取消当前的这个调度任务，返回`true`代表取消，默认永远不取消
    fn cancel_or_not(self: Arc<Self>) -> Box<dyn Future<Output = bool> + Send + 'static> {
        Box::new(async { false })
    }
}

/// 调度任务
pub struct ScheduleTask {
    pub key: String,
    pub task_meta: TaskMeta,
    schedulable: Arc<dyn Schedulable>,
}

// pub type TasksScheduleMapSnapShot = HashMap<String, TaskMeta>;
pub type TasksScheduleMap = HashMap<String, (ScheduleTask, oneshot::Sender<()>)>;
/// 任务调度管理器，用于执行`TaskCommand`之中的相关命令
pub struct TaskManager {
    /// HashMap<Key, (Task, tokio task's handler)>映射到`Task`到调度任务
    tasks_map: Arc<Mutex<TasksScheduleMap>>,
    command_tx: mpsc::Sender<TaskCommand>,
}

/// private
impl TaskManager {
    /// 构建一个新的TaskManager
    fn new() -> Self {
        let (tx, rx) = mpsc::channel(100);
        let tasks_map = Arc::new(Mutex::new(HashMap::new()));

        let manager = TaskManager {
            tasks_map: Arc::clone(&tasks_map),
            command_tx: tx,
        };
        Self::start_command_processor(Arc::clone(&tasks_map), rx);
        manager
    }

    /// 开始调度任务管理循环
    fn start_command_processor(
        tasks_map: Arc<Mutex<TasksScheduleMap>>,
        mut rx: mpsc::Receiver<TaskCommand>,
    ) {
        tokio::spawn(async move {
            while let Some(cmd) = rx.recv().await {
                match cmd {
                    TaskCommand::Add(task) => Self::_add_task(Arc::clone(&tasks_map), task).await,
                    TaskCommand::Update(new_task) => {
                        Self::_update_task(Arc::clone(&tasks_map), new_task).await
                    }
                    TaskCommand::Remove(key) => {
                        Self::_cancel_task(Arc::clone(&tasks_map), key).await
                    }
                }
            }
        });
    }

    /// 添加新任务到调度任务管理器之中
    async fn _add_task(tasks_map: Arc<Mutex<TasksScheduleMap>>, task: ScheduleTask) {
        // 创建并启动tokio任务
        let handle = Self::spawn_task_handler(&task);
        tasks_map
            .lock()
            .unwrap()
            .insert(task.key.clone(), (task, handle));
    }

    /// 更新任务管理器之中存在的相关任务
    async fn _update_task(tasks_map: Arc<Mutex<TasksScheduleMap>>, new_task: ScheduleTask) {
        let mut tasks_guard = tasks_map.lock().unwrap();
        // 检查任务是否存在并获取旧任务句柄，如果没有旧的任务句柄的话，旧相当于直接新增一个任务
        if let Some((old_task, handle)) = tasks_guard.remove(&new_task.key) {
            assert_eq!(old_task.key, new_task.key);
            let _ = handle.send(()).inspect_err(|_| {
                ftlog::error!("[in update_task] send error, may be the task has been canceled!")
            });
        }
        let new_handle = Self::spawn_task_handler(&new_task);
        tasks_guard.insert(new_task.key.clone(), (new_task, new_handle));
    }

    // 取消任务管理器之中的一个任务
    async fn _cancel_task(tasks_map: Arc<Mutex<TasksScheduleMap>>, key: String) {
        // 获取并移除任务
        let task_opt = { tasks_map.lock().unwrap().remove(&key) };
        if let Some((task, handle)) = task_opt {
            assert_eq!(task.key, key);
            let _ = handle.send(()).inspect_err(|_| {
                ftlog::error!("[in cancel_task] send error, may be the task has been canceled!")
            });
        }
    }

    /// 创建调度任务
    fn spawn_task_handler(task: &ScheduleTask) -> oneshot::Sender<()> {
        let task_meta = task.task_meta.clone();
        let schedulable = Arc::clone(&task.schedulable);
        // FIXME 这个东西是网络传进来的数据，很可能是错误的，不能直接unwrap
        let schedule = cron::Schedule::from_str(&task_meta.cron_expr).unwrap();
        let (tx, rx) = oneshot::channel();

        tokio::spawn(async move {
            // let snippet = snippet_gen();
            tokio::select! {
                _ = rx => {}
                _ = async move {
                    // 使用cron表达式创建调度器
                    for next in schedule.upcoming(CST) {
                        ftlog::info!(
                            target: "scheduler::info",
                            "[meta = {:?}] 下次执行时间: {}",
                            task_meta,
                            next.format("%Y-%m-%d %H:%M")
                        );
                        if let Ok(sleep_time) = (next - cst_now()).to_std() {
                            tokio::time::sleep(sleep_time).await;
                        }

                        if Box::into_pin(schedulable.clone().cancel_or_not()).await {
                            break;
                        }
                        Box::into_pin(schedulable.clone().execute()).await;
                    }
                }  => {}
            }
        });

        tx
    }
}

/// public
impl TaskManager {
    pub async fn add_task<S: Schedulable>(&self, task: S) -> String {
        let key = task.gen_key();
        let schedule_task = ScheduleTask {
            key: key.clone(),
            task_meta: task.gen_meta(),
            schedulable: Arc::new(task),
        };

        ftlog::info!(
            "[In TaskManager add_task] task_meta = {:?}",
            schedule_task.task_meta
        );

        let _ = self
            .command_tx
            .send(TaskCommand::Add(schedule_task))
            .await
            .inspect_err(|err| {
                ftlog::error!(
                    "[in TaskManager::add_task] error occurred when add task to task manager: {}",
                    err.to_string()
                )
            });

        key
    }

    pub async fn update_task<S: Schedulable>(&self, old_key: String, new_task: S) {
        let schedule_task = ScheduleTask {
            key: old_key,
            task_meta: new_task.gen_meta(),
            schedulable: Arc::new(new_task),
        };

        ftlog::info!(
            "[In TaskManager update_task] task_meta = {:?}",
            schedule_task.task_meta
        );

        let _ = self
            .command_tx
            .send(TaskCommand::Update(schedule_task))
            .await
            .inspect_err(|err| {
                ftlog::error!(
                    "[in TaskManager::update_task] error occurred when add task to task manager: {}",
                    err.to_string()
                )
            });
    }

    pub async fn cancel_task(&self, key: String) {
        let _ = self.command_tx.send(TaskCommand::Remove(key)).await.inspect_err(|err| {
                ftlog::error!(
                    "[in TaskManager::cancel_task] error occurred when cancel task to task manager: {}",
                    err.to_string()
                )
            });
    }

    /// 清除僵尸️🧟‍♀️任务
    pub fn wait_tasks(&self) {
        let mut maps_guard = self.tasks_map.lock().unwrap();
        maps_guard.retain(|_, (_, sender)| !sender.is_closed());
    }

    pub fn inspect(&self, tag: Option<ScheduleTaskType>) -> Vec<TaskMetaShow> {
        let maps_guard = self.tasks_map.lock().unwrap();
        maps_guard
            .iter()
            .filter(|(_, (task, _))| {
                task.task_meta.tag == tag || matches!(tag, Some(ScheduleTaskType::All))
            })
            .map(|(key, (task, sender))| {
                let schedule = cron::Schedule::from_str(&task.task_meta.cron_expr).unwrap();
                let next_time = schedule.upcoming(CST).next().unwrap();
                TaskMetaShow {
                    name: task.task_meta.name.clone(),
                    desc: task.task_meta.desc.clone(),
                    cron_expr: task.task_meta.cron_expr.clone(),
                    next_time,
                    tag: task
                        .task_meta
                        .tag
                        .as_ref()
                        .map(|tag| tag.to_string())
                        .unwrap_or("None".to_owned()),
                    is_alive: !sender.is_closed(),
                    uuid: key.clone(),
                }
            })
            .collect()
    }

    pub fn inspect_by<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&TasksScheduleMap) -> R,
    {
        let maps_guard = self.tasks_map.lock().unwrap();
        f(&maps_guard)
    }
}

pub fn cst_now() -> DateTime<FixedOffset> {
    Utc::now().with_timezone(&CST)
}
