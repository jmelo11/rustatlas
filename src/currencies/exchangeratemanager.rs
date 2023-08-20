use std::collections::{HashMap, HashSet, VecDeque};

use super::enums::Currency;

pub struct FxRecepy {
    currency: Currency,
    risk_free_curve_id: usize,
}

pub struct ExchangeRateManager {
    fx_recepies: HashMap<Currency, usize>,
    exchange_rate_map: HashMap<(Currency, Currency), f64>,
    exchange_rate_cache: HashMap<(Currency, Currency), f64>,
}

impl ExchangeRateManager {
    pub fn new() -> ExchangeRateManager {
        ExchangeRateManager {
            fx_recepies: HashMap::new(),
            exchange_rate_map: HashMap::new(),
            exchange_rate_cache: HashMap::new(),
        }
    }

    pub fn add_fx_recepy(&mut self, fx_recepy: FxRecepy) {
        self.fx_recepies
            .insert(fx_recepy.currency, fx_recepy.risk_free_curve_id);
    }

    pub fn add_exchange_rate(&mut self, currency1: Currency, currency2: Currency, rate: f64) {
        self.exchange_rate_map.insert((currency1, currency2), rate);
    }

    pub fn get_exchange_rate(
        &mut self,
        first_ccy: Currency,
        second_ccy: Currency,
    ) -> Result<f64, String> {
        let mut first_ccy = first_ccy;
        let mut second_ccy = second_ccy;

        if first_ccy == second_ccy {
            return Ok(1.0);
        }

        let cache_key = (first_ccy, second_ccy);
        if let Some(cached_rate) = self.exchange_rate_cache.get(&cache_key) {
            return Ok(*cached_rate);
        }

        let mut q: VecDeque<(Currency, f64)> = VecDeque::new();
        let mut visited: HashSet<Currency> = HashSet::new();
        q.push_back((first_ccy, 1.0));
        visited.insert(first_ccy);

        while let Some((current_ccy, rate)) = q.pop_front() {
            for (&(source, dest), &map_rate) in &self.exchange_rate_map {
                if source == current_ccy && !visited.contains(&dest) {
                    let new_rate = rate * map_rate;
                    if dest == second_ccy {
                        self.exchange_rate_cache
                            .insert((first_ccy, second_ccy), new_rate);
                        self.exchange_rate_cache
                            .insert((second_ccy, first_ccy), 1.0 / new_rate);
                        return Ok(new_rate);
                    }
                    visited.insert(dest);
                    q.push_back((dest, new_rate));
                } else if dest == current_ccy && !visited.contains(&source) {
                    let new_rate = rate / map_rate;
                    if source == second_ccy {
                        self.exchange_rate_cache
                            .insert((first_ccy, second_ccy), new_rate);
                        self.exchange_rate_cache
                            .insert((second_ccy, first_ccy), 1.0 / new_rate);
                        return Ok(new_rate);
                    }
                    visited.insert(source);
                    q.push_back((source, new_rate));
                }
            }
        }

        Err("Exchange rate not found for the given currencies".into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::currencies::enums::Currency::{EUR, USD};
        
    #[test]
    fn test_same_currency() {
        let mut manager = ExchangeRateManager {
            fx_recepies: HashMap::new(),
            exchange_rate_map: HashMap::new(),
            exchange_rate_cache: HashMap::new(),
        };

        assert_eq!(manager.get_exchange_rate(USD, USD).unwrap(), 1.0);
    }

    #[test]
    fn test_cache() {
        let mut manager = ExchangeRateManager {
            fx_recepies: HashMap::new(),
            exchange_rate_map: {
                let mut map = HashMap::new();
                map.insert((USD, EUR), 0.85);
                map
            },
            exchange_rate_cache: HashMap::new(),
        };

        assert_eq!(manager.get_exchange_rate(USD, EUR).unwrap(), 0.85);
        assert_eq!(manager.exchange_rate_cache.get(&(USD, EUR)).unwrap(), &0.85);
    }

    #[test]
    fn test_nonexistent_rate() {
        let mut manager = ExchangeRateManager {
            fx_recepies: HashMap::new(),
            exchange_rate_map: HashMap::new(),
            exchange_rate_cache: HashMap::new(),
        };

        assert_eq!(
            manager.get_exchange_rate(USD, EUR),
            Err("Exchange rate not found for the given currencies".into())
        );
    }

    #[test]
    fn test_complex_case() {
        let mut manager = ExchangeRateManager {
            fx_recepies: HashMap::new(),
            exchange_rate_map: {
                let mut map = HashMap::new();
                map.insert((USD, EUR), 0.85);
                map.insert((EUR, USD), 1.0 / 0.85);
                map
            },
            exchange_rate_cache: HashMap::new(),
        };

        assert_eq!(manager.get_exchange_rate(EUR, USD).unwrap(), 1.0 / 0.85);
        assert_eq!(manager.get_exchange_rate(USD, EUR).unwrap(), 0.85);
    }
}
