use serde::{Deserialize, Serialize};

use crate::utils::errors::{AtlasError, Result};

use super::{
    structs::{
        AUD, BRL, CAD, CHF, CLF, CLP, CNH, CNY, COP, DKK, EUR, GBP, HKD, IDR, INR, JPY, KRW, MXN,
        NOK, NZD, PEN, SEK, TWD, USD, ZAR,
    },
    traits::CurrencyDetails,
};

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
    BRL,
    COP,
    MXN,
    AUD,
    CAD,
    CNY,
    GBP,
    NZD,
    NOK,
    SEK,
    PEN,
    CNH,
    INR,
    TWD,
    HKD,
    KRW,
    DKK,
    IDR,
}

impl TryFrom<String> for Currency {
    type Error = AtlasError;

    fn try_from(s: String) -> Result<Self> {
        match s.as_str() {
            "USD" => Ok(Currency::USD),
            "EUR" => Ok(Currency::EUR),
            "JPY" => Ok(Currency::JPY),
            "ZAR" => Ok(Currency::ZAR),
            "CLP" => Ok(Currency::CLP),
            "CLF" => Ok(Currency::CLF),
            "CHF" => Ok(Currency::CHF),
            "BRL" => Ok(Currency::BRL),
            "COP" => Ok(Currency::COP),
            "MXN" => Ok(Currency::MXN),
            "AUD" => Ok(Currency::AUD),
            "CAD" => Ok(Currency::CAD),
            "CNY" => Ok(Currency::CNY),
            "GBP" => Ok(Currency::GBP),
            "NZD" => Ok(Currency::NZD),
            "NOK" => Ok(Currency::NOK),
            "SEK" => Ok(Currency::SEK),
            "PEN" => Ok(Currency::PEN),
            "CNH" => Ok(Currency::CNH),
            "INR" => Ok(Currency::INR),
            "TWD" => Ok(Currency::TWD),
            "HKD" => Ok(Currency::HKD),
            "KRW" => Ok(Currency::KRW),
            "DKK" => Ok(Currency::DKK),
            "IDR" => Ok(Currency::IDR),
            _ => Err(AtlasError::InvalidValueErr(format!(
                "Invalid currency: {}",
                s
            ))),
        }
    }
}

