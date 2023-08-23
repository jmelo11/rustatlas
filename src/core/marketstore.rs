use std::rc::Rc;

use crate::{
    currencies::{enums::Currency, exchangeratestore::ExchangeRateStore},
    rates::traits::{HasReferenceDate, YieldProvider},
    rates::yieldproviderstore::YieldProviderStore,
    time::date::Date,
};

/// # MarketStore
/// A store for market data.
#[derive(Clone)]
pub struct MarketStore {
    reference_date: Date,
    local_currency: Currency,
    exchange_rate_manager: ExchangeRateStore,
    yield_provider_store: YieldProviderStore,
}

impl MarketStore {
    pub fn new(reference_date: Date, local_currency: Currency) -> MarketStore {
        MarketStore {
            reference_date,
            local_currency,
            exchange_rate_manager: ExchangeRateStore::new(),
            yield_provider_store: YieldProviderStore::new(),
        }
    }

    pub fn local_currency(&self) -> Currency {
        self.local_currency
    }

    pub fn exchange_rate_manager(&self) -> &ExchangeRateStore {
        &self.exchange_rate_manager
    }

    pub fn mut_exchange_rate_manager(&mut self) -> &mut ExchangeRateStore {
        &mut self.exchange_rate_manager
    }

    pub fn yield_provider_store(&self) -> &YieldProviderStore {
        &self.yield_provider_store
    }

    pub fn mut_yield_providers_store(&mut self) -> &mut YieldProviderStore {
        &mut self.yield_provider_store
    }

    pub fn get_exchange_rate(
        &mut self,
        first_currency: Currency,
        second_currency: Option<Currency>,
    ) -> Option<f64> {
        let second_currency = match second_currency {
            Some(ccy) => ccy,
            None => self.local_currency,
        };
        return self
            .exchange_rate_manager
            .get_exchange_rate(first_currency, second_currency);
    }

    pub fn get_provider_by_id(&self, id: usize) -> Option<&Rc<dyn YieldProvider>> {
        return self.yield_provider_store.get_provider_by_id(id);
    }
}

impl HasReferenceDate for MarketStore {
    fn reference_date(&self) -> Date {
        self.reference_date
    }
}
