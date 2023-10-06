use serde::{Deserialize, Serialize};

use super::structs::{CHF, CLF, CLP, EUR, JPY, USD, ZAR};
use super::traits::CurrencyDetails;

/// # Currency
/// Enum for currencies supported by the library
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Currency {
    USD,
    EUR,
    JPY,
    ZAR,
    CLP,
    CLF,
    CHF,
}

impl CurrencyDetails for Currency {
    fn code(&self) -> String {
        match self {
            Currency::USD => USD.code(),
            Currency::EUR => EUR.code(),
            Currency::JPY => JPY.code(),
            Currency::ZAR => ZAR.code(),
            Currency::CLP => CLP.code(),
            Currency::CLF => CLF.code(),
            Currency::CHF => CHF.code(),
        }
    }
    fn name(&self) -> String {
        match self {
            Currency::USD => USD.name(),
            Currency::EUR => EUR.name(),
            Currency::JPY => JPY.name(),
            Currency::ZAR => ZAR.name(),
            Currency::CLP => CLP.name(),
            Currency::CLF => CLF.name(),
            Currency::CHF => CHF.name(),
        }
    }
    fn symbol(&self) -> String {
        match self {
            Currency::USD => USD.symbol(),
            Currency::EUR => EUR.symbol(),
            Currency::JPY => JPY.symbol(),
            Currency::ZAR => ZAR.symbol(),
            Currency::CLP => CLP.symbol(),
            Currency::CLF => CLF.symbol(),
            Currency::CHF => CHF.symbol(),
        }
    }
    fn precision(&self) -> u8 {
        match self {
            Currency::USD => USD.precision(),
            Currency::EUR => EUR.precision(),
            Currency::JPY => JPY.precision(),
            Currency::ZAR => ZAR.precision(),
            Currency::CLP => CLP.precision(),
            Currency::CLF => CLF.precision(),
            Currency::CHF => CHF.precision(),
        }
    }
    fn numeric_code(&self) -> u16 {
        match self {
            Currency::USD => USD.numeric_code(),
            Currency::EUR => EUR.numeric_code(),
            Currency::JPY => JPY.numeric_code(),
            Currency::ZAR => ZAR.numeric_code(),
            Currency::CLP => CLP.numeric_code(),
            Currency::CLF => CLF.numeric_code(),
            Currency::CHF => CHF.numeric_code(),
        }
    }
}
