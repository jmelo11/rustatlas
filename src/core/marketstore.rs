use crate::{
    currencies::{enums::Currency, exchangeratestore::ExchangeRateStore},
    rates::{
        indexstore::IndexStore, interestrateindex::traits::InterestRateIndexTrait,
        traits::HasReferenceDate,
    },
    time::{date::Date, enums::TimeUnit, period::Period},
    utils::errors::{AtlasError, Result},
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
            index_store: IndexStore::new(reference_date),
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
    ) -> Result<f64> {
        let second_currency = match second_currency {
            Some(ccy) => ccy,
            None => self.local_currency,
        };
        return self
            .exchange_rate_store
            .get_exchange_rate(first_currency, second_currency);
    }

    pub fn get_index_by_id(&self, id: usize) -> Result<&Box<dyn InterestRateIndexTrait>> {
        return self.index_store.get_index_by_id(id);
    }

    pub fn advance_to_period(&self, period: Period) -> Result<MarketStore> {
        if period.length() < 1 {
            return Err(AtlasError::InvalidValueErr(format!(
                "Negative periods are not allowed: {:?}",
                period
            )));
        }
        let new_reference_date = self.reference_date + period;
        let new_index_store = self.index_store.advance_to_period(period)?;
        Ok(MarketStore {
            reference_date: new_reference_date,
            local_currency: self.local_currency,
            exchange_rate_store: self.exchange_rate_store.clone(),
            index_store: new_index_store,
        })
    }

    pub fn advance_to_date(&self, date: Date) -> Result<MarketStore> {
        if date < self.reference_date {
            return Err(AtlasError::InvalidValueErr(format!(
                "Date {:?} is before reference date {:?}",
                date, self.reference_date
            )));
        }
        let days = (date - self.reference_date) as i32;
        let period = Period::new(days, TimeUnit::Days);
        self.advance_to_period(period)
    }
}

impl HasReferenceDate for MarketStore {
    fn reference_date(&self) -> Date {
        self.reference_date
    }
}
