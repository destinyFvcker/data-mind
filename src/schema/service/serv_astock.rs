//! A股相关数据dto实体定义

pub use crate::repository::service::{
    DailyIndicatorFetch as ServDailyIndicator, DailyKlineFetch as ServDailyKline,
    DailyTradingVolumeFetch as ServDailyTradingVolume, MALinesFetch as ServMALines,
};
pub use crate::schema::akshare::{
    AkStockIndividualInfoEm as ServStockIndividualInfoEm, AkStockZhAStEm as ServStockZhAStEm,
};

#[cfg(test)]
mod test {
    use crate::utils::TEST_CH_CLIENT;

    use super::*;

    #[tokio::test]
    async fn test_fetch_malines() {
        let data: Vec<ServMALines> = ServMALines::fetch_with_limit(&TEST_CH_CLIENT, "603777", 90)
            .await
            .unwrap()
            .into_iter()
            .map(From::from)
            .collect();

        println!("{}", serde_json::to_string_pretty(&data).unwrap());
    }
}
