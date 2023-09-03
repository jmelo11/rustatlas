use std::collections::HashMap;

use crate::{
    rates::{
        enums::Compounding,
        interestrate::{InterestRate, RateDefinition},
        traits::{HasReferenceDate, YieldProvider},
        yieldtermstructure::enums::YieldTermStructure,
    },
    time::{
        date::Date,
        enums::{Frequency, TimeUnit},
        period::Period,
    },
};

use super::traits::FixingProvider;

/// # OvernightIndex
/// Overnight index.
#[derive(Clone)]
pub struct OvernightIndex {
    fixings: HashMap<Date, f64>,
    term_structure: Option<YieldTermStructure>,
    rate_definition: RateDefinition,
    tenor: Period,
}

impl OvernightIndex {
    pub fn new() -> OvernightIndex {
        OvernightIndex {
            fixings: HashMap::new(),
            term_structure: None,
            rate_definition: RateDefinition::default(),
            tenor: Period::new(1, TimeUnit::Days),
        }
    }

    pub fn term_structure(&self) -> Option<YieldTermStructure> {
        self.term_structure
    }

    pub fn rate_definition(&self) -> RateDefinition {
        self.rate_definition.clone()
    }

    pub fn tenor(&self) -> Period {
        self.tenor
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

    pub fn average_rate(&self, start_date: Date, end_date: Date) -> f64 {
        let start_index = match self.fixings.get(&start_date) {
            Some(rate) => *rate,
            None => panic!("No fixing rate for date {}", start_date),
        };
        let end_index = match self.fixings.get(&end_date) {
            Some(rate) => *rate,
            None => panic!("No fixing rate for date {}", end_date),
        };

        let comp = end_index / start_index;
        let day_counter = self.rate_definition.day_counter();
        InterestRate::implied_rate(
            comp,
            day_counter,
            self.rate_definition.compounding(),
            self.rate_definition.frequency(),
            day_counter.year_fraction(start_date, end_date),
        )
        .rate()
    }
}

impl FixingProvider for OvernightIndex {
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

impl HasReferenceDate for OvernightIndex {
    fn reference_date(&self) -> Date {
        match self.fixings.keys().max() {
            Some(date) => *date,
            None => match self.term_structure {
                Some(term_structure) => term_structure.reference_date(),
                None => panic!("No term structure for this OvernightIndex"),
            },
        }
    }
}

impl YieldProvider for OvernightIndex {
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
        // mixed case - return w.a.
        if start_date < self.reference_date() && end_date > self.reference_date() {
            let first_fixing = match self.fixing(start_date) {
                Some(fixing) => fixing,
                None => panic!("No fixing rate for date {}", self.reference_date()),
            };
            let second_fixing = match self.fixing(self.reference_date()) {
                Some(fixing) => fixing,
                None => panic!("No fixing rate for date {}", end_date),
            };

            let df = match self.term_structure {
                Some(term_structure) => term_structure.discount_factor(end_date),
                None => panic!("No term structure for this OvernightIndex"),
            };

            let third_fixing = second_fixing / df;

            let comp = third_fixing / first_fixing;
            let day_counter = self.rate_definition.day_counter();
            return InterestRate::implied_rate(
                comp,
                day_counter,
                self.rate_definition.compounding(),
                self.rate_definition.frequency(),
                day_counter.year_fraction(start_date, end_date),
            )
            .rate();
        }

        // past fixing case
        if start_date < self.reference_date() && end_date <= self.reference_date() {
            return self.average_rate(start_date, end_date);
        }

        // forecast case
        if start_date >= self.reference_date() && end_date > self.reference_date() {
            match self.term_structure {
                Some(term_structure) => {
                    return term_structure.forward_rate(start_date, end_date, comp, freq)
                }
                None => panic!("No term structure for this OvernightIndex"),
            }
        } else {
            panic!("Invalid start and end dates")
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        rates::yieldtermstructure::flatforwardtermstructure::FlatForwardTermStructure,
        time::daycounter::DayCounter,
    };

    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_new_overnight_index() {
        let overnight_index = OvernightIndex::new();
        assert!(overnight_index.fixings.is_empty());
        assert!(overnight_index.term_structure.is_none());
    }

    #[test]
    fn test_with_rate_definition() {
        let overnight_index = OvernightIndex::new().with_rate_definition(RateDefinition::default());
        assert_eq!(overnight_index.rate_definition, RateDefinition::default());
    }

    #[test]
    fn test_with_fixings() {
        let mut fixings = HashMap::new();
        fixings.insert(Date::new(2021, 1, 1), 0.02);
        let overnight_index = OvernightIndex::new().with_fixings(fixings.clone());
        assert_eq!(overnight_index.fixings, fixings);
    }

    #[test]
    fn test_average_rate() {
        let mut fixings = HashMap::new();
        let start_date = Date::new(2021, 1, 1);
        let end_date = Date::new(2022, 1, 1);

        fixings.insert(start_date, 100.0);
        fixings.insert(end_date, 105.0);
        let overnight_index = OvernightIndex::new()
            .with_fixings(fixings)
            .with_rate_definition(RateDefinition::default());

        let average_rate = overnight_index.average_rate(start_date, end_date);

        // Add your assertions here based on how average_rate is calculated
        assert!(average_rate > 0.0);
    }

    #[test]
    fn test_fixing() {
        let mut fixings = HashMap::new();
        fixings.insert(Date::new(2021, 1, 1), 0.02);
        let overnight_index = OvernightIndex::new().with_fixings(fixings);

        assert_eq!(overnight_index.fixing(Date::new(2021, 1, 1)), Some(0.02));
        assert_eq!(overnight_index.fixing(Date::new(2021, 1, 2)), None);
    }

    #[test]
    fn test_reference_date() {
        let mut fixings = HashMap::new();
        let ref_date = Date::new(2021, 1, 1);

        fixings.insert(ref_date, 100.0);
        let mut overnight_index = OvernightIndex::new()
            .with_fixings(fixings.clone())
            .with_term_structure(YieldTermStructure::FlatForwardTermStructure(
                FlatForwardTermStructure::new(
                    ref_date,
                    InterestRate::new(
                        0.02,
                        Compounding::Simple,
                        Frequency::Annual,
                        DayCounter::Actual360,
                    ),
                ),
            ));

        assert_eq!(overnight_index.reference_date(), ref_date);

        let next_date = Date::new(2021, 1, 2);
        overnight_index.add_fixing(next_date, 100.2);

        assert_eq!(overnight_index.reference_date(), next_date);

        let next_date_2 = Date::new(2021, 1, 3);
        let overnight_index = OvernightIndex::new()
            .with_term_structure(YieldTermStructure::FlatForwardTermStructure(
                FlatForwardTermStructure::new(
                    next_date_2,
                    InterestRate::new(
                        0.02,
                        Compounding::Simple,
                        Frequency::Annual,
                        DayCounter::Actual360,
                    ),
                ),
            ))
            .with_fixings(fixings);

        assert_eq!(overnight_index.reference_date(), ref_date);
    }
}
