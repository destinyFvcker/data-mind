use actix_web::{
    get,
    web::{self, Data, Json},
};
use serde::Deserialize;
use utoipa::IntoParams;
use utoipa_actix_web::{scope, service_config::ServiceConfig};

use crate::schema::service::news::{AkStockNewsEm, StockNewsMainCx};

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
        (status = 200, description = "成功获取最近100条精选的财经信息", body = Vec<StockNewsMainCx>)
    )
)]
#[get("/chosen_rescent100")]
async fn fetch_recent100_news_main_cx(
    ch_client: Data<clickhouse::Client>,
) -> actix_web::Result<Json<Vec<StockNewsMainCx>>> {
    let data = StockNewsMainCx::fetch_recent100(&ch_client)
        .await
        .map_err(|err| actix_web::error::ErrorInternalServerError(err))?;
    Ok(Json(data))
}

/// 获取指定stock的最近100条财经信息
#[utoipa::path(
    tag = API_TAG,
    params(
        ("symbol_id", description = "需要获取新闻信息对应的股票代码", example = "603777")
    ),
    responses(
        (status = 200, description = "成功获取指定stock的最近100条财经信息", body = Vec<AkStockNewsEm>)
    )
)]
#[get("/stock_recent100/{symbol_id}")]
async fn fetch_recent100_news_em(
    symbol_id: web::Path<String>,
    reqwest_client: Data<reqwest::Client>,
) -> actix_web::Result<Json<Vec<AkStockNewsEm>>> {
    let symbol_id = symbol_id.into_inner();
    let news = AkStockNewsEm::from_astock_api(&reqwest_client, &symbol_id)
        .await
        .map_err(|err| actix_web::error::ErrorInternalServerError(err))?;
    Ok(Json(news))
}

/// 通过指定时间范围、偏移量和返回数据条数以请求部分精选财经新闻条目
#[utoipa::path(
    tag = API_TAG,
    params(
        NewsRangeQuery
    ),
    responses(
        (status = 200, description = "成功获取指定范围内的精选财经新闻条目", body = Vec<StockNewsMainCx>)
    )
)]
#[get("/chosen_with_range")]
async fn fetch_news_main_cx_with_range(
    news_range: web::Query<NewsRangeQuery>,
    ch_client: Data<clickhouse::Client>,
) -> actix_web::Result<Json<Vec<StockNewsMainCx>>> {
    let data = StockNewsMainCx::fetch_range(
        &ch_client,
        &news_range.start_time,
        &news_range.end_time,
        news_range.limit,
        news_range.offset,
    )
    .await
    .map_err(|err| actix_web::error::ErrorInternalServerError(err))?;
    Ok(Json(data))
}
