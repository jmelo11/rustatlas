use crate::{
    rates::interestrate::RateDefinition,
    time::{date::Date, period::Period},
};
use std::collections::HashMap;

/// # IborIndex
/// Struct that defines an Ibor index.
/// # Example
/// ```
/// use crate::rates::interestrateindex::iborindex::IborIndex;
/// use crate::rates::interestrate::RateDefinition;
/// use crate::time::period::Period;
/// use crate::time::date::Date;
/// use crate::time::daycounters::enums::DayCounter;
/// use crate::rates::enums::{Compounding, Frequency};
/// let tenor = Period::new(1, TimeUnit::Months);
/// let rate_definition = RateDefinition::new(Compounding::Simple, Frequency::Annual, DayCounter::Actual360);
/// let ibor_index = IborIndex::new(tenor).with_rate_definition(rate_definition);
/// assert_eq!(ibor_index.tenor(), tenor);
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

    pub fn with_rate_definition(&mut self, rate_definition: RateDefinition) -> &mut Self {
        self.rate_definition = rate_definition;
        return self;
    }

    pub fn with_fixings(&mut self, fixings: HashMap<Date, f64>) -> &mut Self {
        self.fixings = fixings;
        return self;
    }
}
