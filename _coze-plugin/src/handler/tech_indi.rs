//! a股相关技术指标的api handler

use actix_web::{
    get,
    web::{self, Data, Json},
};
use data_mind::schema::akshare::{
    StockFinancialAbstractThs, StockRankCxdThs, StockRankCxflThs, StockRankCxgThs,
    StockRankCxslThs, StockRankLxszThs, StockRankLxxdThs,
};
use utoipa_actix_web::{scope, service_config::ServiceConfig};

pub const API_TAG: &'static str = "技术指标";
pub const API_DESC: &'static str = "相关从同花顺之中获取的股票市场技术指标，具有较强的市场概括性";

pub fn mount_tech_indicator_scopr(config: &mut ServiceConfig) {
    config.service(
        scope("/tech_indicator")
            .service(stock_financial_abstract_ths)
            .service(stock_rank_cxd_ths)
            .service(stock_rank_cxg_ths)
            .service(stock_rank_cxfl_ths)
            .service(stock_rank_cxsl_ths)
            .service(stock_rank_lxsz_ths)
            .service(stock_rank_lxxd_ths),
    );
}

/// 同花顺-财务指标-主要指标  
#[utoipa::path(
    tag = API_TAG,
    params(
        ("symbol_id", description = "想要获取数据的对应股票id"),
        ("indicator", description = "获取数据时间范围指示器参数，example = '按报告期'; choice of {'按报告期', '按年度', '按单季度'}")
    ),
    responses(
        (status = 200, description = "成功获取对应股票id的同花顺-财务指标-主要指标", body = Vec<StockFinancialAbstractThs>)
    )
)]
#[get("/stock_financial_abstract_ths/{symbol_id}/{indicator}")]
async fn stock_financial_abstract_ths(
    symbol_id: web::Path<(String, String)>,
    reqwest_client: Data<reqwest::Client>,
) -> actix_web::Result<Json<Vec<StockFinancialAbstractThs>>> {
    let (symbol_id, indicator) = symbol_id.into_inner();
    let data = StockFinancialAbstractThs::from_astock_api(&reqwest_client, &symbol_id, &indicator)
        .await
        .map_err(|err| actix_web::error::ErrorInternalServerError(err))?;
    Ok(Json(data))
}

/// 同花顺-数据中心-技术选股-创新高
#[utoipa::path(
    tag = API_TAG,
    params(
        ("time_range", description = "指定获取数据的时间区间范围，choice of {'创月新高', '半年新高', '一年新高', '历史新高'}")
    ),
    responses(
        (status = 200, description = "成功获取对应时间范围内的同花顺-数据中心-技术选股-创新高数据", body = Vec<StockRankCxgThs>)
    )
)]
#[get("/stock_rank_cxg_ths/{time_range}")]
async fn stock_rank_cxg_ths(
    time_range: web::Path<String>,
    reqwest_client: Data<reqwest::Client>,
) -> actix_web::Result<Json<Vec<StockRankCxgThs>>> {
    let time_range = time_range.into_inner();
    let data = StockRankCxgThs::from_astock_api(&reqwest_client, &time_range)
        .await
        .map_err(|err| actix_web::error::ErrorInternalServerError(err))?;
    Ok(Json(data))
}

/// 同花顺-数据中心-技术选股-创新低  
#[utoipa::path(
    tag = API_TAG,
    params(
        ("time_range", description = "指定获取数据的时间区间范围，choice of {'创月新低', '半年新低', '一年新低', '历史新低'}")
    ),
    responses(
        (status = 200, description = "成功获取指定时间范围内的同花顺-数据中心-技术选股-创新低数据", body = Vec<StockRankCxdThs>)
    )
)]
#[get("/stock_rank_cxd_ths/{time_range}")]
async fn stock_rank_cxd_ths(
    time_range: web::Path<String>,
    reqwest_client: Data<reqwest::Client>,
) -> actix_web::Result<Json<Vec<StockRankCxdThs>>> {
    let time_range = time_range.into_inner();
    let data = StockRankCxdThs::from_astock_api(&reqwest_client, &time_range)
        .await
        .map_err(|err| actix_web::error::ErrorInternalServerError(err))?;
    Ok(Json(data))
}

/// 同花顺-数据中心-技术选股-连续上涨(连续上涨天数超过一周)
#[utoipa::path(
    tag = API_TAG,
    responses(
        (status = 200, description = "获取同花顺-数据中心-技术选股-连续上涨数据成功", body = Vec<StockRankLxszThs>)
    )
)]
#[get("/stock_rank_lxsz_ths")]
async fn stock_rank_lxsz_ths(
    ch_client: Data<clickhouse::Client>,
) -> actix_web::Result<Json<Vec<StockRankLxszThs>>> {
    let data = StockRankLxszThs::fetch_with_min_rising_days(&ch_client, 7)
        .await
        .map_err(|err| actix_web::error::ErrorInternalServerError(err))?;
    Ok(Json(data))
}

/// 同花顺-数据中心-技术选股-连续下跌
#[utoipa::path(
    tag = API_TAG,
    responses(
        (status = 200, description = "获取同花顺-数据中心-技术选股-连续下跌数据成功", body = Vec<StockRankLxxdThs>)
    )
)]
#[get("/stock_rank_lxxd_ths")]
async fn stock_rank_lxxd_ths(
    reqwest_client: Data<reqwest::Client>,
) -> actix_web::Result<Json<Vec<StockRankLxxdThs>>> {
    let data = StockRankLxxdThs::from_astock_api(&reqwest_client)
        .await
        .map_err(|err| actix_web::error::ErrorInternalServerError(err))?;
    Ok(Json(data))
}

/// 同花顺-数据中心-技术选股-持续放量
#[utoipa::path(
    tag = API_TAG,
    responses(
        (status = 200, description = "获取同花顺-数据中心-技术选股-持续放量成功", body = Vec<StockRankCxflThs>)
    )
)]
#[get("/stock_rank_cxfl_ths")]
async fn stock_rank_cxfl_ths(
    reqwest_client: Data<reqwest::Client>,
) -> actix_web::Result<Json<Vec<StockRankCxflThs>>> {
    let data = StockRankCxflThs::from_astock_api(&reqwest_client)
        .await
        .map_err(|err| actix_web::error::ErrorInternalServerError(err))?;
    Ok(Json(data))
}

/// 同花顺-数据中心-技术选股-持续缩量
#[utoipa::path(
    tag = API_TAG,
    responses(
        (status = 200, description = "获取同花顺-数据中心-技术选股-持续缩量成功", body = Vec<StockRankCxslThs>)
    )
)]
#[get("/stock_rank_cxsl_ths")]
async fn stock_rank_cxsl_ths(
    reqwest_client: Data<reqwest::Client>,
) -> actix_web::Result<Json<Vec<StockRankCxslThs>>> {
    let data = StockRankCxslThs::from_astock_api(&reqwest_client)
        .await
        .map_err(|err| actix_web::error::ErrorInternalServerError(err))?;
    Ok(Json(data))
}
