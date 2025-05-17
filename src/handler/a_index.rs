use crate::{
    repository::service::is_index_code_exists,
    schema::{
        common::OkRes,
        error::{InternalServerSnafu, NotFoundSnafu, OrdinError},
        service::serv_aindex,
    },
};
use actix_web::{
    get,
    web::{self, Json},
};
use serde::Deserialize;
use snafu::ResultExt;
use utoipa::IntoParams;
use utoipa_actix_web::{scope, service_config::ServiceConfig};

pub const API_TAG: &'static str = "A股指数量化金融数据";
pub const API_DESC: &'static str = "一组用于获取A股指数相关金融交易信息的数据接口";

pub fn mount_aindex_scope(config: &mut ServiceConfig) {
    config.service(
        scope("/aindex")
            .service(index_option_50etf_qvix_kline)
            .service(index_option_50etf_qvix_mas)
            .service(stock_zh_index_daily_kline)
            .service(stock_zh_index_daily_mas)
            .service(stock_zh_index_daily_volume)
            .service(stock_zh_index_daily_pagin),
    );
}

/// 普通限定请求体
#[derive(Debug, Deserialize, IntoParams)]
struct LimitQuery {
    /// 限定返回的数据天数(日频数据), 设定为`-1`
    /// 来返回所有数据
    #[param(example = 30)]
    limit: i32,
}

/// 50ETF 期权波动率指数 QVIX K线数据
#[utoipa::path(
    tag = API_TAG,
    params(
        LimitQuery
    ),
    responses(
        (status = 200, description = "成功获取限定时间范围内的50ETF期权波动率指数QVIX K线数据", body = OkRes<Vec<serv_aindex::IndexOption50EtfQvixKline>>),
        (status = 401, description = "没有访问权限", body = OrdinError),
        (status = 500, description = "发生服务器内部错误", body = OrdinError),
    )
)]
#[get("/index_option_50etf_qvix_kline")]
async fn index_option_50etf_qvix_kline(
    query: web::Query<LimitQuery>,
    ch_client: web::Data<clickhouse::Client>,
) -> Result<Json<OkRes<Vec<serv_aindex::IndexOption50EtfQvixKline>>>, OrdinError> {
    let data = serv_aindex::IndexOption50EtfQvixKline::fetch_with_limit(&ch_client, query.limit)
        .await
        .context(InternalServerSnafu)?;

    let res = OkRes::from_with_msg(
        "成功获取限定时间范围内的50ETF期权波动率指数QVIX K线数据".to_owned(),
        data,
    );
    Ok(Json(res))
}

/// 50ETF 期权波动率指数 QVIX 5日/10日/20日移动平均线数据
#[utoipa::path(
    tag = API_TAG,
    params(
        LimitQuery
    ),
    responses(
        (status = 200, description = "成功获取限定时间范围内50ETF期权波动率指数QVIX5日/10日/20日移动平均线数据", body = OkRes<Vec<serv_aindex::IndexOption50EtfQvixMA>>),
        (status = 401, description = "没有访问权限", body = OrdinError),
        (status = 500, description = "发生服务器内部错误", body = OrdinError),
    )
)]
#[get("/index_option_50etf_qvix_mas")]
async fn index_option_50etf_qvix_mas(
    query: web::Query<LimitQuery>,
    ch_client: web::Data<clickhouse::Client>,
) -> Result<Json<OkRes<Vec<serv_aindex::IndexOption50EtfQvixMA>>>, OrdinError> {
    let data = serv_aindex::IndexOption50EtfQvixMA::fetch_with_limit(&ch_client, query.limit)
        .await
        .context(InternalServerSnafu)?;

    let res = OkRes::from_with_msg(
        "成功获取限定时间范围内50ETF期权波动率指数QVIX5日/10日/20日移动平均线数据".to_owned(),
        data,
    );
    Ok(Json(res))
}

/// 带指定指数代码的限定请求体
#[derive(Debug, Deserialize, IntoParams)]
struct LimitQueryWithCode {
    /// 指数代码(带交易所2位编号)
    #[param(example = "sz399982")]
    index_code: String,
    /// 限定返回的数据天数(日频数据), 设定为`-1`
    /// 来返回所有数据
    #[param(example = 30)]
    limit: i32,
}

/// 获取指定指数代码的日频K线数据
#[utoipa::path(
    tag = API_TAG,
    params(
        LimitQueryWithCode
    ),
    responses(
        (status = 200, description = "成功获取指定指数代码的日频K线数据", body = OkRes<Vec<serv_aindex::StockZhIndexDailyKline>>),
        (status = 404, description = "对应个股信息不存在", body = OrdinError),
        (status = 401, description = "没有访问权限", body = OrdinError),
        (status = 500, description = "发生服务器内部错误", body = OrdinError),
    )
)]
#[get("/stock_zh_index_daily_kline")]
async fn stock_zh_index_daily_kline(
    query: web::Query<LimitQueryWithCode>,
    ch_client: web::Data<clickhouse::Client>,
) -> Result<Json<OkRes<Vec<serv_aindex::StockZhIndexDailyKline>>>, OrdinError> {
    is_index_code_exists(&ch_client, &query.index_code)
        .await
        .context(InternalServerSnafu)?
        .then_some(())
        .ok_or(NotFoundSnafu.build())?;

    let data = serv_aindex::StockZhIndexDailyKline::fetch_with_limit(
        &ch_client,
        &query.index_code,
        query.limit,
    )
    .await
    .context(InternalServerSnafu)?;

    let res = OkRes::from_with_msg("成功获取指定指数代码的日频K线数据".to_owned(), data);
    Ok(Json(res))
}

