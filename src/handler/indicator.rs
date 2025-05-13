//! a股相关技术指标的api handler

use crate::{
    repository::service::is_stock_code_exists,
    schema::{
        akshare::{
            AkStockFinancialAbstractThs, AkStockRankCxdThs, AkStockRankCxflThs, AkStockRankCxgThs,
            AkStockRankCxslThs, AkStockRankLxszThs, AkStockRankLxxdThs,
        },
        common::OkRes,
        error::{InternalServerSnafu, NotFoundSnafu, OrdinError},
    },
};
use actix_web::{
    get,
    web::{self, Data, Json},
};
use snafu::ResultExt;
use utoipa_actix_web::{scope, service_config::ServiceConfig};

pub const API_TAG: &'static str = "技术指标";
pub const API_DESC: &'static str = "相关从同花顺之中获取的股票市场技术指标，具有较强的市场概括性";

pub fn mount_tech_indicator_scope(config: &mut ServiceConfig) {
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
        (status = 200, description = "成功获取对应股票id的同花顺-财务指标-主要指标", body = OkRes<Vec<AkStockFinancialAbstractThs>>),
        (status = 404, description = "指定的股票代码或报告期不存在", body = OrdinError),
        (status = 401, description = "没有访问权限", body = OrdinError),
        (status = 500, description = "发生服务器内部错误", body = OrdinError),
    )
)]
#[get("/stock_financial_abstract_ths/{symbol_id}/{indicator}")]
async fn stock_financial_abstract_ths(
    symbol_id: web::Path<(String, String)>,
    reqwest_client: Data<reqwest::Client>,
    ch_client: Data<clickhouse::Client>,
) -> Result<Json<OkRes<Vec<AkStockFinancialAbstractThs>>>, OrdinError> {
    let (symbol_id, indicator) = symbol_id.into_inner();
    let is_stock_exist = is_stock_code_exists(&ch_client, &symbol_id)
        .await
        .context(InternalServerSnafu)?;
    let is_indicator_exist =
        indicator == "按报告期" || indicator == "按年度" || indicator == "按单季度";

    (is_stock_exist && is_indicator_exist)
        .then_some(())
        .ok_or(NotFoundSnafu.build())?;

    let data =
        AkStockFinancialAbstractThs::from_astock_api(&reqwest_client, &symbol_id, &indicator)
            .await
            .context(InternalServerSnafu)?;

    let res = OkRes::from_with_msg(
        "成功获取对应股票id的同花顺-财务指标-主要指标".to_owned(),
        data,
    );
    Ok(Json(res))
}

/// 同花顺-数据中心-技术选股-创新高
#[utoipa::path(
    tag = API_TAG,
    params(
        ("time_range", description = "指定获取数据的时间区间范围，choice of {'创月新高', '半年新高', '一年新高', '历史新高'}")
    ),
    responses(
        (status = 200, description = "成功获取对应时间范围内的同花顺-数据中心-技术选股-创新高数据", body = OkRes<Vec<AkStockRankCxgThs>>),
        (status = 401, description = "没有访问权限", body = OrdinError),
        (status = 500, description = "发生服务器内部错误", body = OrdinError),
    )
)]
#[get("/stock_rank_cxg_ths/{time_range}")]
async fn stock_rank_cxg_ths(
    time_range: web::Path<String>,
    reqwest_client: Data<reqwest::Client>,
) -> Result<Json<OkRes<Vec<AkStockRankCxgThs>>>, OrdinError> {
    let time_range = time_range.into_inner();
    let data = AkStockRankCxgThs::from_astock_api(&reqwest_client, &time_range)
        .await
        .context(InternalServerSnafu)?;

    let res = OkRes::from_with_msg(
        "成功获取对应时间范围内的同花顺-数据中心-技术选股-创新高数据".to_owned(),
        data,
    );
    Ok(Json(res))
}

/// 同花顺-数据中心-技术选股-创新低  
#[utoipa::path(
    tag = API_TAG,
    params(
        ("time_range", description = "指定获取数据的时间区间范围，choice of {'创月新低', '半年新低', '一年新低', '历史新低'}")
    ),
    responses(
        (status = 200, description = "成功获取指定时间范围内的同花顺-数据中心-技术选股-创新低数据", body = OkRes<Vec<AkStockRankCxdThs>>),
        (status = 401, description = "没有访问权限", body = OrdinError),
        (status = 500, description = "发生服务器内部错误", body = OrdinError),
    )
)]
#[get("/stock_rank_cxd_ths/{time_range}")]
async fn stock_rank_cxd_ths(
    time_range: web::Path<String>,
    reqwest_client: Data<reqwest::Client>,
) -> Result<Json<OkRes<Vec<AkStockRankCxdThs>>>, OrdinError> {
    let time_range = time_range.into_inner();
    let data = AkStockRankCxdThs::from_astock_api(&reqwest_client, &time_range)
        .await
        .context(InternalServerSnafu)?;

    let res = OkRes::from_with_msg(
        "成功获取指定时间范围内的同花顺-数据中心-技术选股-创新低数据".to_owned(),
        data,
    );
    Ok(Json(res))
}

