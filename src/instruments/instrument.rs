use serde::{Deserialize, Serialize};

use crate::{
    cashflows::cashflow::{Cashflow, Side},
    currencies::enums::Currency,
    prelude::{HasCurrency, InterestAccrual},
    time::{date::Date, enums::Frequency},
    utils::errors::{AtlasError, Result},
    visitors::traits::HasCashflows,
};

use super::{
    fixedrateinstrument::FixedRateInstrument, floatingrateinstrument::FloatingRateInstrument,
    traits::Structure,
};

/// # PositionType
/// This enum is used to differentiate between base and simulated positions
#[derive(Clone, Copy, Debug, Serialize, Deserialize, Eq, PartialEq)]
pub enum PositionType {
    Base,
    Simulated,
}

impl TryFrom<String> for PositionType {
    type Error = AtlasError;

    fn try_from(s: String) -> Result<Self> {
        match s.as_str() {
            "Base" => Ok(PositionType::Base),
            "Simulated" => Ok(PositionType::Simulated),
            _ => Err(AtlasError::InvalidValueErr(format!(
                "Invalid position type: {}",
                s
            ))),
        }
    }
}

impl From<PositionType> for String {
    fn from(position_type: PositionType) -> Self {
        match position_type {
            PositionType::Base => "Base".to_string(),
            PositionType::Simulated => "Simulated".to_string(),
        }
    }
}

/// # RateType
/// Represents the type of rate. It can be either fixed or floating.
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RateType {
    Fixed,
    Floating,
}

impl TryFrom<String> for RateType {
    type Error = AtlasError;

    fn try_from(s: String) -> Result<Self> {
        match s.as_str() {
            "Fixed" => Ok(RateType::Fixed),
            "Floating" => Ok(RateType::Floating),
            _ => Err(AtlasError::InvalidValueErr(format!(
                "Invalid rate type: {}",
                s
            ))),
        }
    }
}

impl From<RateType> for String {
    fn from(rate_type: RateType) -> Self {
        match rate_type {
            RateType::Fixed => "Fixed".to_string(),
            RateType::Floating => "Floating".to_string(),
        }
    }
}

/// # Instrument
/// Represents an instrument. This is a wrapper around the FixedRateInstrument and FloatingRateInstrument.
#[derive(Clone)]
pub enum Instrument {
    FixedRateInstrument(FixedRateInstrument),
    FloatingRateInstrument(FloatingRateInstrument),
}

impl InterestAccrual for Instrument {
    fn accrual_start_date(&self) -> Date {
        match self {
            Instrument::FixedRateInstrument(fri) => fri.accrual_start_date(),
            Instrument::FloatingRateInstrument(fri) => fri.accrual_start_date(),
        }
    }

    fn accrual_end_date(&self) -> Date {
        match self {
            Instrument::FixedRateInstrument(fri) => fri.accrual_end_date(),
            Instrument::FloatingRateInstrument(fri) => fri.accrual_end_date(),
        }
    }

    fn accrued_amount (&self, start_date: Date, end_date: Date) -> Result<f64> {
        match self {
            Instrument::FixedRateInstrument(fri) => fri.accrued_amount(start_date, end_date),
            Instrument::FloatingRateInstrument(fri) => fri.accrued_amount(start_date, end_date),
        }
    }
}

impl HasCashflows for Instrument {
    fn cashflows(&self) -> &[Cashflow] {
        match self {
            Instrument::FixedRateInstrument(fri) => fri.cashflows(),
            Instrument::FloatingRateInstrument(fri) => fri.cashflows(),
        }
    }

    fn mut_cashflows(&mut self) -> &mut [Cashflow] {
        match self {
            Instrument::FixedRateInstrument(fri) => fri.mut_cashflows(),
            Instrument::FloatingRateInstrument(fri) => fri.mut_cashflows(),
        }
    }
}

impl Instrument {
    pub fn notional(&self) -> f64 {
        match self {
            Instrument::FixedRateInstrument(fri) => fri.notional(),
            Instrument::FloatingRateInstrument(fri) => fri.notional(),
        }
    }

    pub fn start_date(&self) -> Date {
        match self {
            Instrument::FixedRateInstrument(fri) => fri.start_date(),
            Instrument::FloatingRateInstrument(fri) => fri.start_date(),
        }
    }

    pub fn end_date(&self) -> Date {
        match self {
            Instrument::FixedRateInstrument(fri) => fri.end_date(),
            Instrument::FloatingRateInstrument(fri) => fri.end_date(),
        }
    }

    pub fn id(&self) -> Option<usize> {
        match self {
            Instrument::FixedRateInstrument(fri) => fri.id(),
            Instrument::FloatingRateInstrument(fri) => fri.id(),
        }
    }

    pub fn structure(&self) -> Structure {
        match self {
            Instrument::FixedRateInstrument(fri) => fri.structure(),
            Instrument::FloatingRateInstrument(fri) => fri.structure(),
        }
    }

    pub fn payment_frequency(&self) -> Frequency {
        match self {
            Instrument::FixedRateInstrument(fri) => fri.payment_frequency(),
            Instrument::FloatingRateInstrument(fri) => fri.payment_frequency(),
        }
    }

    pub fn side(&self) -> Side {
        match self {
            Instrument::FixedRateInstrument(fri) => fri.side(),
            Instrument::FloatingRateInstrument(fri) => fri.side(),
        }
    }

    pub fn issue_date(&self) -> Option<Date> {
        match self {
            Instrument::FixedRateInstrument(fri) => fri.issue_date(),
            Instrument::FloatingRateInstrument(fri) => fri.issue_date(),
        }
    }

    pub fn rate_type(&self) -> RateType {
        match self {
            Instrument::FixedRateInstrument(_) => RateType::Fixed,
            Instrument::FloatingRateInstrument(_) => RateType::Floating,
        }
    }
}

impl HasCurrency for Instrument {
    fn currency(&self) -> Result<Currency> {
        match self {
            Instrument::FixedRateInstrument(fri) => Ok(fri.currency()),
            Instrument::FloatingRateInstrument(fri) => Ok(fri.currency()),
        }
    }
}
