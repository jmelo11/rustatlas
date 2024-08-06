use std::{
    collections::{HashMap, HashSet, VecDeque},
    sync::{Arc, Mutex},
};

use super::{enums::Currency, traits::AdvanceExchangeRateStoreInTime};

use crate::{rates::indexstore::IndexStore, time::{date::Date, period::Period}, utils::errors::{AtlasError, Result}};

/// # ExchangeRateStore
/// A store for exchange rates.
/// Exchange rates are stored as a map of pairs of currencies to rates.
///
/// ## Details
/// - Exchange rates are stored as a map of pairs of currencies to rates.
/// - The exchange rate between two currencies is calculated by traversing the graph of exchange rates.
#[derive(Clone)]
pub struct ExchangeRateStore {
    reference_date: Date,
    exchange_rate_map: HashMap<(Currency, Currency), f64>,
    exchange_rate_cache: Arc<Mutex<HashMap<(Currency, Currency), f64>>>,
}

impl ExchangeRateStore {
    pub fn new(date : Date) -> ExchangeRateStore {
        ExchangeRateStore {
            reference_date: date,
            exchange_rate_map: HashMap::new(),
            exchange_rate_cache: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn with_exchange_rates(
        &mut self,
        exchange_rate_map: HashMap<(Currency, Currency), f64>,
    ) -> &mut Self {
        self.exchange_rate_map = exchange_rate_map;
        self
    }

    pub fn add_exchange_rate(&mut self, currency1: Currency, currency2: Currency, rate: f64) {
        self.exchange_rate_map.insert((currency1, currency2), rate);
    }

    pub fn reference_date(&self) -> Date {
        self.reference_date
    }

    pub fn get_exchange_rate_map(&self) -> HashMap<(Currency, Currency), f64> {
        self.exchange_rate_map.clone()
    }

    pub fn get_exchange_rate(&self, first_ccy: Currency, second_ccy: Currency) -> Result<f64> {
        let first_ccy = first_ccy;
        let second_ccy = second_ccy;

        if first_ccy == second_ccy {
            return Ok(1.0);
        }

        let cache_key = (first_ccy, second_ccy);
        if let Some(cached_rate) = self.exchange_rate_cache.lock().unwrap().get(&cache_key) {
            return Ok(*cached_rate);
        }

        let mut q: VecDeque<(Currency, f64)> = VecDeque::new();
        let mut visited: HashSet<Currency> = HashSet::new();
        q.push_back((first_ccy, 1.0));
        visited.insert(first_ccy);

        let mut mutable_cache = self.exchange_rate_cache.lock().unwrap();
        while let Some((current_ccy, rate)) = q.pop_front() {
            for (&(source, dest), &map_rate) in &self.exchange_rate_map {
                if source == current_ccy && !visited.contains(&dest) {
                    let new_rate = rate * map_rate;
                    if dest == second_ccy {
                        mutable_cache.insert((first_ccy, second_ccy), new_rate);
                        mutable_cache.insert((second_ccy, first_ccy), 1.0 / new_rate);
                        return Ok(new_rate);
                    }
                    visited.insert(dest);
                    q.push_back((dest, new_rate));
                } else if dest == current_ccy && !visited.contains(&source) {
                    let new_rate = rate / map_rate;
                    if source == second_ccy {
                        mutable_cache.insert((first_ccy, second_ccy), new_rate);
                        mutable_cache.insert((second_ccy, first_ccy), 1.0 / new_rate);
                        return Ok(new_rate);
                    }
                    visited.insert(source);
                    q.push_back((source, new_rate));
                }
            }
        }
        Err(AtlasError::NotFoundErr(format!(
            "No exchange rate found between {:?} and {:?}",
            first_ccy, second_ccy
        )))
    }

}


impl AdvanceExchangeRateStoreInTime for ExchangeRateStore {
    fn advance_to_period(&self, period: Period, index_store: &IndexStore) -> Result<ExchangeRateStore> { 
        let new_date = self.reference_date + period;
        self.advance_to_date(new_date, index_store)
    }

    fn advance_to_date(&self, date: Date, index_store: &IndexStore) -> Result<ExchangeRateStore> {
        if self.reference_date() != index_store.reference_date() {
            return Err(AtlasError::InvalidValueErr(format!(
                "Reference date of exchange rate store and index store do not match"
            )));
        }

        let mut new_store = ExchangeRateStore::new(date);
        for ((ccy1, ccy2), fx) in self.exchange_rate_map.iter() {
            let compound_factor = index_store.currency_forescast_factor(*ccy1, *ccy2, date);
            match compound_factor {
                Ok(cf) => new_store.add_exchange_rate(*ccy1, *ccy2, fx * cf),
                Err(_) => {
                    // If the compound factor is not available, we use the last fx rate
                    new_store.add_exchange_rate(*ccy1, *ccy2, *fx);
                }
            }    
        }
        Ok(new_store)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::currencies::enums::Currency::{CLP, EUR, USD};

    #[test]
    fn test_same_currency() {
        let ref_date = Date::new(2021, 1, 1);
        let manager = ExchangeRateStore::new(ref_date);
        assert_eq!(manager.get_exchange_rate(USD, USD).unwrap(), 1.0);
    }

    #[test]
    fn test_cache() {
        let ref_date = Date::new(2021, 1, 1);
        let manager = ExchangeRateStore {
            reference_date: ref_date,
            exchange_rate_map: {
                let mut map = HashMap::new();
                map.insert((USD, EUR), 0.85);
                map
            },
            exchange_rate_cache: Arc::new(Mutex::new(HashMap::new())),
        };

        assert_eq!(manager.get_exchange_rate(USD, EUR).unwrap(), 0.85);
        assert_eq!(
            manager
                .exchange_rate_cache
                .lock()
                .unwrap()
                .get(&(USD, EUR))
                .unwrap(),
            &0.85
        );
    }

    #[test]
    fn test_nonexistent_rate() {
        let ref_date = Date::new(2021, 1, 1);
        let manager = ExchangeRateStore {
            reference_date: ref_date,
            exchange_rate_map: HashMap::new(),
            exchange_rate_cache: Arc::new(Mutex::new(HashMap::new())),
        };

        let result = manager.get_exchange_rate(USD, EUR);
        assert!(result.is_err());
    }

    #[test]
    fn test_complex_case() {
        let ref_date = Date::new(2021, 1, 1);
        let manager = ExchangeRateStore {
            reference_date: ref_date,
            exchange_rate_map: {
                let mut map = HashMap::new();
                map.insert((USD, EUR), 0.85);
                map.insert((EUR, USD), 1.0 / 0.85);
                map
            },
            exchange_rate_cache: Arc::new(Mutex::new(HashMap::new())),
        };

        assert_eq!(manager.get_exchange_rate(EUR, USD).unwrap(), 1.0 / 0.85);
        assert_eq!(manager.get_exchange_rate(USD, EUR).unwrap(), 0.85);
    }

    #[test]
    fn test_triangulation_case() {
        let ref_date = Date::new(2021, 1, 1);
        let mut manager = ExchangeRateStore::new(ref_date);
        manager.add_exchange_rate(CLP, USD, 800.0);
        manager.add_exchange_rate(USD, EUR, 1.1);

        assert_eq!(manager.get_exchange_rate(CLP, EUR).unwrap(), 1.1 * 800.0);
        assert_eq!(
            manager.get_exchange_rate(EUR, CLP).unwrap(),
            1.0 / (1.1 * 800.0)
        );
    }
}
