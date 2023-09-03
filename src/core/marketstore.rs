use crate::{
    currencies::{enums::Currency, exchangeratestore::ExchangeRateStore},
    rates::{
        indexstore::IndexStore, interestrateindex::enums::InterestRateIndex,
        traits::HasReferenceDate,
    },
    time::date::Date,
};

/// # MarketStore
/// A store for market data.
#[derive(Clone)]
pub struct MarketStore {
    reference_date: Date,
    local_currency: Currency,
    exchange_rate_store: ExchangeRateStore,
    index_store: IndexStore,
}

impl MarketStore {
    pub fn new(reference_date: Date, local_currency: Currency) -> MarketStore {
        MarketStore {
            reference_date,
            local_currency,
            exchange_rate_store: ExchangeRateStore::new(),
            index_store: IndexStore::new(),
        }
    }

    pub fn local_currency(&self) -> Currency {
        self.local_currency
    }

    pub fn exchange_rate_store(&self) -> &ExchangeRateStore {
        &self.exchange_rate_store
    }

    pub fn mut_exchange_rate_store(&mut self) -> &mut ExchangeRateStore {
        &mut self.exchange_rate_store
    }

    pub fn index_store(&self) -> &IndexStore {
        &self.index_store
    }

    pub fn mut_index_store(&mut self) -> &mut IndexStore {
        &mut self.index_store
    }

    pub fn get_exchange_rate(
        &self,
        first_currency: Currency,
        second_currency: Option<Currency>,
    ) -> Option<f64> {
        let second_currency = match second_currency {
            Some(ccy) => ccy,
            None => self.local_currency,
        };
        return self
            .exchange_rate_store
            .get_exchange_rate(first_currency, second_currency);
    }

    pub fn get_index_by_id(&self, id: usize) -> Option<&InterestRateIndex> {
        return self.index_store.get_index_by_id(id);
    }
}

impl HasReferenceDate for MarketStore {
    fn reference_date(&self) -> Date {
        self.reference_date
    }
}
