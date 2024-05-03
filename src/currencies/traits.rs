use crate::{rates::indexstore::IndexStore, 
            time::{date::Date, period::Period},
            utils::errors::Result};

use super::exchangeratestore::ExchangeRateStore;

/// # CurrencyDetails
/// Trait for currency details
pub trait CurrencyDetails {
    fn code(&self) -> String;
    fn name(&self) -> String;
    fn symbol(&self) -> String;
    fn precision(&self) -> u8;
    fn numeric_code(&self) -> u16;
}

/// # AdvanceExchangeRateStoreInTime
/// Trait for advancing an exchange rate store in time using de index store
/// It is necessary for any currency, have free risk curve tabulated in the index store
/// If the currency does not have a free risk curve, method advance_to_period and advance_to_date will mantain the same fx 
pub trait AdvanceExchangeRateStoreInTime {
    fn advance_to_period(&self, period: Period, index_store: &IndexStore) -> Result<ExchangeRateStore>;
    fn advance_to_date(&self, date: Date, index_store: &IndexStore) -> Result<ExchangeRateStore>;
}
