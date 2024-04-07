use crate::{
    core::{
        marketstore::MarketStore,
        meta::{DiscountFactorRequest, ExchangeRateRequest, ForwardRateRequest},
    },
    rates::{indexstore::ReadIndex, traits::HasReferenceDate},
    time::date::Date,
    utils::errors::Result,
};

use super::traits::Model;

/// # SimpleModel
/// A simple model that provides market data based on the current market state. Uses the
/// market store to get the market data (curves, currencies and others). All values are calculated using the
/// reference date and local currency of the market store.
///
/// ## Parameters
/// * `market_store` - The market store.
/// * `transform_currencies` - If true, the model will transform the currencies to the local currency of the market store.
#[derive(Clone)]
pub struct SimpleModel<'a> {
    market_store: &'a MarketStore,
    transform_currencies: bool,
}

impl<'a> SimpleModel<'a> {
    pub fn new(market_store: &'a MarketStore) -> SimpleModel {
        SimpleModel {
            market_store,
            transform_currencies: false,
        }
    }

    pub fn with_transform_currencies(mut self, flag: bool) -> SimpleModel<'a> {
        self.transform_currencies = flag;
        self
    }
}

impl<'a> Model for SimpleModel<'a> {
    fn reference_date(&self) -> Date {
        self.market_store.reference_date()
    }

    fn gen_df_data(&self, df: DiscountFactorRequest) -> Result<f64> {
        let date = df.date();
        let ref_date = self.market_store.reference_date();

        // eval today or before ref date
        if ref_date > date {
            return Ok(0.0);
        } else if ref_date == date {
            return Ok(1.0);
        }

        let id = df.provider_id();
        let index = self.market_store.get_index(id)?;
        let curve = index.read_index()?.term_structure()?;
        Ok(curve.discount_factor(date)?)
    }

    fn gen_fwd_data(&self, fwd: ForwardRateRequest) -> Result<f64> {
        let id = fwd.provider_id();
        let end_date = fwd.end_date();
        let ref_date = self.market_store.reference_date();
        if end_date <= ref_date {
            return Ok(0.0);
        }

        let index = self.market_store.get_index(id)?;
        let fwd_rate_provider = index.read_index()?;
        let start_date = fwd.start_date();
        Ok(fwd_rate_provider.forward_rate(
            start_date,
            end_date,
            fwd.compounding(),
            fwd.frequency(),
        )?)
    }

    fn gen_fx_data(&self, fx: ExchangeRateRequest) -> Result<f64> {
        let first_currency = fx.first_currency();
        let second_currency = match fx.second_currency() {
            Some(ccy) => ccy,
            None => {
                if self.transform_currencies {
                    self.market_store.local_currency()
                } else {
                    first_currency
                }
            }
        };

        match fx.reference_date() {
            Some(date) => {
                let first_id = self
                    .market_store
                    .exchange_rate_store()
                    .get_currency_curve(first_currency)?;

                let second_id = self
                    .market_store
                    .exchange_rate_store()
                    .get_currency_curve(second_currency)?;

                let spot = self
                    .market_store
                    .exchange_rate_store()
                    .get_exchange_rate(first_currency, second_currency)?;

                let first_curve = self.market_store.get_index(first_id)?;
                let second_curve = self.market_store.get_index(second_id)?;

                let first_df = first_curve.read_index()?.discount_factor(date)?;
                let second_df = second_curve.read_index()?.discount_factor(date)?;

                Ok(spot * first_df / second_df)
            }
            None => Ok(self
                .market_store
                .exchange_rate_store()
                .get_exchange_rate(first_currency, second_currency)?),
        }
    }
}
