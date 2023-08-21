use crate::{
    rates::interestrate::RateDefinition,
    time::{date::Date, period::Period},
};
use std::collections::HashMap;

use super::traits::FixingRateHolder;

/// # IborIndex
/// Struct that defines an Ibor index.
/// # Example
/// ```
/// use rustatlas::rates::interestrateindex::iborindex::IborIndex;
/// use rustatlas::rates::interestrate::RateDefinition;
/// use rustatlas::time::period::Period;
/// use rustatlas::time::date::Date;
/// use rustatlas::time::daycounters::enums::DayCounter;
/// use rustatlas::rates::enums::Compounding;
/// use rustatlas::time::enums::{Frequency, TimeUnit};
/// use rustatlas::rates::interestrateindex::traits::FixingRateHolder;
/// let tenor = Period::new(1, TimeUnit::Months);
/// let rate_definition = RateDefinition::new(Compounding::Simple, Frequency::Annual, DayCounter::Actual360);
/// let ibor_index = IborIndex::new(tenor).with_rate_definition(rate_definition);
/// assert_eq!(ibor_index.fixing_tenor(), tenor);
/// assert_eq!(ibor_index.rate_definition().compounding(), Compounding::Simple);
/// assert_eq!(ibor_index.rate_definition().frequency(), Frequency::Annual);
/// assert_eq!(ibor_index.rate_definition().day_counter(), DayCounter::Actual360);
/// ```
#[derive(Debug, Clone)]
pub struct IborIndex {
    tenor: Period,
    rate_definition: RateDefinition,
    fixings: HashMap<Date, f64>,
}

impl IborIndex {
    pub fn new(tenor: Period) -> IborIndex {
        IborIndex {
            tenor,
            rate_definition: RateDefinition::common_definition(),
            fixings: HashMap::new(),
        }
    }

    pub fn with_rate_definition(mut self, rate_definition: RateDefinition) -> Self {
        self.rate_definition = rate_definition;
        self
    }

    pub fn with_fixings(mut self, fixings: HashMap<Date, f64>) -> Self {
        self.fixings = fixings;
        self
    }
}

impl FixingRateHolder for IborIndex {
    fn fixing(&self, date: Date) -> Option<f64> {
        match self.fixings.get(&date) {
            Some(rate) => Some(*rate),
            None => None,
        }
    }

    fn add_fixing(&mut self, date: Date, rate: f64) {
        self.fixings.insert(date, rate);
    }

    fn rate_definition(&self) -> RateDefinition {
        self.rate_definition.clone()
    }

    fn fixing_tenor(&self) -> Period {
        self.tenor
    }
}
