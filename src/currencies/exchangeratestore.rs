use std::{collections::{HashMap, HashSet, VecDeque}, borrow::BorrowMut, cell::RefCell};

use super::enums::Currency;

pub struct FxRecepy {
    currency: Currency,
    risk_free_curve_id: usize,
}

#[derive(Clone)]
pub struct ExchangeRateStore {
    fx_recepies: HashMap<Currency, usize>,
    exchange_rate_map: HashMap<(Currency, Currency), f64>,
    exchange_rate_cache: RefCell<HashMap<(Currency, Currency), f64>>,
}

impl ExchangeRateStore {
    pub fn new() -> ExchangeRateStore {
        ExchangeRateStore {
            fx_recepies: HashMap::new(),
            exchange_rate_map: HashMap::new(),
            exchange_rate_cache: RefCell::new(HashMap::new()),
        }
    }

    pub fn with_exchange_rates(
        &mut self,
        exchange_rate_map: HashMap<(Currency, Currency), f64>,
    ) -> &mut Self {
        self.exchange_rate_map = exchange_rate_map;
        return self;
    }

    pub fn add_fx_recepy(&mut self, fx_recepy: FxRecepy) {
        self.fx_recepies
            .insert(fx_recepy.currency, fx_recepy.risk_free_curve_id);
    }

    pub fn add_exchange_rate(&mut self, currency1: Currency, currency2: Currency, rate: f64) {
        self.exchange_rate_map.insert((currency1, currency2), rate);
    }

    pub fn get_exchange_rate(&self, first_ccy: Currency, second_ccy: Currency) -> Option<f64> {
        let first_ccy = first_ccy;
        let second_ccy = second_ccy;

        if first_ccy == second_ccy {
            return Some(1.0);
        }

        let cache_key = (first_ccy, second_ccy);
        if let Some(cached_rate) = self.exchange_rate_cache.borrow().get(&cache_key) {
            return Some(*cached_rate);
        }

        let mut q: VecDeque<(Currency, f64)> = VecDeque::new();
        let mut visited: HashSet<Currency> = HashSet::new();
        q.push_back((first_ccy, 1.0));
        visited.insert(first_ccy);

        let mut mutable_cache = self.exchange_rate_cache.borrow_mut();
        while let Some((current_ccy, rate)) = q.pop_front() {
            for (&(source, dest), &map_rate) in &self.exchange_rate_map {
                if source == current_ccy && !visited.contains(&dest) {
                    let new_rate = rate * map_rate;
                    if dest == second_ccy {
                        mutable_cache
                            .insert((first_ccy, second_ccy), new_rate);
                        mutable_cache
                            .insert((second_ccy, first_ccy), 1.0 / new_rate);
                        return Some(new_rate);
                    }
                    visited.insert(dest);
                    q.push_back((dest, new_rate));
                } else if dest == current_ccy && !visited.contains(&source) {
                    let new_rate = rate / map_rate;
                    if source == second_ccy {
                        mutable_cache
                            .insert((first_ccy, second_ccy), new_rate);
                        mutable_cache
                            .insert((second_ccy, first_ccy), 1.0 / new_rate);
                        return Some(new_rate);
                    }
                    visited.insert(source);
                    q.push_back((source, new_rate));
                }
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::currencies::enums::Currency::{CLP, EUR, USD};

    #[test]
    fn test_same_currency() {
        let manager = ExchangeRateStore {
            fx_recepies: HashMap::new(),
            exchange_rate_map: HashMap::new(),
            exchange_rate_cache: RefCell::new(HashMap::new()),
        };

        assert_eq!(manager.get_exchange_rate(USD, USD).unwrap(), 1.0);
    }

    #[test]
    fn test_cache() {
        let manager = ExchangeRateStore {
            fx_recepies: HashMap::new(),
            exchange_rate_map: {
                let mut map = HashMap::new();
                map.insert((USD, EUR), 0.85);
                map
            },
            exchange_rate_cache: RefCell::new(HashMap::new()),
        };

        assert_eq!(manager.get_exchange_rate(USD, EUR).unwrap(), 0.85);
        assert_eq!(manager.exchange_rate_cache.borrow().get(&(USD, EUR)).unwrap(), &0.85);
    }

    #[test]
    fn test_nonexistent_rate() {
        let manager = ExchangeRateStore {
            fx_recepies: HashMap::new(),
            exchange_rate_map: HashMap::new(),
            exchange_rate_cache: RefCell::new(HashMap::new()),
        };

        assert_eq!(manager.get_exchange_rate(USD, EUR), None);
    }

    #[test]
    fn test_complex_case() {
        let manager = ExchangeRateStore {
            fx_recepies: HashMap::new(),
            exchange_rate_map: {
                let mut map = HashMap::new();
                map.insert((USD, EUR), 0.85);
                map.insert((EUR, USD), 1.0 / 0.85);
                map
            },
            exchange_rate_cache: RefCell::new(HashMap::new()),
        };

        assert_eq!(manager.get_exchange_rate(EUR, USD).unwrap(), 1.0 / 0.85);
        assert_eq!(manager.get_exchange_rate(USD, EUR).unwrap(), 0.85);
    }

    #[test]
    fn test_triangulation_case() {
        let mut manager = ExchangeRateStore::new();
        manager.add_exchange_rate(CLP, USD, 800.0);
        manager.add_exchange_rate(USD, EUR, 1.1);

        assert_eq!(manager.get_exchange_rate(CLP, EUR).unwrap(), 1.1 * 800.0);
        assert_eq!(
            manager.get_exchange_rate(EUR, CLP).unwrap(),
            1.0 / (1.1 * 800.0)
        );
    }
}
