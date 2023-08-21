use crate::{
    rates::interestrate::RateDefinition,
    time::{date::Date, period::Period},
};

use super::{iborindex::IborIndex, traits::FixingRateHolder};

/// # InterestRateIndex
/// Enum that defines an interest rate index.
#[derive(Debug, Clone)]
pub enum InterestRateIndex {
    IborIndex(IborIndex),
    Other,
}

impl FixingRateHolder for InterestRateIndex {
    fn fixing(&self, date: Date) -> Option<f64> {
        match self {
            InterestRateIndex::IborIndex(ibor_index) => ibor_index.fixing(date),
            InterestRateIndex::Other => panic!("No fixing for this index"),
        }
    }

    fn add_fixing(&mut self, date: Date, rate: f64) {
        match self {
            InterestRateIndex::IborIndex(ibor_index) => ibor_index.add_fixing(date, rate),
            InterestRateIndex::Other => panic!("No fixing for this index"),
        }
    }

    fn rate_definition(&self) -> RateDefinition {
        match self {
            InterestRateIndex::IborIndex(ibor_index) => ibor_index.rate_definition(),
            InterestRateIndex::Other => panic!("No rate definition for this index"),
        }
    }

    fn fixing_tenor(&self) -> Period {
        match self {
            InterestRateIndex::IborIndex(ibor_index) => ibor_index.fixing_tenor(),
            InterestRateIndex::Other => panic!("No fixing tenor for this index"),
        }
    }
}