impl From<Currency> for String {
    fn from(currency: Currency) -> Self {
        match currency {
            Currency::USD => "USD".to_string(),
            Currency::EUR => "EUR".to_string(),
            Currency::JPY => "JPY".to_string(),
            Currency::ZAR => "ZAR".to_string(),
            Currency::CLP => "CLP".to_string(),
            Currency::CLF => "CLF".to_string(),
            Currency::CHF => "CHF".to_string(),
            Currency::BRL => "BRL".to_string(),
            Currency::COP => "COP".to_string(),
            Currency::MXN => "MXN".to_string(),
            Currency::AUD => "AUD".to_string(),
            Currency::CAD => "CAD".to_string(),
            Currency::CNY => "CNY".to_string(),
            Currency::GBP => "GBP".to_string(),
            Currency::NZD => "NZD".to_string(),
            Currency::NOK => "NOK".to_string(),
            Currency::SEK => "SEK".to_string(),
            Currency::PEN => "PEN".to_string(),
            Currency::CNH => "CNH".to_string(),
            Currency::INR => "INR".to_string(),
            Currency::TWD => "TWD".to_string(),
            Currency::HKD => "HKD".to_string(),
            Currency::KRW => "KRW".to_string(),
            Currency::DKK => "DKK".to_string(),
            Currency::IDR => "IDR".to_string(),
        }
    }
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
            Currency::BRL => BRL.code(),
            Currency::COP => COP.code(),
            Currency::MXN => MXN.code(),
            Currency::AUD => AUD.code(),
            Currency::CAD => CAD.code(),
            Currency::CNY => CNY.code(),
            Currency::GBP => GBP.code(),
            Currency::NZD => NZD.code(),
            Currency::NOK => NOK.code(),
            Currency::SEK => SEK.code(),
            Currency::PEN => PEN.code(),
            Currency::CNH => CNH.code(),
            Currency::INR => INR.code(),
            Currency::TWD => TWD.code(),
            Currency::HKD => HKD.code(),
            Currency::KRW => KRW.code(),
            Currency::DKK => DKK.code(),
            Currency::IDR => IDR.code(),
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
            Currency::BRL => BRL.name(),
            Currency::COP => COP.name(),
            Currency::MXN => MXN.name(),
            Currency::AUD => AUD.name(),
            Currency::CAD => CAD.name(),
            Currency::CNY => CNY.name(),
            Currency::GBP => GBP.name(),
            Currency::NZD => NZD.name(),
            Currency::NOK => NOK.name(),
            Currency::SEK => SEK.name(),
            Currency::PEN => PEN.name(),
            Currency::CNH => CNH.name(),
            Currency::INR => INR.name(),
            Currency::TWD => TWD.name(),
            Currency::HKD => HKD.name(),
            Currency::KRW => KRW.name(),
            Currency::DKK => DKK.name(),
            Currency::IDR => IDR.name(),
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
            Currency::BRL => BRL.symbol(),
            Currency::COP => COP.symbol(),
            Currency::MXN => MXN.symbol(),
            Currency::AUD => AUD.symbol(),
            Currency::CAD => CAD.symbol(),
            Currency::CNY => CNY.symbol(),
            Currency::GBP => GBP.symbol(),
            Currency::NZD => NZD.symbol(),
            Currency::NOK => NOK.symbol(),
            Currency::SEK => SEK.symbol(),
            Currency::PEN => PEN.symbol(),
            Currency::CNH => CNH.symbol(),
            Currency::INR => INR.symbol(),
            Currency::TWD => TWD.symbol(),
            Currency::HKD => HKD.symbol(),
            Currency::KRW => KRW.symbol(),
            Currency::DKK => DKK.symbol(),
            Currency::IDR => IDR.symbol(),
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
            Currency::BRL => BRL.precision(),
            Currency::COP => COP.precision(),
            Currency::MXN => MXN.precision(),
            Currency::AUD => AUD.precision(),
            Currency::CAD => CAD.precision(),
            Currency::CNY => CNY.precision(),
            Currency::GBP => GBP.precision(),
            Currency::NZD => NZD.precision(),
            Currency::NOK => NOK.precision(),
            Currency::SEK => SEK.precision(),
            Currency::PEN => PEN.precision(),
            Currency::CNH => CNH.precision(),
            Currency::INR => INR.precision(),
            Currency::TWD => TWD.precision(),
            Currency::HKD => HKD.precision(),
            Currency::KRW => KRW.precision(),
            Currency::DKK => DKK.precision(),
            Currency::IDR => IDR.precision(),
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
            Currency::BRL => BRL.numeric_code(),
            Currency::COP => COP.numeric_code(),
            Currency::MXN => MXN.numeric_code(),
            Currency::AUD => AUD.numeric_code(),
            Currency::CAD => CAD.numeric_code(),
            Currency::CNY => CNY.numeric_code(),
            Currency::GBP => GBP.numeric_code(),
            Currency::NZD => NZD.numeric_code(),
            Currency::NOK => NOK.numeric_code(),
            Currency::SEK => SEK.numeric_code(),
            Currency::PEN => PEN.numeric_code(),
            Currency::CNH => CNH.numeric_code(),
            Currency::INR => INR.numeric_code(),
            Currency::TWD => TWD.numeric_code(),
            Currency::HKD => HKD.numeric_code(),
            Currency::KRW => KRW.numeric_code(),
            Currency::DKK => DKK.numeric_code(),
            Currency::IDR => IDR.numeric_code(),
        }
    }
}
