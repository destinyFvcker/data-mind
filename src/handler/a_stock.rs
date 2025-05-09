//! 关于A股的相关数据接口

use actix_web::{
    get,
    web::{self, Data, Json},
};
use serde::Deserialize;
use utoipa::IntoParams;
use utoipa_actix_web::{scope, service_config::ServiceConfig};

use crate::{
    repository::MALinesRepo,
    schema::service::a_stock::{MALines, StockIndividualInfoEm},
};

pub const API_TAG: &'static str = "A股量化金融数据";
pub const API_DESC: &'static str = "一组用于获取A股相关金融交易信息的数据接口";

pub fn mount_astock_scope(config: &mut ServiceConfig) {
    config.service(
        scope("/astock")
            .service(fetch_stock_individual_info)
            .service(fetch_mas_with_limit),
    );
}

/// 获取个股简单信息
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

#[derive(Debug, Deserialize, IntoParams)]
struct MAQuery {
    /// 需要获取移动平均线的股票代码
    #[param(example = "603777")]
    stock_id: String,
    #[param(example = "30")]
    /// 限定返回的天数，或者说数据点数量(注意不要超过90天)
    limit_days: u32,
}

/// 获取一定时间范围内的移动平均线(MA5/MA10/MA20)数据
#[utoipa::path(
    tag = API_TAG,
    params(
        MAQuery
    ),
    responses(
        (status = 200, description = "成功获取请求的股票Id对应的时间范围内的移动平均线数据", body = Vec<MALines>)
    )
)]
#[get("/ma_with_limit")]
async fn fetch_mas_with_limit(
    ma_query: web::Query<MAQuery>,
    ch_client: web::Data<clickhouse::Client>,
) -> actix_web::Result<Json<Vec<MALines>>> {
    let data: Vec<MALines> =
        MALinesRepo::fetch_with_limit(&ch_client, &ma_query.stock_id, ma_query.limit_days)
            .await
            .map_err(|err| actix_web::error::ErrorInternalServerError(err))?
            .into_iter()
            .map(From::from)
            .collect();
    Ok(Json(data))
}
