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

use super::traits::FixingProvider;

/// # IborIndex
/// Struct that defines an Ibor index.
///
/// # Example
/// ```
/// use rustatlas::prelude::*;
///
/// let tenor = Period::new(1, TimeUnit::Months);
/// let rate_definition = RateDefinition::new(DayCounter::Actual360, Compounding::Simple, Frequency::Annual);
/// let ibor_index = IborIndex::new().with_tenor(tenor).with_rate_definition(rate_definition);
/// assert_eq!(ibor_index.tenor(), tenor);
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
    provider_id: Option<usize>,
}

impl IborIndex {
    pub fn new() -> IborIndex {
        IborIndex {
            tenor: Period::empty(),
            rate_definition: RateDefinition::default(),
            fixings: HashMap::new(),
            term_structure: None,
            provider_id: None,
        }
    }

    pub fn tenor(&self) -> Period {
        self.tenor
    }

    pub fn rate_definition(&self) -> RateDefinition {
        self.rate_definition
    }

    pub fn term_structure(&self) -> Option<YieldTermStructure> {
        self.term_structure
    }

    pub fn provider_id(&self) -> Option<usize> {
        self.provider_id
    }

    pub fn with_tenor(mut self, tenor: Period) -> Self {
        self.tenor = tenor;
        self
    }

    pub fn with_frequency(mut self, frequency: Frequency) -> Self {
        self.tenor = Period::from_frequency(frequency).expect("Invalid frequency");
        self
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

    pub fn with_provider_id(mut self, provider_id: usize) -> Self {
        self.provider_id = Some(provider_id);
        self
    }
}

impl FixingProvider for IborIndex {
    fn fixing(&self, date: Date) -> Option<f64> {
        match self.fixings.get(&date) {
            Some(rate) => Some(*rate),
            None => None,
        }
    }

    fn add_fixing(&mut self, date: Date, rate: f64) {
        self.fixings.insert(date, rate);
    }
}

impl HasReferenceDate for IborIndex {
    fn reference_date(&self) -> Date {
        match self.fixings.keys().max() {
            Some(date) => *date,
            None => self
                .term_structure
                .expect("No term structure for this IborIndex")
                .reference_date(),
        }
    }
}

impl YieldProvider for IborIndex {
    fn discount_factor(&self, date: Date) -> f64 {
        if date < self.reference_date() {
            panic!("Date must be greater than reference date");
        }
        self.term_structure
            .expect("No term structure for this IborIndex")
            .discount_factor(date)
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
            self.fixing(start_date)
                .expect("No fixing for this IborIndex")
        } else {
            self.term_structure
                .expect("No term structure for this IborIndex")
                .forward_rate(start_date, end_date, comp, freq)
        }
    }
}
