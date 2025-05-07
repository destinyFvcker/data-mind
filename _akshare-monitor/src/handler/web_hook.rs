use std::sync::Arc;

use data_mind::{
    repository::AlarmHist,
    schema::service::webhook::{AlertStatus, GrafanaWebhookMsg},
};
use poem::{
    Route, RouteMethod, handler,
    web::{Data, Json},
};
use prost::Message;
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
    Json(grafana_msg): Json<GrafanaWebhookMsg>,
    kafka_client: Data<&Arc<PartitionClient>>,
    ch_client: Data<&clickhouse::Client>,
) -> poem::Result<()> {
    ftlog::info!(target: "webhook::log", "grafana_msg = {:?}", grafana_msg);

    if AlertStatus::Firing == grafana_msg.status {
        // 首先把相关数据转换为包含关键报错信息的形态
        if let Some(alarm_msg) = grafana_msg.extract_proto_msg() {
            // 然后尝试把这个数据插入到clickhouse之中
            let mut buf = Vec::new();
            alarm_msg
                .encode(&mut buf)
                .inspect_err(|err| ftlog::error!("error occur when ser proto data = {:?}", err))
                .map_err(poem::error::InternalServerError)?;
            AlarmHist::insert(&ch_client, &alarm_msg.fingerprint, &buf)
                .await
                .inspect_err(|err| ftlog::error!("error occur when insert alarm hist = {:?}", err))
                .map_err(poem::error::InternalServerError)?;

            // 将数据发送到kafka之中
        }
    }

    Ok(())
}
