//! 关于A股的相关数据接口

use actix_web::{
    get,
    web::{self, Data, Json},
};
use serde::{Deserialize, Serialize};
use snafu::ResultExt;
use utoipa::IntoParams;
use utoipa_actix_web::{scope, service_config::ServiceConfig};

use crate::{
    repository::{akshare::StockAdjustmentType, service::is_stock_code_exists},
    schema::{
        akshare::{AkStockIndividualInfoEm, AkStockZhAStEm},
        common::OkRes,
        error::{BadReqSnafu, InternalServerSnafu, NotFoundSnafu, OrdinError},
        service::serv_astock::{self, StockDailyPagin},
    },
};

pub const API_TAG: &'static str = "A股量化金融数据";
pub const API_DESC: &'static str = "一组用于获取A股相关金融交易信息的数据接口";

pub fn mount_astock_scope(config: &mut ServiceConfig) {
    config.service(
        scope("/astock")
            .service(fetch_stock_individual_info)
            .service(fetch_mas_with_limit)
            .service(fetch_daily_kline)
            .service(fetch_daily_trading_volume)
            .service(fetch_daily_indicator)
            .service(fetch_astock_daily_pagin),
    );
}

/// 获取个股简单信息
#[utoipa::path(
    tag = API_TAG,
    params(
        ("stock_id", description = "需要获取个股信息对应的股票代码", example = "603777")
    ),
    responses(
        (status = 200, description = "成功获取个股信息", body = OkRes<AkStockIndividualInfoEm>),
        (status = 404, description = "对应个股信息不存在", body = OrdinError),
        (status = 401, description = "没有访问权限", body = OrdinError),
        (status = 500, description = "发生服务器内部错误", body = OrdinError),
    )
)]
#[get("/stock_individual_info/{stock_id}")]
async fn fetch_stock_individual_info(
    stock_id: web::Path<String>,
    reqwest_client: Data<reqwest::Client>,
    ch_client: Data<clickhouse::Client>,
) -> Result<Json<OkRes<AkStockIndividualInfoEm>>, OrdinError> {
    let stock_id = stock_id.into_inner();
    is_stock_code_exists(&ch_client, &stock_id)
        .await
        .context(InternalServerSnafu)?
        .then_some(())
        .ok_or(NotFoundSnafu.build())?;

    let data = AkStockIndividualInfoEm::from_astock_api(&reqwest_client, &stock_id)
        .await
        .context(InternalServerSnafu)?;

    let res = OkRes::from_with_msg("成功获取个股信息".to_owned(), data);
    Ok(Json(res))
}

#[derive(Debug, Deserialize, IntoParams)]
struct MAQuery {
    /// 需要获取移动平均线的股票代码
    #[param(example = "603777")]
    stock_id: String,
    #[param(example = "30")]
    /// 限定返回的倒推天数范围，或者说数据点数量(注意不要超过90天)
    limit_days: u32,
}

/// 获取一定时间范围内的移动平均线(MA5/MA10/MA20)数据
#[utoipa::path(
    tag = API_TAG,
    params(
        MAQuery
    ),
    responses(
        (status = 200, description = "成功获取请求的股票Id对应的时间范围内的移动平均线数据", body = OkRes<Vec<serv_astock::StockMALines>>),
        (status = 404, description = "对应个股信息不存在", body = OrdinError),
        (status = 401, description = "没有访问权限", body = OrdinError),
        (status = 500, description = "发生服务器内部错误", body = OrdinError),
    )
)]
#[get("/ma_with_limit")]
async fn fetch_mas_with_limit(
    ma_query: web::Query<MAQuery>,
    ch_client: web::Data<clickhouse::Client>,
) -> Result<Json<OkRes<Vec<serv_astock::StockMALines>>>, OrdinError> {
    is_stock_code_exists(&ch_client, &ma_query.stock_id)
        .await
        .context(InternalServerSnafu)?
        .then_some(())
        .ok_or(NotFoundSnafu.build())?;

    let data = serv_astock::StockMALines::fetch_with_limit(
        &ch_client,
        &ma_query.stock_id,
        ma_query.limit_days,
    )
    .await
    .context(InternalServerSnafu)?;

    let res = OkRes::from_with_msg(
        "成功获取请求的股票Id对应的时间范围内的移动平均线数据".to_owned(),
        data,
    );
    Ok(Json(res))
}

