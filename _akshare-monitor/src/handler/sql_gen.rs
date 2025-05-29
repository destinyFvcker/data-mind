use data_mind::schema::coze::{AgentDesc, CozeFlowInput, CozeReqBody};
use poem::{
    Route,
    error::InternalServerError,
    handler, post,
    web::{Data, Json},
};
use reqwest::header::CONTENT_TYPE;
use serde::Deserialize;

use crate::config::INIT_CONFIG;

pub const PATH_NAME: &'static str = "/sql-gen";
pub fn sqlgen_api() -> Route {
    Route::new().at("/trigger", post(trigger_sql_gen_workflow))
}

#[handler]
async fn trigger_sql_gen_workflow(
    reqwest_client: Data<&reqwest::Client>,
    Json(sql_desc): Json<AgentDesc>,
) -> poem::Result<String> {
    let sql_desc = serde_json::to_string(&sql_desc).map_err(|err| InternalServerError(err))?;
    let work_flow_input = CozeFlowInput { input: sql_desc };

    let json_payload = serde_json::to_string(&CozeReqBody::new(
        work_flow_input,
        false,
        &INIT_CONFIG.coze.botid,
    ))
    .map_err(|err| InternalServerError(err))?;

    use reqwest::header;
    let coze_res: CozeRes = reqwest_client
        .post("https://api.coze.cn/v1/workflow/run")
        .header(header::AUTHORIZATION, &INIT_CONFIG.coze.token)
        .header(CONTENT_TYPE, "application/json")
        .body(json_payload)
        .send()
        .await
        .map_err(|err| InternalServerError(err))?
        .error_for_status()
        .map_err(|err| InternalServerError(err))?
        .json()
        .await
        .map_err(|err| InternalServerError(err))?;

    return Ok(coze_res.output);
}

#[derive(Debug, Deserialize)]
pub struct CozeRes {
    output: String,
}
