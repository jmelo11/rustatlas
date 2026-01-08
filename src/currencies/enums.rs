use std::fmt;

use serde::{Deserialize, Serialize};

use super::traits::CurrencyDetails;
use crate::utils::errors::{AtlasError, Result};

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

impl Currency {
    /// Returns static metadata about the currency as
    /// (alphabetic code, display name, symbol, decimal precision, numeric ISO 4217 code).
    pub const fn details(self) -> (&'static str, &'static str, &'static str, u8, u16) {
        match self {
            Currency::USD => ("USD", "US Dollar", "$", 2, 840),
            Currency::EUR => ("EUR", "Euro", "€", 2, 978),
            Currency::JPY => ("JPY", "Japanese Yen", "¥", 0, 392),
            Currency::ZAR => ("ZAR", "South African Rand", "R", 2, 710),
            Currency::CLP => ("CLP", "Chilean Peso", "$", 0, 152),
            Currency::CLF => ("CLF", "Chilean Unidad de Fomento", "UF", 4, 990),
            Currency::CHF => ("CHF", "Swiss Franc", "Fr", 2, 756),
            Currency::BRL => ("BRL", "Brazilian Real", "R$", 2, 986),
            Currency::COP => ("COP", "Colombian Peso", "$", 2, 170),
            Currency::MXN => ("MXN", "Mexican Peso", "Mex$", 2, 484),
            Currency::AUD => ("AUD", "Australian Dollar", "A$", 2, 36),
            Currency::CAD => ("CAD", "Canadian Dollar", "Can$", 2, 124),
            Currency::CNY => ("CNY", "Chinese Yuan", "¥", 2, 156),
            Currency::GBP => ("GBP", "British Pound", "£", 2, 826),
            Currency::NZD => ("NZD", "New Zealand Dollar", "NZ$", 2, 554),
            Currency::NOK => ("NOK", "Norwegian Krone", "kr", 2, 578),
            Currency::SEK => ("SEK", "Swedish Krona", "kr", 2, 752),
            Currency::PEN => ("PEN", "Peruvian Sol", "S/.", 2, 604),
            Currency::CNH => ("CNH", "Chinese Yuan (offshore)", "¥", 2, 156),
            Currency::INR => ("INR", "Indian Rupee", "₹", 2, 356),
            Currency::TWD => ("TWD", "New Taiwan Dollar", "NT$", 2, 901),
            Currency::HKD => ("HKD", "Hong Kong Dollar", "HK$", 2, 344),
            Currency::KRW => ("KRW", "South Korean Won", "₩", 0, 410),
            Currency::DKK => ("DKK", "Danish Krone", "kr", 2, 208),
            Currency::IDR => ("IDR", "Indonesian Rupiah", "Rp", 2, 360),
        }
    }

    pub const fn as_str(self) -> &'static str {
        self.details().0
    }
    pub const fn name(self) -> &'static str {
        self.details().1
    }
    pub const fn symbol(self) -> &'static str {
        self.details().2
    }
    pub const fn precision(self) -> u8 {
        self.details().3
    }
    pub const fn numeric_code(self) -> u16 {
        self.details().4
    }
}

impl CurrencyDetails for Currency {
    fn code(&self) -> &'static str {
        self.as_str()
    }
    fn name(&self) -> &'static str {
        self.details().1
    }
    fn symbol(&self) -> &'static str {
        self.details().2
    }
    fn precision(&self) -> u8 {
        self.details().3
    }
    fn numeric_code(&self) -> u16 {
        self.details().4
    }
}

impl fmt::Display for Currency {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl TryFrom<&str> for Currency {
    type Error = AtlasError;

    fn try_from(s: &str) -> Result<Self> {
        // trim white space
        let s = s.trim();
        match s {
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

impl TryFrom<String> for Currency {
    type Error = AtlasError;
    fn try_from(s: String) -> Result<Self> {
        Self::try_from(s.as_str())
    }
}

impl std::str::FromStr for Currency {
    type Err = AtlasError;
    fn from_str(s: &str) -> Result<Self> {
        Self::try_from(s)
    }
}

impl From<Currency> for String {
    fn from(c: Currency) -> Self {
        c.as_str().to_owned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    const ALL: &[Currency] = &[
        Currency::USD,
        Currency::EUR,
        Currency::JPY,
        Currency::ZAR,
        Currency::CLP,
        Currency::CLF,
        Currency::CHF,
        Currency::BRL,
        Currency::COP,
        Currency::MXN,
        Currency::AUD,
        Currency::CAD,
        Currency::CNY,
        Currency::GBP,
        Currency::NZD,
        Currency::NOK,
        Currency::SEK,
        Currency::PEN,
        Currency::CNH,
        Currency::INR,
        Currency::TWD,
        Currency::HKD,
        Currency::KRW,
        Currency::DKK,
        Currency::IDR,
    ];

    #[test]
    fn details_are_self_consistent_for_all_currencies() {
        for &c in ALL {
            let (code, name, symbol, precision, numeric_code) = c.details();

            assert_eq!(c.as_str(), code);
            assert_eq!(c.name(), name);
            assert_eq!(c.symbol(), symbol);
            assert_eq!(c.precision(), precision);
            assert_eq!(c.numeric_code(), numeric_code);

            assert_eq!(CurrencyDetails::code(&c), code);
            assert_eq!(CurrencyDetails::name(&c), name);
            assert_eq!(CurrencyDetails::symbol(&c), symbol);
            assert_eq!(CurrencyDetails::precision(&c), precision);
            assert_eq!(CurrencyDetails::numeric_code(&c), numeric_code);

            assert_eq!(c.to_string(), code);

            let s: String = c.into();
            assert_eq!(s, code);
        }
    }

    #[test]
    fn try_from_str_parses_known_codes_and_trims() {
        assert_eq!(Currency::try_from("USD").unwrap(), Currency::USD);
        assert_eq!(Currency::try_from("  USD ").unwrap(), Currency::USD);
        assert_eq!(Currency::try_from("\nEUR\t").unwrap(), Currency::EUR);
    }

    #[test]
    fn try_from_string_parses_same_as_str() {
        let c = Currency::try_from("JPY".to_string()).unwrap();
        assert_eq!(c, Currency::JPY);
    }

    #[test]
    fn from_str_parses_same_as_try_from() {
        let c = Currency::from_str("GBP").unwrap();
        assert_eq!(c, Currency::GBP);
    }

    #[test]
    fn invalid_currency_rejected() {
        assert!(Currency::try_from("NOPE").is_err());
        assert!(Currency::from_str("usd").is_err());
        assert!(Currency::try_from("").is_err());
    }

    #[test]
    fn spot_checks_for_non_trivial_metadata() {
        // Precision edge cases
        assert_eq!(Currency::JPY.precision(), 0);
        assert_eq!(Currency::CLF.precision(), 4);

        assert_eq!(Currency::USD.numeric_code(), 840);
        assert_eq!(Currency::EUR.numeric_code(), 978);
        assert_eq!(Currency::KRW.numeric_code(), 410);

        assert_eq!(Currency::EUR.symbol(), "€");
        assert_eq!(Currency::KRW.symbol(), "₩");

        assert_eq!(Currency::CNY.numeric_code(), 156);
        assert_eq!(Currency::CNH.numeric_code(), 156);
    }
}
