use poem::{
    Route, RouteMethod, get, handler,
    web::{Json, Path, Query},
};
use serde::Deserialize;

use crate::scheduler::{SCHEDULE_TASK_MANAGER, ScheduleTaskType, TaskMetaShow};

pub const PATH_NAME: &'static str = "/scheduler";
pub fn scheduler_api() -> Route {
    Route::new()
        // 同时处理数据库和内存调度器api
        .at(
            PATH_NAME,
            RouteMethod::new().get(schedule_inspect_api), // 获取现在所有正在运行的调度任务
        )
        .at(format!("/{}/:task_id", PATH_NAME), get(trigger_task_api))
}

#[derive(Debug, Deserialize)]
struct InspectQuery {
    tag: Option<ScheduleTaskType>,
}

#[handler]
async fn schedule_inspect_api(
    Query(InspectQuery { tag }): Query<InspectQuery>,
) -> Json<Vec<TaskMetaShow>> {
    Json(SCHEDULE_TASK_MANAGER.inspect(tag))
}

#[handler]
async fn trigger_task_api(Path(task_id): Path<String>) {
    // println!("task_id = {task_id}")
    SCHEDULE_TASK_MANAGER.trigger_task(task_id).await;
}