#[derive(Debug, Deserialize, Serialize, IntoParams)]
struct DailyStockQuery {
    /// 需要获取对应日频数据的股票代码
    #[param(example = "603777")]
    stock_id: String,
    /// 复权选项(不复权 = 0、前复权 = 1、后复权 = 2)
    #[param(example = "0")]
    adj_type: String,
    /// 从今日开始的倒推时间范围
    #[param(example = 30)]
    limit_days: u32,
}

/// 获取对应`stock_id`的A股股票从今日开始倒推一定天数的日频K线数据
#[utoipa::path(
    tag = API_TAG,
    params(
        DailyStockQuery
    ),
    responses(
        (status = 200, description = "获取对应时间范围的日频K线数据成功", body = OkRes<Vec<serv_astock::StockDailyKline>>),
        (status = 404, description = "对应个股信息不存在", body = OrdinError),
        (status = 401, description = "没有访问权限", body = OrdinError),
        (status = 500, description = "发生服务器内部错误", body = OrdinError),
    )
)]
#[get("/daily_kline")]
async fn fetch_daily_kline(
    query: web::Query<DailyStockQuery>,
    ch_client: web::Data<clickhouse::Client>,
) -> Result<Json<OkRes<Vec<serv_astock::StockDailyKline>>>, OrdinError> {
    is_stock_code_exists(&ch_client, &query.stock_id)
        .await
        .context(InternalServerSnafu)?
        .then_some(())
        .ok_or(NotFoundSnafu.build())?;

    let adj_type: StockAdjustmentType = serde_json::from_str(&query.adj_type).map_err(|_| {
        BadReqSnafu {
            desc: r#"adj_type must be choice of {"0"(不复权), "1"(前复权), "2"(后复权)}"#
                .to_owned(),
        }
        .build()
    })?;

    let data: Vec<serv_astock::StockDailyKline> = serv_astock::StockDailyKline::fetch_with_limit(
        &ch_client,
        adj_type,
        &query.stock_id,
        query.limit_days,
    )
    .await
    .context(InternalServerSnafu)?;

    let res = OkRes::from_with_msg("获取对应时间范围的日频K线数据成功".to_owned(), data);
    Ok(Json(res))
}

/// 获取对应`stock_id`的A股股票从今日开始倒推一定天数的日频交易量数据
#[utoipa::path(
    tag = API_TAG,
    params(
        DailyStockQuery
    ),
    responses(
        (status = 200, description = "获取对应时间范围的日频交易量数据成功", body = OkRes<Vec<serv_astock::StockDailyTradingVolume>>),
        (status = 404, description = "对应个股信息不存在", body = OrdinError),
        (status = 401, description = "没有访问权限", body = OrdinError),
        (status = 500, description = "发生服务器内部错误", body = OrdinError),
    )
)]
#[get("/daily_trading_volume")]
async fn fetch_daily_trading_volume(
    query: web::Query<DailyStockQuery>,
    ch_client: web::Data<clickhouse::Client>,
) -> Result<Json<OkRes<Vec<serv_astock::StockDailyTradingVolume>>>, OrdinError> {
    is_stock_code_exists(&ch_client, &query.stock_id)
        .await
        .context(InternalServerSnafu)?
        .then_some(())
        .ok_or(NotFoundSnafu.build())?;

    let adj_type: StockAdjustmentType = serde_json::from_str(&query.adj_type).map_err(|_| {
        BadReqSnafu {
            desc: r#"adj_type must be choice of {"0"(不复权), "1"(前复权), "2"(后复权)}"#
                .to_owned(),
        }
        .build()
    })?;

    let data: Vec<serv_astock::StockDailyTradingVolume> =
        serv_astock::StockDailyTradingVolume::fetch_with_limit(
            &ch_client,
            adj_type,
            &query.stock_id,
            query.limit_days,
        )
        .await
        .context(InternalServerSnafu)?;

    let res = OkRes::from_with_msg("获取对应时间范围的日频交易量数据成功".to_owned(), data);
    Ok(Json(res))
}

