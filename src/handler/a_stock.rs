//! 关于A股的相关数据接口

use actix_web::{
    get,
    web::{self, Data, Json},
};
use utoipa_actix_web::{scope, service_config::ServiceConfig};

use crate::schema::service::a_stock::StockIndividualInfoEm;

pub const API_TAG: &'static str = "A股量化金融数据";
pub const API_DESC: &'static str = "一组用于获取A股相关金融交易信息的数据接口";

pub fn mount_astock_scope(config: &mut ServiceConfig) {
    config.service(scope("/astock").service(fetch_stock_individual_info));
}

#[utoipa::path(
    tag = API_TAG,
    params(
        ("symbol_id", description = "需要获取个股信息对应的股票代码", example = "603777")
    ),
    responses(
        (status = 200, description = "成功获取个股信息", body = StockIndividualInfoEm),
        (status = 404, description = "对应个股信息不存在")
    )
)]
#[get("/stock_individual_info/{stock_id}")]
async fn fetch_stock_individual_info(
    stock_id: web::Path<String>,
    reqwest_client: Data<reqwest::Client>,
) -> actix_web::Result<Json<StockIndividualInfoEm>> {
    let stock_id = stock_id.into_inner();
    let data = StockIndividualInfoEm::from_astock_api(&reqwest_client, &stock_id)
        .await
        .map_err(|err| actix_web::error::ErrorNotFound(err))?;
    Ok(Json(data))
}
