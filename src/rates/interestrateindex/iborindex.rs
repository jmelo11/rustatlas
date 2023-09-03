use crate::{
    rates::yieldtermstructure::enums::YieldTermStructure,
    rates::{
        enums::Compounding,
        interestrate::RateDefinition,
        traits::{HasReferenceDate, YieldProvider},
    },
    time::{date::Date, enums::Frequency, period::Period},
};
use std::collections::HashMap;

use super::traits::FloatingRateProvider;

/// # IborIndex
/// Struct that defines an Ibor index.
///
/// # Example
/// ```
/// use rustatlas::prelude::*;
///
/// let tenor = Period::new(1, TimeUnit::Months);
/// let rate_definition = RateDefinition::new(Compounding::Simple, Frequency::Annual, DayCounter::Actual360);
/// let ibor_index = IborIndex::new(tenor).with_rate_definition(rate_definition);
/// assert_eq!(ibor_index.fixing_tenor(), tenor);
/// assert_eq!(ibor_index.rate_definition().compounding(), Compounding::Simple);
/// assert_eq!(ibor_index.rate_definition().frequency(), Frequency::Annual);
/// assert_eq!(ibor_index.rate_definition().day_counter(), DayCounter::Actual360);
/// ```
#[derive(Clone)]
pub struct IborIndex {
    tenor: Period,
    rate_definition: RateDefinition,
    fixings: HashMap<Date, f64>,
    term_structure: Option<YieldTermStructure>,
}

impl IborIndex {
    pub fn new(tenor: Period) -> IborIndex {
        IborIndex {
            tenor,
            rate_definition: RateDefinition::default(),
            fixings: HashMap::new(),
            term_structure: None,
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

    pub fn with_term_structure(mut self, term_structure: YieldTermStructure) -> Self {
        self.term_structure = Some(term_structure);
        self
    }
}

impl FloatingRateProvider for IborIndex {
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

impl HasReferenceDate for IborIndex {
    fn reference_date(&self) -> Date {
        match self.fixings.keys().max() {
            Some(date) => *date,
            None => match self.term_structure {
                Some(term_structure) => term_structure.reference_date(),
                None => panic!("No reference date for this IborIndex"),
            }
        }
    }
}

impl YieldProvider for IborIndex {
    fn discount_factor(&self, date: Date) -> f64 {
        if date < self.reference_date() {
            panic!("Date must be greater than reference date");
        }
        match self.term_structure {
            Some(term_structure) => term_structure.discount_factor(date),
            None => panic!("No term structure for this IborIndex"),
        }
    }

    fn forward_rate(
        &self,
        start_date: Date,
        end_date: Date,
        comp: Compounding,
        freq: Frequency,
    ) -> f64 {
        if end_date < start_date {
            panic!("End date must be greater than start date");
        }
        if start_date < self.reference_date() {
            let fixing = self.fixing(start_date);
            match fixing {
                Some(fixing) => return fixing,
                None => panic!("No fixing found for date {} for this IborIndex", start_date),
            }
        }
        match self.term_structure {
            Some(ts) => ts.forward_rate(start_date, end_date, comp, freq),
            None => panic!("No term structure for this IborIndex"),
        }
    }
}
