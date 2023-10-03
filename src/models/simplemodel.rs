use std::rc::Rc;

use crate::core::marketstore::MarketStore;

use crate::core::meta::*;
use crate::currencies::traits::CurrencyDetails;
use crate::rates::traits::{HasReferenceDate, YieldProvider, YieldProviderError};
use crate::time::date::Date;

use super::traits::{Model, ModelError};

/// # SimpleModel
/// A simple model that provides market data based in the current market state. Uses the
/// market store to get the market data. All values are calculated using the reference date
/// of the market store.
///
/// ## Parameters
/// * `market_store` - The market store.
#[derive(Clone)]
pub struct SimpleModel {
    market_store: Rc<MarketStore>,
}

impl SimpleModel {
    pub fn new(market_store: Rc<MarketStore>) -> SimpleModel {
        SimpleModel { market_store }
    }
}

impl Model for SimpleModel {
    fn reference_date(&self) -> Date {
        self.market_store.reference_date()
    }
    fn gen_df_data(&self, df: DiscountFactorRequest) -> Result<f64, ModelError> {
        let date = df.date();
        let ref_date = self.market_store.reference_date();

        // eval today or before ref date
        if ref_date > date {
            return Ok(0.0);
        } else if ref_date == date {
            return Ok(1.0);
        }

        let id = df.provider_id();
        let index = self
            .market_store
            .get_index_by_id(id)
            .ok_or(ModelError::NoCurveFound(id.to_string()))?;

        let curve = index
            .term_structure()
            .ok_or(YieldProviderError::NoTermStructure)?;
        Ok(curve.discount_factor(date)?)
    }

    fn gen_fwd_data(&self, fwd: ForwardRateRequest) -> Result<f64, ModelError> {
        let id = fwd.provider_id();
        let index = self
            .market_store
            .get_index_by_id(id)
            .ok_or(ModelError::NoCurveFound(id.to_string()))?;

        let start_date = fwd.start_date();
        let end_date = fwd.end_date();
        Ok(index.forward_rate(start_date, end_date, fwd.compounding(), fwd.frequency())?)
    }

    fn gen_fx_data(&self, fx: ExchangeRateRequest) -> Result<f64, ModelError> {
        let first_currency = fx.first_currency();
        let second_currency = match fx.second_currency() {
            Some(ccy) => ccy,
            None => self.market_store.local_currency(),
        };

        match fx.reference_date() {
            Some(date) => {
                let first_id = self
                    .market_store
                    .exchange_rate_store()
                    .get_currency_curve(first_currency)
                    .ok_or(ModelError::NoCurveFoundForCcy(first_currency.name()))?;

                let second_id = self
                    .market_store
                    .exchange_rate_store()
                    .get_currency_curve(second_currency)
                    .ok_or(ModelError::NoCurveFoundForCcy(second_currency.name()))?;

                let spot = self
                    .market_store
                    .exchange_rate_store()
                    .get_exchange_rate(first_currency, second_currency)
                    .ok_or(ModelError::NoFxRateFound(
                        first_currency.name(),
                        second_currency.name(),
                    ))?;

                let first_curve = self
                    .market_store
                    .get_index_by_id(first_id)
                    .ok_or(ModelError::NoCurveFound(first_id.to_string()))?;

                let second_curve = self
                    .market_store
                    .get_index_by_id(second_id)
                    .ok_or(ModelError::NoCurveFound(second_id.to_string()))?;

                let first_df = first_curve.discount_factor(date)?;
                let second_df = second_curve.discount_factor(date)?;

                Ok(spot * first_df / second_df)
            }
            None => Ok(self
                .market_store
                .exchange_rate_store()
                .get_exchange_rate(first_currency, second_currency)
                .ok_or(ModelError::NoFxRateFound(
                    first_currency.name(),
                    second_currency.name(),
                ))?),
        }
    }
}
