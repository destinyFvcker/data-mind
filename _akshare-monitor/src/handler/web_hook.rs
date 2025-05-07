use std::sync::Arc;

use data_mind::schema::service::webhook::GrafanaWebhookMsg;
use poem::{
    Route, RouteMethod, handler,
    web::{Data, Json},
};
use rskafka::client::partition::PartitionClient;

pub const PATH_NAME: &'static str = "/hook";
pub fn web_hook_api() -> Route {
    Route::new().at(
        PATH_NAME,
        RouteMethod::new().post(grafana_alarm_hook), // 获取现在所有正在运行的调度任务
    )
}

#[handler]
async fn grafana_alarm_hook(
    grafana_msg: Json<GrafanaWebhookMsg>,
    kafka_client: Data<&Arc<PartitionClient>>,
) -> poem::Result<()> {
    ftlog::info!(target: "webhook::log", "{}", kafka_client.partition());

    Ok(())
}
