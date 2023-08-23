use crate::core::marketstore::MarketStore;

use crate::core::meta::{
    MetaDiscountFactor, MetaExchangeRate, MetaForwardRate, MetaMarketDataNode,
};
use crate::rates::traits::YieldProvider;
use crate::time::date::Date;

use super::traits::Model;

/// # SimpleModel
/// A simple model that provides market data based in the current market state.
///
/// ## Parameters
/// * `market_store` - The market store.
/// * `meta_market_data` - The meta market data.
/// * `eval_dates` - The evaluation dates.
#[derive(Clone)]
pub struct SimpleModel {
    market_store: MarketStore,
    meta_market_data: Vec<MetaMarketDataNode>,
}

impl SimpleModel {
    pub fn new(
        market_store: MarketStore,
        meta_market_data: Vec<MetaMarketDataNode>,
    ) -> SimpleModel {
        SimpleModel {
            market_store,
            meta_market_data,
        }
    }
}

impl Model for SimpleModel {
    fn gen_df_data(&self, df: MetaDiscountFactor, eval_date: Date) -> f64 {
        let date = df.reference_date();
        if eval_date > date {
            return 0.0;
        } else if eval_date == date {
            return 1.0;
        }
        let id: usize = df.provider_id();
        let provider = self.market_store.get_provider_by_id(id);
        let df = match provider {
            Some(curve) => {
                return curve.discount_factor(date);
            }
            None => panic!("No curve found for id {}", id),
        };
    }

    fn gen_fwd_data(&self, fwd: MetaForwardRate, eval_date: Date) -> f64 {
        let id = fwd.provider_id();
        let provider = self.market_store.get_provider_by_id(id);
        let fwd = match provider {
            Some(curve) => {
                return curve.forward_rate(
                    fwd.start_date(),
                    fwd.end_date(),
                    fwd.compounding(),
                    fwd.frequency(),
                );
            }
            None => panic!("No curve found for id {}", id),
        };
    }

    fn gen_fx_data(&mut self, fx: MetaExchangeRate, eval_date: Date) -> f64 {
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

mod tests {
    use std::rc::Rc;

    use crate::prelude::*;

    #[test]
    fn test_market_data_generation() {
        let reference_date = Date::from_ymd(2021, 1, 1);
        let local_currency = Currency::USD;
        let mut market_store = MarketStore::new(reference_date, local_currency);
        let rate = InterestRate::new(
            0.05,
            Compounding::Simple,
            Frequency::Annual,
            DayCounter::Actual360,
        );

        let term_structure = YieldTermStructure::FlatForwardTermStructure(
            FlatForwardTermStructure::new(reference_date, rate),
        );

        let interest_rate_index = InterestRateIndex::IborIndex(
            IborIndex::new(Period::new(6, TimeUnit::Months)).with_term_structure(term_structure),
        );

        market_store
            .mut_yield_providers_store()
            .add_provider("Example".to_string(), Rc::new(interest_rate_index));

        let request_date = Date::from_ymd(2025, 1, 1);
        let df = MetaDiscountFactor::new(0, request_date);
        let meta_data = vec![MetaMarketDataNode::new(0, Some(df), None, None)];

        let eval_dates = vec![Date::from_ymd(2021, 1, 1), Date::from_ymd(2022, 6, 1)];
        let model = SimpleModel::new(market_store, meta_data);
    }
}
