use crate::{
    currencies::{enums::Currency, exchangeratemanager::ExchangeRateManager},
    rates::yieldtermstructuremanager::YieldTermStructureManager,
    time::date::Date,
};

/// # MarketStore
/// A store for market data.
#[derive(Clone)]
pub struct MarketStore {
    reference_date: Date,
    local_currency: Currency,
    curve_manager: YieldTermStructureManager,
    exchange_rate_manager: ExchangeRateManager,
}

impl MarketStore {
    pub fn new(reference_date: Date, local_currency: Currency) -> MarketStore {
        MarketStore {
            reference_date,
            local_currency,
            curve_manager: YieldTermStructureManager::new(),
            exchange_rate_manager: ExchangeRateManager::new(),
        }
    }

    pub fn reference_date(&self) -> Date {
        self.reference_date
    }

    pub fn local_currency(&self) -> Currency {
        self.local_currency
    }

    pub fn curve_manager(&mut self) -> &mut YieldTermStructureManager {
        &mut self.curve_manager
    }

    pub fn exchange_rate_manager(&mut self) -> &mut ExchangeRateManager {
        &mut self.exchange_rate_manager
    }
}
