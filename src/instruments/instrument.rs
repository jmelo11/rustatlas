use serde::{Deserialize, Serialize};

use crate::{
    cashflows::{
        cashflow::{Cashflow, Side},
        traits::InterestAccrual,
    }, core::traits::HasCurrency, currencies::enums::Currency, prelude::RateDefinition, time::{date::Date, enums::Frequency}, utils::errors::{AtlasError, Result}, visitors::traits::HasCashflows
};

use super::{
    fixedrateinstrument::FixedRateInstrument, floatingrateinstrument::FloatingRateInstrument,
    traits::Structure,
};



/// # PositionType
/// This enum is used to differentiate between base and simulated positions
#[derive(Clone, Copy, Debug, Serialize, Deserialize, Eq, PartialEq, Hash)]
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
    FixedThenFloating,
    FloatingThenFixed,
    FixedThenFixed,
}

impl TryFrom<String> for RateType {
    type Error = AtlasError;

    fn try_from(s: String) -> Result<Self> {
        match s.as_str() {
            "Fixed" => Ok(RateType::Fixed),
            "Floating" => Ok(RateType::Floating),
            "FixedThenFloating" => Ok(RateType::FixedThenFloating),
            "FloatingThenFixed" => Ok(RateType::FloatingThenFixed),
            "FixedThenFixed" => Ok(RateType::FixedThenFixed),
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
            RateType::FixedThenFloating => "FixedThenFloating".to_string(),
            RateType::FloatingThenFixed => "FloatingThenFixed".to_string(),
            RateType::FixedThenFixed => "FixedThenFixed".to_string(),
        }
    }
}

/// # Instrument
/// Represents an instrument. This is a wrapper around the FixedRateInstrument and FloatingRateInstrument.
#[derive(Clone, Debug)]
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

    fn accrued_amount(&self, start_date: Date, end_date: Date) -> Result<f64> {
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

    pub fn id(&self) -> Option<String> {
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

    pub fn rate(&self) -> f64 {
        match self {
            Instrument::FixedRateInstrument(fri) => fri.rate().rate(),
            Instrument::FloatingRateInstrument(fri) => fri.spread(),
        }
    }

    pub fn forecast_curve_id(&self) -> Option<usize> {
        match self {
            Instrument::FixedRateInstrument(_) => None,
            Instrument::FloatingRateInstrument(fri) => fri.forecast_curve_id(),
        }
    }

    pub fn discount_curve_id(&self) -> Option<usize> {
        match self {
            Instrument::FixedRateInstrument(fri) => fri.discount_curve_id(),
            Instrument::FloatingRateInstrument(fri) => fri.discount_curve_id(),
        }
    }

    pub fn set_discount_curve_id(&mut self, id: usize) {
        match self {
            Instrument::FixedRateInstrument(fri) => fri.set_discount_curve_id(id),
            Instrument::FloatingRateInstrument(fri) => fri.set_discount_curve_id(id),
        }
    }

    pub fn set_forecast_curve_id(&mut self, id: usize) {
        match self {
            Instrument::FloatingRateInstrument(fri) => fri.set_forecast_curve_id(id),
            _ => {}
        }
    }

    pub fn first_rate_definition(&self) -> RateDefinition {
        match self {
            Instrument::FixedRateInstrument(fri) => *fri.rate().rate_definition(),
            Instrument::FloatingRateInstrument(fri) => fri.rate_definition(),
        }
    }
    
    pub fn second_rate_definition(&self) -> Option<RateDefinition> {
        match self {
            Instrument::FixedRateInstrument(_) => None,
            Instrument::FloatingRateInstrument(_) => None,
        }
    }
}

impl HasCurrency for Instrument {
    fn currency(&self) -> Result<Currency> {
        match self {
            Instrument::FixedRateInstrument(fri) => fri.currency(),
            Instrument::FloatingRateInstrument(fri) => fri.currency(),
        }
    }
}