/// 获取指定指数代码的移动平均线数据
#[utoipa::path(
    tag = API_TAG,
    params(
        LimitQueryWithCode
    ),
    responses(
        (status = 200, description = "成功获取指定的指数代码的移动平均线数据", body = Vec<serv_aindex::StockZhIndexDailyMA>),
        (status = 404, description = "对应个股信息不存在", body = OrdinError),
        (status = 401, description = "没有访问权限", body = OrdinError),
        (status = 500, description = "发生服务器内部错误", body = OrdinError),
    )
)]
#[get("/stock_zh_index_daily_mas")]
async fn stock_zh_index_daily_mas(
    query: web::Query<LimitQueryWithCode>,
    ch_client: web::Data<clickhouse::Client>,
) -> Result<Json<OkRes<Vec<serv_aindex::StockZhIndexDailyMA>>>, OrdinError> {
    is_index_code_exists(&ch_client, &query.index_code)
        .await
        .context(InternalServerSnafu)?
        .then_some(())
        .ok_or(NotFoundSnafu.build())?;

    let data = serv_aindex::StockZhIndexDailyMA::fetch_with_limit(
        &ch_client,
        &query.index_code,
        query.limit,
    )
    .await
    .context(InternalServerSnafu)?;

    let res = OkRes::from_with_msg("成功获取指定的指数代码的移动平均线数据".to_owned(), data);
    Ok(Json(res))
}

/// 获取指定指数代码的日频交易量数据
#[utoipa::path(
    tag = API_TAG,
    params(
        LimitQueryWithCode
    ),
    responses(
        (status = 200, description = "成功获取指定的指数代码的日频交易量数据", body = OkRes<Vec<serv_aindex::StockZhIndexDailyVolume>>),
        (status = 404, description = "对应个股信息不存在", body = OrdinError),
        (status = 401, description = "没有访问权限", body = OrdinError),
        (status = 500, description = "发生服务器内部错误", body = OrdinError),
    )
)]
#[get("/stock_zh_index_daily_volume")]
async fn stock_zh_index_daily_volume(
    query: web::Query<LimitQueryWithCode>,
    ch_client: web::Data<clickhouse::Client>,
) -> Result<Json<OkRes<Vec<serv_aindex::StockZhIndexDailyVolume>>>, OrdinError> {
    is_index_code_exists(&ch_client, &query.index_code)
        .await
        .context(InternalServerSnafu)?
        .then_some(())
        .ok_or(NotFoundSnafu.build())?;

    let data = serv_aindex::StockZhIndexDailyVolume::fetch_with_limit(
        &ch_client,
        &query.index_code,
        query.limit,
    )
    .await
    .context(InternalServerSnafu)?;

    let res = OkRes::from_with_msg("成功获取指定的指数代码的日频交易量数据".to_owned(), data);
    Ok(Json(res))
}

/// 普通分页请求体，假如传入的参数有一个为空则获取所有数据
#[derive(Debug, Deserialize, IntoParams)]
struct PaginQuery {
    /// 请求某页的页索引(从1开始)
    #[param(example = 1)]
    page_index: Option<u32>,
    /// 每页的大小
    #[param(example = 50)]
    page_size: Option<u32>,
}

/// 分页获取所有指数最新一个交易日的交易信息
#[utoipa::path(
    tag = API_TAG,
    params(
        PaginQuery
    ),
    responses(
        (status = 200, description = "成功分页获取对应交易日的交易信息", body = OkRes<Vec<serv_aindex::StockZhIndexDailyPagin>>),
        (status = 401, description = "没有访问权限", body = OrdinError),
        (status = 500, description = "发生服务器内部错误", body = OrdinError), 
    )
)]
#[get("/stock_zh_index_daily_pagin")]
async fn stock_zh_index_daily_pagin(
    query: web::Query<PaginQuery>,
    ch_client: web::Data<clickhouse::Client>,
) -> Result<Json<OkRes<Vec<serv_aindex::StockZhIndexDailyPagin>>>, OrdinError> {
    let data = match (query.page_index, query.page_size) {
        (Some(page_index), Some(page_size)) => {
            serv_aindex::StockZhIndexDailyPagin::fetch_paginate(&ch_client, page_size, page_index)
                .await
                .context(InternalServerSnafu)?
        }
        _ => serv_aindex::StockZhIndexDailyPagin::fetch_all(&ch_client)
            .await
            .context(InternalServerSnafu)?,
    };

    let res = OkRes::from_with_msg("成功分页获取对应交易日的交易信息".to_owned(), data);
    Ok(Json(res))
}
