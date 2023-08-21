use std::ops::Deref;

use crate::core::marketstore::MarketStore;

use crate::core::meta::{MarketData, MetaDiscountFactor, MetaExchangeRate, MetaForwardRate};
use crate::rates::traits::YieldProvider;
use crate::{core::meta::MetaMarketData, time::date::Date};

use super::traits::{Model, Scenario};

/// # SimpleModel
/// A simple model that provides market data based in the current market state.
pub struct SimpleModel {
    market_store: MarketStore,
    meta_market_data: Vec<MetaMarketData>,
    eval_dates: Vec<Date>,
}

impl SimpleModel {
    pub fn new(market_store: MarketStore, meta_market_data: Vec<MetaMarketData>) -> SimpleModel {
        SimpleModel {
            market_store,
            meta_market_data,
            eval_dates: Vec::new(),
        }
    }

    pub fn with_eval_dates(mut self, eval_dates: Vec<Date>) -> SimpleModel {
        self.eval_dates = eval_dates;
        return self;
    }
}

impl Model for SimpleModel {
    fn eval_dates(&self) -> Vec<Date> {
        return self.eval_dates;
    }

    fn generate_df_data(&self, df: MetaDiscountFactor, eval_date: Date) -> f64 {
        let id = df.discount_curve_id();
        let date = df.reference_date();
        return 1.0;
    }

    fn generate_fwd_data(&self, fwd: MetaForwardRate, eval_date: Date) -> f64 {
        let id = fwd.forward_curve_id();
        let start_date = fwd.start_date();

        return 0.0;
    }

    fn generate_fx_data(&self, fx: MetaExchangeRate, eval_date: Date) -> f64 {
        // pending
        return 0.0;
    }

    fn generate_scenario(&self) -> Option<Scenario> {
        let mut scenario = Vec::new();
        for eval_date in self.eval_dates.iter() {
            let market_data = self.generate_market_data(*eval_date);
            scenario.push(market_data);
        }
        return Some(scenario);
    }

    fn generate_market_data(&self, eval_date: Date) -> Vec<MarketData> {
        let mut market_data = Vec::new();
        let mut results = (Option::None, Option::None, Option::None);
        for meta in self.meta_market_data.iter() {
            match meta.df() {
                Some(df) => {
                    results.0 = Some(self.generate_df_data(df, eval_date));
                }
                None => (),
            }
            match meta.fwd() {
                Some(fwd) => {
                    results.1 = Some(self.generate_fwd_data(fwd, eval_date));
                }
                None => (),
            }
            match meta.fx() {
                Some(fx) => {
                    results.2 = Some(self.generate_fx_data(fx, eval_date));
                }
                None => (),
            }
            market_data.push(MarketData::new(meta.id(), results.0, results.1, results.2));
        }
        return market_data;
    }
}