/// 获取对应`stock_id`的A股股票从今日开始倒推一定天数的日频交易指标数据
#[utoipa::path(
    tag = API_TAG,
    params(
        DailyStockQuery
    ),
    responses(
        (status = 200, description = "获取对应时间范围的日频交易指标数据成功", body = OkRes<Vec<serv_astock::StockDailyIndicator>>),
        (status = 404, description = "对应个股信息不存在", body = OrdinError),
        (status = 401, description = "没有访问权限", body = OrdinError),
        (status = 500, description = "发生服务器内部错误", body = OrdinError),
    )
)]
#[get("/daily_indicator")]
async fn fetch_daily_indicator(
    query: web::Query<DailyStockQuery>,
    ch_client: web::Data<clickhouse::Client>,
) -> Result<Json<OkRes<Vec<serv_astock::StockDailyIndicator>>>, OrdinError> {
    is_stock_code_exists(&ch_client, &query.stock_id)
        .await
        .context(InternalServerSnafu)?
        .then_some(())
        .ok_or(NotFoundSnafu.build())?;

    let adj_type: StockAdjustmentType = serde_json::from_str(&query.adj_type).map_err(|_| {
        BadReqSnafu {
            desc: r#"adj_type must be choice of {"0"(不复权), "1"(前复权), "2"(后复权)}"#
                .to_owned(),
        }
        .build()
    })?;

    let data: Vec<serv_astock::StockDailyIndicator> =
        serv_astock::StockDailyIndicator::fetch_with_limit(
            &ch_client,
            adj_type,
            &query.stock_id,
            query.limit_days,
        )
        .await
        .context(InternalServerSnafu)?;

    let res = OkRes::from_with_msg("获取对应时间范围的日频交易指标数据成功".to_owned(), data);
    Ok(Json(res))
}

/// 单次返回当前交易日风险警示版的所有股票的行情数据
#[utoipa::path(
    tag = API_TAG,
    responses(
        (status = 200, description = "成功获取当前交易日风险警示版的所有股票的行情数据", body = OkRes<Vec<AkStockZhAStEm>>),
        (status = 401, description = "没有访问权限", body = OrdinError),
        (status = 500, description = "发生服务器内部错误", body = OrdinError),
    )
)]
#[get("/stock_zh_a_st")]
async fn fetch_stock_zh_a_st(
    reqwest_client: Data<reqwest::Client>,
) -> Result<Json<OkRes<Vec<AkStockZhAStEm>>>, OrdinError> {
    let data = AkStockZhAStEm::from_astock_api(&reqwest_client)
        .await
        .context(InternalServerSnafu)?;

    let res = OkRes::from_with_msg(
        "成功获取当前交易日风险警示版的所有股票的行情数据".to_owned(),
        data,
    );
    Ok(Json(res))
}

/// A股每股最新日频数据获取
#[utoipa::path(
    tag = API_TAG,
    responses(
        (status = 200, description = "成功获取最新交易日A股日频分页数据", body = OkRes<Vec<StockDailyPagin>>),
        (status = 401, description = "没有访问权限", body = OrdinError),
        (status = 500, description = "发生服务器内部错误", body = OrdinError),
    )
)]
#[get("/stock_daily_pagin")]
async fn fetch_astock_daily_pagin(
    ch_client: web::Data<clickhouse::Client>,
) -> Result<Json<OkRes<Vec<StockDailyPagin>>>, OrdinError> {
    // TODO 加上分页/排序数据获取
    let data = StockDailyPagin::fetch_all(&ch_client)
        .await
        .context(InternalServerSnafu)?;

    let res = OkRes::from_with_msg("成功获取最新交易日A股日频分页数据".to_owned(), data);
    Ok(Json(res))
}
