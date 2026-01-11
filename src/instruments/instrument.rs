use serde::{Deserialize, Serialize};

use crate::{
    cashflows::cashflow::{Cashflow, Side},
    core::traits::HasCurrency,
    currencies::enums::Currency,
    rates::interestrate::RateDefinition,
    time::{date::Date, enums::Frequency},
    utils::errors::{AtlasError, Result},
    visitors::traits::HasCashflows,
};

use super::{
    doublerateinstrument::DoubleRateInstrument, fixedrateinstrument::FixedRateInstrument,
    floatingrateinstrument::FloatingRateInstrument, hybridrateinstrument::HybridRateInstrument,
    traits::Structure,
};

/// # `RateType`
/// Represents the type of rate.
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RateType {
    /// Fixed rate type.
    Fixed,
    /// Floating rate type.
    Floating,
    /// Fixed then floating rate type.
    FixedThenFloating,
    /// Floating then fixed rate type.
    FloatingThenFixed,
    /// Fixed then fixed rate type.
    FixedThenFixed,
    /// Shuffled rate type.
    Suffled,
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
            "Suffled" => Ok(RateType::Suffled),
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
            RateType::Suffled => "Suffled".to_string(),
        }
    }
}

/// # `Instrument`
/// Represents an instrument. This is a wrapper around the `FixedRateInstrument` and
/// `FloatingRateInstrument` types.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Instrument {
    /// Fixed rate instrument.
    FixedRateInstrument(FixedRateInstrument),
    /// Floating rate instrument.
    FloatingRateInstrument(FloatingRateInstrument),
    /// Hybrid rate instrument.
    HybridRateInstrument(HybridRateInstrument),
    /// Double rate instrument.
    DoubleRateInstrument(DoubleRateInstrument),
}

impl HasCashflows for Instrument {
    fn cashflows(&self) -> &[Cashflow] {
        match self {
            Instrument::FixedRateInstrument(fri) => fri.cashflows(),
            Instrument::FloatingRateInstrument(fri) => fri.cashflows(),
            Instrument::HybridRateInstrument(hri) => hri.cashflows(),
            Instrument::DoubleRateInstrument(dri) => dri.cashflows(),
        }
    }

    fn mut_cashflows(&mut self) -> &mut [Cashflow] {
        match self {
            Instrument::FixedRateInstrument(fri) => fri.mut_cashflows(),
            Instrument::FloatingRateInstrument(fri) => fri.mut_cashflows(),
            Instrument::HybridRateInstrument(hri) => hri.mut_cashflows(),
            Instrument::DoubleRateInstrument(dri) => dri.mut_cashflows(),
        }
    }
}

impl Instrument {
    /// Returns the notional value of the instrument.
    #[must_use]
    pub fn notional(&self) -> f64 {
        match self {
            Instrument::FixedRateInstrument(fri) => fri.notional(),
            Instrument::FloatingRateInstrument(fri) => fri.notional(),
            Instrument::HybridRateInstrument(hri) => hri.notional(),
            Instrument::DoubleRateInstrument(dri) => dri.notional(),
        }
    }

    /// Returns the start date of the instrument.
    #[must_use]
    pub fn start_date(&self) -> Date {
        match self {
            Instrument::FixedRateInstrument(fri) => fri.start_date(),
            Instrument::FloatingRateInstrument(fri) => fri.start_date(),
            Instrument::HybridRateInstrument(hri) => hri.start_date(),
            Instrument::DoubleRateInstrument(dri) => dri.start_date(),
        }
    }

    /// Returns the end date of the instrument.
    #[must_use]
    pub fn end_date(&self) -> Date {
        match self {
            Instrument::FixedRateInstrument(fri) => fri.end_date(),
            Instrument::FloatingRateInstrument(fri) => fri.end_date(),
            Instrument::HybridRateInstrument(hri) => hri.end_date(),
            Instrument::DoubleRateInstrument(dri) => dri.end_date(),
        }
    }

    /// Returns the identifier of the instrument.
    #[must_use]
    pub fn id(&self) -> Option<String> {
        match self {
            Instrument::FixedRateInstrument(fri) => fri.id(),
            Instrument::FloatingRateInstrument(fri) => fri.id(),
            Instrument::HybridRateInstrument(hri) => hri.id(),
            Instrument::DoubleRateInstrument(dri) => dri.id(),
        }
    }

    /// Returns the structure of the instrument.
    #[must_use]
    pub fn structure(&self) -> Structure {
        match self {
            Instrument::FixedRateInstrument(fri) => fri.structure(),
            Instrument::FloatingRateInstrument(fri) => fri.structure(),
            Instrument::HybridRateInstrument(hri) => hri.structure(),
            _ => todo!(),
        }
    }

    /// Returns the payment frequency of the instrument.
    #[must_use]
    pub fn payment_frequency(&self) -> Frequency {
        match self {
            Instrument::FixedRateInstrument(fri) => fri.payment_frequency(),
            Instrument::FloatingRateInstrument(fri) => fri.payment_frequency(),
            Instrument::HybridRateInstrument(hri) => hri.payment_frequency(),
            Instrument::DoubleRateInstrument(dri) => dri.payment_frequency(),
        }
    }

