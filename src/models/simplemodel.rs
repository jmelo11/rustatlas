use crate::core::marketstore::MarketStore;

use crate::core::meta::*;
use crate::rates::traits::{HasReferenceDate, YieldProvider};

use super::traits::Model;

/// # SimpleModel
/// A simple model that provides market data based in the current market state. Uses the
/// market store to get the market data. All values are calculated using the reference date
/// of the market store.
///
/// ## Parameters
/// * `market_store` - The market store.
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
    fn gen_df_data(&self, df: DiscountFactorRequest) -> f64 {
        let date = df.date();
        let ref_date = self.market_store.reference_date();

        // eval today or before ref date
        if ref_date > date {
            return 0.0;
        } else if ref_date == date {
            return 1.0;
        }

        let id = df.provider_id();
        let index = self
            .market_store
            .get_index_by_id(id)
            .expect(format!("No curve found for id {}", id).as_str());

        let curve = index.term_structure().expect("No term structure found");
        curve.discount_factor(date)
    }

    fn gen_fwd_data(&self, fwd: ForwardRateRequest) -> f64 {
        let id = fwd.provider_id();
        let index = self
            .market_store
            .get_index_by_id(id)
            .expect(format!("No curve found for id {}", id).as_str());

        let start_date = fwd.start_date();
        let end_date = fwd.end_date();
        index.forward_rate(start_date, end_date, fwd.compounding(), fwd.frequency())
    }

    fn gen_fx_data(&self, fx: ExchangeRateRequest) -> f64 {
        let first_currency = fx.first_currency();
        let second_currency = fx.second_currency();
        let fx = self
            .market_store
            .get_exchange_rate(first_currency, second_currency);
        fx.expect("No exchange rate found")
    }
}
