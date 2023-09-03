use crate::{
    rates::{
        enums::Compounding,
        traits::{HasReferenceDate, YieldProvider},
    },
    time::{date::Date, enums::Frequency},
};

use super::{iborindex::IborIndex, overnightindex::OvernightIndex, traits::FixingProvider};

/// # InterestRateIndex
/// Enum that defines an interest rate index.
#[derive(Clone)]
pub enum InterestRateIndex {
    IborIndex(IborIndex),
    OvernightIndex(OvernightIndex),
}

impl FixingProvider for InterestRateIndex {
    fn fixing(&self, date: Date) -> Option<f64> {
        match self {
            InterestRateIndex::IborIndex(ibor_index) => ibor_index.fixing(date),
            InterestRateIndex::OvernightIndex(overnight_index) => overnight_index.fixing(date),
        }
    }

    fn add_fixing(&mut self, date: Date, rate: f64) {
        match self {
            InterestRateIndex::IborIndex(ibor_index) => ibor_index.add_fixing(date, rate),
            InterestRateIndex::OvernightIndex(overnight_index) => {
                overnight_index.add_fixing(date, rate)
            }
        }
    }
}

impl HasReferenceDate for InterestRateIndex {
    fn reference_date(&self) -> Date {
        match self {
            InterestRateIndex::IborIndex(ibor_index) => ibor_index.reference_date(),
            InterestRateIndex::OvernightIndex(overnight_index) => overnight_index.reference_date(),
        }
    }
}

impl YieldProvider for InterestRateIndex {
    fn discount_factor(&self, date: Date) -> f64 {
        match self {
            InterestRateIndex::IborIndex(ibor_index) => ibor_index.discount_factor(date),
            InterestRateIndex::OvernightIndex(overnight_index) => {
                overnight_index.discount_factor(date)
            }
        }
    }

    fn forward_rate(
        &self,
        start_date: Date,
        end_date: Date,
        compounding: Compounding,
        frequency: Frequency,
    ) -> f64 {
        match self {
            InterestRateIndex::IborIndex(ibor_index) => {
                ibor_index.forward_rate(start_date, end_date, compounding, frequency)
            }
            InterestRateIndex::OvernightIndex(overnight_index) => {
                overnight_index.forward_rate(start_date, end_date, compounding, frequency)
            }
        }
    }
}