/// 同花顺-数据中心-技术选股-连续上涨(连续上涨天数超过一周)
#[utoipa::path(
    tag = API_TAG,
    responses(
        (status = 200, description = "获取同花顺-数据中心-技术选股-连续上涨数据成功", body = OkRes<Vec<AkStockRankLxszThs>>),
        (status = 401, description = "没有访问权限", body = OrdinError),
        (status = 500, description = "发生服务器内部错误", body = OrdinError),
    )
)]
#[get("/stock_rank_lxsz_ths")]
async fn stock_rank_lxsz_ths(
    ch_client: Data<clickhouse::Client>,
) -> Result<Json<OkRes<Vec<AkStockRankLxszThs>>>, OrdinError> {
    let data = AkStockRankLxszThs::fetch_with_min_rising_days(&ch_client, 7)
        .await
        .context(InternalServerSnafu)?;

    let res = OkRes::from_with_msg(
        "获取同花顺-数据中心-技术选股-连续上涨数据成功".to_owned(),
        data,
    );
    Ok(Json(res))
}

/// 同花顺-数据中心-技术选股-连续下跌
#[utoipa::path(
    tag = API_TAG,
    responses(
        (status = 200, description = "获取同花顺-数据中心-技术选股-连续下跌数据成功", body = OkRes<Vec<AkStockRankLxxdThs>>),
        (status = 401, description = "没有访问权限", body = OrdinError),
        (status = 500, description = "发生服务器内部错误", body = OrdinError),
    )
)]
#[get("/stock_rank_lxxd_ths")]
async fn stock_rank_lxxd_ths(
    reqwest_client: Data<reqwest::Client>,
) -> Result<Json<OkRes<Vec<AkStockRankLxxdThs>>>, OrdinError> {
    let data = AkStockRankLxxdThs::from_astock_api(&reqwest_client)
        .await
        .context(InternalServerSnafu)?;

    let res = OkRes::from_with_msg(
        "获取同花顺-数据中心-技术选股-连续下跌数据成功".to_owned(),
        data,
    );
    Ok(Json(res))
}

/// 同花顺-数据中心-技术选股-持续放量
#[utoipa::path(
    tag = API_TAG,
    responses(
        (status = 200, description = "获取同花顺-数据中心-技术选股-持续放量成功", body = OkRes<Vec<AkStockRankCxflThs>>),
        (status = 401, description = "没有访问权限", body = OrdinError),
        (status = 500, description = "发生服务器内部错误", body = OrdinError),
    )
)]
#[get("/stock_rank_cxfl_ths")]
async fn stock_rank_cxfl_ths(
    reqwest_client: Data<reqwest::Client>,
) -> Result<Json<OkRes<Vec<AkStockRankCxflThs>>>, OrdinError> {
    let data = AkStockRankCxflThs::from_astock_api(&reqwest_client)
        .await
        .context(InternalServerSnafu)?;

    let res = OkRes::from_with_msg("获取同花顺-数据中心-技术选股-持续放量成功".to_owned(), data);
    Ok(Json(res))
}

/// 同花顺-数据中心-技术选股-持续缩量
#[utoipa::path(
    tag = API_TAG,
    responses(
        (status = 200, description = "获取同花顺-数据中心-技术选股-持续缩量成功", body = OkRes<Vec<AkStockRankCxslThs>>),
        (status = 401, description = "没有访问权限", body = OrdinError),
        (status = 500, description = "发生服务器内部错误", body = OrdinError),
    )
)]
#[get("/stock_rank_cxsl_ths")]
async fn stock_rank_cxsl_ths(
    reqwest_client: Data<reqwest::Client>,
) -> Result<Json<OkRes<Vec<AkStockRankCxslThs>>>, OrdinError> {
    let data = AkStockRankCxslThs::from_astock_api(&reqwest_client)
        .await
        .context(InternalServerSnafu)?;

    let res = OkRes::from_with_msg("获取同花顺-数据中心-技术选股-持续缩量成功".to_owned(), data);
    Ok(Json(res))
}
