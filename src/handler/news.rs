use actix_web::{
    get,
    web::{self, Data, Json},
};
use serde::Deserialize;
use snafu::ResultExt;
use utoipa::IntoParams;
use utoipa_actix_web::{scope, service_config::ServiceConfig};

use crate::{
    repository::service::is_stock_code_exists,
    schema::{
        common::OkRes,
        error::{InternalServerSnafu, NotFoundSnafu, OrdinError},
        service::news::{AkStockNewsEm, StockNewsMainCx},
    },
};

pub const API_TAG: &'static str = "A股金融新闻";
pub const API_DESC: &'static str = "获取个股新闻以及精选新闻的接口集合";

pub fn mount_news_scope(config: &mut ServiceConfig) {
    config.service(
        scope("/news")
            .service(fetch_recent100_news_em)
            .service(fetch_recent100_news_main_cx)
            .service(fetch_news_main_cx_with_range),
    );
}

/// 请求财经内容精选数据时使用的请求结构信息
#[derive(Debug, Deserialize, IntoParams)]
struct NewsRangeQuery {
    /// 时间区间的开始，格式为YYYY-MM-DD HH:mm:ss
    #[param(example = "2025-05-01 00:00:00")]
    start_time: String,
    /// 时间区间的结束，格式为YYYY-MM-DD HH:mm:ss
    #[param(example = "2025-05-07 23:59:59")]
    end_time: String,
    /// 用于指定结果集开始返回的偏移量，默认是从第 0 条记录开始。
    #[param(example = 0)]
    offset: u32,
    /// 限制返回结果的最大条数
    #[param(example = 10)]
    limit: u32,
}

/// 获取最近的100条精选的财经信息
#[utoipa::path(
    tag = API_TAG,
    responses(
        (status = 200, description = "成功获取最近100条精选的财经信息", body = OkRes<Vec<StockNewsMainCx>>),
        (status = 401, description = "没有访问权限", body = OrdinError),
        (status = 500, description = "发生服务器内部错误", body = OrdinError),
    )
)]
#[get("/chosen_rescent100")]
async fn fetch_recent100_news_main_cx(
    ch_client: Data<clickhouse::Client>,
) -> Result<Json<OkRes<Vec<StockNewsMainCx>>>, OrdinError> {
    let data = StockNewsMainCx::fetch_recent100(&ch_client)
        .await
        .context(InternalServerSnafu)?;
    let res = OkRes::from_with_msg("成功获取最近100条精选的财经信息".to_string(), data);
    Ok(Json(res))
}

/// 获取指定stock的最近100条财经信息
#[utoipa::path(
    tag = API_TAG,
    params(
        ("symbol_id", description = "需要获取新闻信息对应的股票代码", example = "603777")
    ),
    responses(
        (status = 200, description = "成功获取指定stock的最近100条财经信息", body = OkRes<Vec<AkStockNewsEm>>),
        (status = 404, description = "指定的股票代码不存在", body = OrdinError),
        (status = 401, description = "没有访问权限", body = OrdinError),
        (status = 500, description = "发生服务器内部错误", body = OrdinError),
    )
)]
#[get("/stock_recent100/{symbol_id}")]
async fn fetch_recent100_news_em(
    symbol_id: web::Path<String>,
    reqwest_client: Data<reqwest::Client>,
    ch_client: Data<clickhouse::Client>,
) -> Result<Json<OkRes<Vec<AkStockNewsEm>>>, OrdinError> {
    let symbol_id = symbol_id.into_inner();
    is_stock_code_exists(&ch_client, &symbol_id)
        .await
        .context(InternalServerSnafu)?
        .then_some(())
        .ok_or(NotFoundSnafu.build())?;

    let news = AkStockNewsEm::from_astock_api(&reqwest_client, &symbol_id)
        .await
        .context(InternalServerSnafu)?;

    let res = OkRes::from_with_msg("成功获取指定stock的最近100条财经信息".to_owned(), news);
    Ok(Json(res))
}

/// 通过指定时间范围、偏移量和返回数据条数以请求部分精选财经新闻条目
#[utoipa::path(
    tag = API_TAG,
    params(
        NewsRangeQuery
    ),
    responses(
        (status = 200, description = "成功获取指定范围内的精选财经新闻条目", body = OkRes<Vec<StockNewsMainCx>>),
        (status = 401, description = "没有访问权限", body = OrdinError),
        (status = 500, description = "发生服务器内部错误", body = OrdinError),
    )
)]
#[get("/chosen_with_range")]
async fn fetch_news_main_cx_with_range(
    news_range: web::Query<NewsRangeQuery>,
    ch_client: Data<clickhouse::Client>,
) -> Result<Json<OkRes<Vec<StockNewsMainCx>>>, OrdinError> {
    let data = StockNewsMainCx::fetch_range(
        &ch_client,
        &news_range.start_time,
        &news_range.end_time,
        news_range.limit,
        news_range.offset,
    )
    .await
    .context(InternalServerSnafu)?;

    let res = OkRes::from_with_msg("成功获取指定范围内的精选财经新闻条目".to_owned(), data);
    Ok(Json(res))
}
