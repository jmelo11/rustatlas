use crate::{
    currencies::{enums::Currency, exchangeratemanager::ExchangeRateManager},
    rates::yieldtermstructuremanager::YieldTermStructureManager,
};

pub struct MarketStore {
    curve_manager: YieldTermStructureManager,
    exchange_rate_manager: ExchangeRateManager,
    local_currency: Currency,
}

impl MarketStore {
    pub fn new(local_currency: Currency) -> MarketStore {
        MarketStore {
            curve_manager: YieldTermStructureManager::new(),
            exchange_rate_manager: ExchangeRateManager::new(),
            local_currency,
        }
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
