use crate::{
    core::meta::{MarketData, MetaDiscountFactor, MetaExchangeRate, MetaForwardRate},
    time::date::Date,
};

pub type Scenario = Vec<Vec<MarketData>>;
pub trait Model {
    fn eval_dates(&self) -> Vec<Date>;
    fn generate_scenario(&self) -> Option<Scenario>;
    fn generate_market_data(&self, eval_date: Date) -> Vec<MarketData>;
    fn generate_fwd_data(&self, fwd: MetaForwardRate, eval_date: Date) -> f64;
    fn generate_df_data(&self, df: MetaDiscountFactor, eval_date: Date) -> f64;
    fn generate_fx_data(&self, fx: MetaExchangeRate, eval_date: Date) -> f64;
}
