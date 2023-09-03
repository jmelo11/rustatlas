use crate::core::marketstore::MarketStore;

use crate::core::meta::*;
use crate::rates::traits::YieldProvider;
use crate::time::date::Date;

use super::traits::Model;

/// # SimpleModel
/// A simple model that provides market data based in the current market state.
///
/// ## Parameters
/// * `market_store` - The market store.
/// * `meta_market_data` - The meta market data.
#[derive(Clone)]
pub struct SimpleModel {
    market_store: MarketStore,
}

impl SimpleModel {
    pub fn new(market_store: MarketStore) -> SimpleModel {
        SimpleModel { market_store }
    }
}

impl Model for SimpleModel {
    fn gen_df_data(&self, df: DiscountFactorRequest, eval_date: Date) -> f64 {
        let date = df.date();
        if eval_date > date {
            return 0.0;
        } else if eval_date == date {
            return 1.0;
        }
        let id: usize = df.provider_id();
        let index = self.market_store.get_index_by_id(id);
        match index {
            Some(curve) => {
                return curve.discount_factor(date);
            }
            None => panic!("No curve found for id {}", id),
        };
    }

    fn gen_fwd_data(&self, fwd: ForwardRateRequest, _eval_date: Date) -> f64 {
        let id = fwd.provider_id();
        let index = self.market_store.get_index_by_id(id);
        match index {
            Some(idx) => {
                let start_date = fwd.start_date();
                let end_date = fwd.end_date();
                let compounding = fwd.compounding();
                let frequency = fwd.frequency();
                return idx.forward_rate(start_date, end_date, compounding, frequency);
            }
            None => panic!("No curve found for id {}", id),
        };
    }

    fn gen_fx_data(&self, fx: ExchangeRateRequest, _eval_date: Date) -> f64 {
        let first_currency = fx.first_currency();
        let second_currency = fx.second_currency();
        let fx = self
            .market_store
            .get_exchange_rate(first_currency, second_currency);
        match fx {
            Some(fx) => return fx,
            None => panic!(
                "No exchange rate found for {:?} and {:?}",
                first_currency, second_currency
            ),
        }
    }
}
