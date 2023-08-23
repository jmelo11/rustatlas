use crate::{
    prelude::{Compounding, Frequency, YieldProvider},
    rates::{interestrate::RateDefinition, traits::HasReferenceDate},
    time::{date::Date, period::Period},
};

use super::{iborindex::IborIndex, traits::FloatingRateProvider};

/// # InterestRateIndex
/// Enum that defines an interest rate index.
#[derive(Clone)]
pub enum InterestRateIndex {
    IborIndex(IborIndex),
    Other,
}

impl FloatingRateProvider for InterestRateIndex {
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

impl HasReferenceDate for InterestRateIndex {
    fn reference_date(&self) -> Date {
        match self {
            InterestRateIndex::IborIndex(ibor_index) => ibor_index.reference_date(),
            InterestRateIndex::Other => panic!("No reference date for this index"),
        }
    }
}

impl YieldProvider for InterestRateIndex {
    fn discount_factor(&self, date: Date) -> f64 {
        match self {
            InterestRateIndex::IborIndex(ibor_index) => ibor_index.discount_factor(date),
            InterestRateIndex::Other => panic!("No discount factor for this index"),
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
            InterestRateIndex::Other => panic!("No forward rate for this index"),
        }
    }
}
