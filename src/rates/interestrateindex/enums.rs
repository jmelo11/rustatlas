use std::collections::HashMap;

use crate::{
    rates::{
        enums::Compounding,
        traits::{HasReferenceDate, YieldProvider, YieldProviderError},
        yieldtermstructure::enums::YieldTermStructure,
    },
    time::{date::Date, enums::Frequency, period::Period},
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

    fn fixings(&self) -> &HashMap<Date, f64> {
        match self {
            InterestRateIndex::IborIndex(ibor_index) => ibor_index.fixings(),
            InterestRateIndex::OvernightIndex(overnight_index) => overnight_index.fixings(),
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
    fn discount_factor(&self, date: Date) -> Result<f64, YieldProviderError> {
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
    ) -> Result<f64, YieldProviderError> {
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

impl InterestRateIndex {
    pub fn term_structure(&self) -> Option<YieldTermStructure> {
        match self {
            InterestRateIndex::IborIndex(ibor_index) => ibor_index.term_structure(),
            InterestRateIndex::OvernightIndex(overnight_index) => overnight_index.term_structure(),
        }
    }

    pub fn tenor(&self) -> Period {
        match self {
            InterestRateIndex::IborIndex(ibor_index) => ibor_index.tenor(),
            InterestRateIndex::OvernightIndex(overnight_index) => overnight_index.tenor(),
        }
    }

    pub fn provider_id(&self) -> Option<usize> {
        match self {
            InterestRateIndex::IborIndex(ibor_index) => ibor_index.provider_id(),
            InterestRateIndex::OvernightIndex(overnight_index) => overnight_index.provider_id(),
        }
    }
}

// impl AdvanceInTime for InterestRateIndex {
//     type Output = InterestRateIndex;
//     fn advance(&self, period: Period) -> Self::Output {
//         match self {
//             InterestRateIndex::IborIndex(ibor_index) => {
//                 InterestRateIndex::IborIndex(ibor_index.advance(period))
//             }
//             InterestRateIndex::OvernightIndex(overnight_index) => {
//                 InterestRateIndex::OvernightIndex(overnight_index.advance(period))
//             }
//         }
//     }
// }
