use super::structs::{CLF, CLP, EUR, JPY, USD, ZAR};
use super::traits::CurrencyDetails;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Currency {
    USD,
    EUR,
    JPY,
    ZAR,
    CLP,
    CLF,
}

impl Currency {
    pub fn code(&self) -> String {
        match self {
            Currency::USD => USD.code(),
            Currency::EUR => EUR.code(),
            Currency::JPY => JPY.code(),
            Currency::ZAR => ZAR.code(),
            Currency::CLP => CLP.code(),
            Currency::CLF => CLF.code(),
        }
    }
    pub fn name(&self) -> String {
        match self {
            Currency::USD => USD.name(),
            Currency::EUR => EUR.name(),
            Currency::JPY => JPY.name(),
            Currency::ZAR => ZAR.name(),
            Currency::CLP => CLP.name(),
            Currency::CLF => CLF.name(),
        }
    }
    pub fn symbol(&self) -> String {
        match self {
            Currency::USD => USD.symbol(),
            Currency::EUR => EUR.symbol(),
            Currency::JPY => JPY.symbol(),
            Currency::ZAR => ZAR.symbol(),
            Currency::CLP => CLP.symbol(),
            Currency::CLF => CLF.symbol(),
        }
    }
    pub fn precision(&self) -> u8 {
        match self {
            Currency::USD => USD.precision(),
            Currency::EUR => EUR.precision(),
            Currency::JPY => JPY.precision(),
            Currency::ZAR => ZAR.precision(),
            Currency::CLP => CLP.precision(),
            Currency::CLF => CLF.precision(),
        }
    }
    pub fn numeric_code(&self) -> u16 {
        match self {
            Currency::USD => USD.numeric_code(),
            Currency::EUR => EUR.numeric_code(),
            Currency::JPY => JPY.numeric_code(),
            Currency::ZAR => ZAR.numeric_code(),
            Currency::CLP => CLP.numeric_code(),
            Currency::CLF => CLF.numeric_code(),
        }
    }
}