    /// Returns the side of the instrument.
    #[must_use]
    pub fn side(&self) -> Option<Side> {
        match self {
            Instrument::FixedRateInstrument(fri) => Some(fri.side()),
            Instrument::FloatingRateInstrument(fri) => Some(fri.side()),
            Instrument::HybridRateInstrument(hri) => hri.side(),
            Instrument::DoubleRateInstrument(dri) => Some(dri.side()),
        }
    }

    /// Returns the issue date of the instrument.
    #[must_use]
    pub fn issue_date(&self) -> Option<Date> {
        match self {
            Instrument::FixedRateInstrument(fri) => fri.issue_date(),
            Instrument::FloatingRateInstrument(fri) => fri.issue_date(),
            Instrument::HybridRateInstrument(hri) => hri.issue_date(),
            Instrument::DoubleRateInstrument(dri) => dri.issue_date(),
        }
    }

    /// Returns the rate type of the instrument.
    #[must_use]
    pub fn rate_type(&self) -> RateType {
        match self {
            Instrument::FixedRateInstrument(_) => RateType::Fixed,
            Instrument::FloatingRateInstrument(_) => RateType::Floating,
            Instrument::HybridRateInstrument(hri) => hri.rate_type(),
            Instrument::DoubleRateInstrument(dri) => dri.rate_type(),
        }
    }

    /// Returns the fixed rate of the instrument, if applicable.
    #[must_use]
    pub fn rate(&self) -> Option<f64> {
        match self {
            Instrument::FixedRateInstrument(fri) => Some(fri.rate().rate()),
            Instrument::FloatingRateInstrument(_) => None,
            Instrument::HybridRateInstrument(_) => todo!(),
            Instrument::DoubleRateInstrument(_) => todo!(),
        }
    }

    /// Returns the spread of the instrument, if applicable.
    #[must_use]
    pub fn spread(&self) -> Option<f64> {
        match self {
            Instrument::FixedRateInstrument(_) => None,
            Instrument::FloatingRateInstrument(fri) => Some(fri.spread()),
            Instrument::HybridRateInstrument(_) => todo!(),
            Instrument::DoubleRateInstrument(_) => todo!(),
        }
    }

    /// Returns the forecast curve identifier of the instrument.
    #[must_use]
    pub fn forecast_curve_id(&self) -> Option<usize> {
        match self {
            Instrument::FixedRateInstrument(_) => None,
            Instrument::FloatingRateInstrument(fri) => fri.forecast_curve_id(),
            Instrument::HybridRateInstrument(hri) => hri.forecast_curve_id(),
            Instrument::DoubleRateInstrument(dri) => dri.forecast_curve_id(),
        }
    }

    /// Returns the discount curve identifier of the instrument.
    #[must_use]
    pub fn discount_curve_id(&self) -> Option<usize> {
        match self {
            Instrument::FixedRateInstrument(fri) => fri.discount_curve_id(),
            Instrument::FloatingRateInstrument(fri) => fri.discount_curve_id(),
            Instrument::HybridRateInstrument(hri) => hri.discount_curve_id(),
            Instrument::DoubleRateInstrument(dri) => dri.discount_curve_id(),
        }
    }

    /// Sets the discount curve identifier for the instrument.
    pub fn set_discount_curve_id(&mut self, id: usize) {
        match self {
            Instrument::FixedRateInstrument(fri) => fri.set_discount_curve_id(id),
            Instrument::FloatingRateInstrument(fri) => fri.set_discount_curve_id(id),
            Instrument::HybridRateInstrument(hri) => hri.set_discount_curve_id(id),
            Instrument::DoubleRateInstrument(dri) => dri.set_discount_curve_id(id),
        }
    }

    /// Sets the forecast curve identifier for the instrument.
    pub fn set_forecast_curve_id(&mut self, id: usize) {
        match self {
            Instrument::FloatingRateInstrument(fri) => fri.set_forecast_curve_id(id),
            Instrument::HybridRateInstrument(hri) => hri.set_forecast_curve_id(id),
            Instrument::DoubleRateInstrument(dri) => dri.set_forecast_curve_id(id),
            _ => {}
        }
    }

    /// Returns the first rate definition of the instrument.
    #[must_use]
    pub fn first_rate_definition(&self) -> Option<RateDefinition> {
        match self {
            Instrument::FixedRateInstrument(fri) => Some(fri.rate().rate_definition()),
            Instrument::FloatingRateInstrument(fri) => Some(fri.rate_definition()),
            Instrument::HybridRateInstrument(hri) => hri.first_rate_definition(),
            Instrument::DoubleRateInstrument(dri) => dri.first_rate_definition(),
        }
    }

    /// Returns the second rate definition of the instrument.
    #[must_use]
    pub fn second_rate_definition(&self) -> Option<RateDefinition> {
        match self {
            Instrument::FixedRateInstrument(_) => None,
            Instrument::FloatingRateInstrument(_) => None,
            Instrument::HybridRateInstrument(hri) => hri.second_rate_definition(),
            Instrument::DoubleRateInstrument(dri) => dri.second_rate_definition(),
        }
    }
}

impl HasCurrency for Instrument {
    fn currency(&self) -> Result<Currency> {
        match self {
            Instrument::FixedRateInstrument(fri) => fri.currency(),
            Instrument::FloatingRateInstrument(fri) => fri.currency(),
            Instrument::HybridRateInstrument(hri) => hri.currency(),
            Instrument::DoubleRateInstrument(dri) => dri.currency(),
        }
    }
}
