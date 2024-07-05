use core::fmt;
use std::sync::{Arc, RwLock};

use crate::{
    currencies::{enums::Currency, exchangeratestore::ExchangeRateStore, traits::{AdvanceExchangeRateStoreInTime, CurrencyDetails}}, rates::{
        indexstore::{IndexStore, ReadIndex}, interestrateindex::traits::InterestRateIndexTrait,
        traits::HasReferenceDate,
    }, time::{date::Date, enums::TimeUnit, period::Period}, utils::{errors::{AtlasError, Result}, tools}
};

/// # MarketStore
/// A store for market data.
///
/// ## Parameters
/// * `reference_date` - The reference date of the market store
/// * `local_currency` - The local currency of the market store
/// * `exchange_rate_store` - The exchange rate store
/// * `index_store` - The index store
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
            exchange_rate_store: ExchangeRateStore::new(reference_date),
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

    pub fn get_index(&self, id: usize) -> Result<Arc<RwLock<dyn InterestRateIndexTrait>>> {
        return self.index_store.get_index(id);
    }

    pub fn advance_to_period(&self, period: Period) -> Result<MarketStore> {
        if period.length() < 0 {
            return Err(AtlasError::InvalidValueErr(format!(
                "Negative periods are not allowed when advancing market store in time ({:?})",
                period
            )));
        }
        let new_reference_date = self.reference_date + period;
        let new_exchange_rate_store = self.exchange_rate_store.advance_to_period(period, &self.index_store)?;
        let new_index_store = self.index_store.advance_to_period(period)?;
        
        Ok(MarketStore {
            reference_date: new_reference_date,
            local_currency: self.local_currency,
            exchange_rate_store: new_exchange_rate_store,
            index_store: new_index_store,
        })
    }

    pub fn advance_to_date(&self, date: Date) -> Result<MarketStore> {
        if date < self.reference_date {
            return Err(AtlasError::InvalidValueErr(format!(
                "Date {} is before reference date {}",
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

// fecha, moneda, qué curvas tiene cargadas, paridades
impl fmt::Display for MarketStore {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

        let mut msg = "=====================================\n".to_string();
        msg.push_str("======= MarketStore features! =======\n");
        msg.push_str("=====================================\n");

        msg.push_str("> Reference Date: ");
        msg.push_str(&self.reference_date.to_string());
        msg.push_str("\n");
        msg.push_str("-------------------------------------\n");
        msg.push_str("> Currency: ");
        msg.push_str(&self.local_currency.code().to_string());
        msg.push_str("\n");
        msg.push_str("-------------------------------------\n");

        let index_store = self.index_store();
        let all_indices = index_store.get_all_indices();

        let mut indices_names: Vec<String> = vec![];
        let mut indice_name: String;

        for indice in all_indices {
            indice_name = match indice.read_index() {
                Ok(indice) => indice.name().unwrap().to_string(),
                Err(_) => "".to_string(),
            };
            if indice_name != "".to_string() {
                indices_names.push(indice_name);
            }   
        }

        let indices_map = index_store.get_index_map().unwrap();

        indices_names = tools::sort_strings_alphabetically(&indices_names);

        msg.push_str("> Indices (");
        msg.push_str(&indices_names.len().to_string());
        msg.push_str("):\n");
        for indice_name in indices_names {

            let indice_idx = match indices_map.get(&indice_name) {
                Some(idx) => idx.to_string(),
                None => "".to_string(),
            };

            msg.push_str(">> ");
            msg.push_str(&indice_idx);
            msg.push_str(" -> ");
            msg.push_str(&indice_name);
            msg.push_str("\n");
        }

        let exchange_rate_store = self.exchange_rate_store();
        let exchange_rate_map = exchange_rate_store.get_exchange_rate_map();
        msg.push_str("-------------------------------------\n");
        msg.push_str("> Currency pairs (");
        msg.push_str(&exchange_rate_map.len().to_string());
        msg.push_str("):\n");
        for (currencies, value) in &exchange_rate_map {
            // println!("{} - {}: {}", currencies.0.code(), currencies.1.code(), value);
            msg.push_str(">> ");
            msg.push_str(&currencies.0.code());
            msg.push_str(" -> ");
            msg.push_str(&currencies.1.code());
            msg.push_str(": ");
            msg.push_str(&value.to_string());
            msg.push_str("\n");
        }

        msg.push_str("=====================================\n");

        write!(f, "{}", msg)
    }
}