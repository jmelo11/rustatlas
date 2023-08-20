use crate::core::marketstore::MarketStore;

use crate::{
    core::meta::{MarketData, MetaMarketData},
    time::date::Date,
};

type Scenario = Vec<Vec<MarketData>>;

struct SpotMarketDataModel {
    market_store: MarketStore,
    meta_market_data: Vec<MetaMarketData>,
}

trait Model {
    fn eval_dates(&self) -> Vec<Date>;
    fn generate_scenario(&self) -> Scenario;
    fn generate_market_data(&self, eval_date: Date) -> Vec<MarketData>;
}
