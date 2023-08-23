use std::collections::HashMap;

use crate::{
    prelude::DayCountProvider,
    rates::{
        enums::Compounding,
        interestrate::RateDefinition,
        traits::{HasReferenceDate, YieldProvider},
        yieldtermstructure::enums::YieldTermStructure,
    },
    time::{
        date::Date,
        enums::{Frequency, TimeUnit},
        period::Period,
    },
};

use super::traits::FloatingRateProvider;

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

    pub fn fixing_compounded_rate(&self, date: Date) -> f64 {
        let eval_fixings = self
            .fixings
            .iter()
            .filter(|(d, _)| d >= date)
            .sort_by(|(d1, _), (d2, _)| d1.cmp(d2));

        if eval_fixings.len() < 2 {
            panic!("Not enough fixings to compute compounded rate");
        } else if eval_fixings.len() == 1 {
            return *eval_fixings[0].1;
        } else {
            let day_counter = self.rate_definition.day_counter();
            let mut comp = 1.0;
            for i in 0..eval_fixings.len() {
                let d1 = eval_fixings[i].0;
                let d2 = eval_fixings[i + 1].0;
                let yf = day_counter.year_fraction(d1, d2);
                comp *= 1.0 + eval_fixings[i].1 * yf;
            }
            let yf = day_counter.year_fraction(date, eval_fixings.last());
            return (1.0 / comp - 1.0) / yf;
        }
    }
}

impl FloatingRateProvider for OvernightIndex {
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

impl HasReferenceDate for OvernightIndex {
    fn reference_date(&self) -> Date {
        match self.fixings.keys().max() {
            Some(date) => *date,
            None => panic!("No reference date for this IborIndex"),
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
            let comp_rate = self.fixing_compounded_rate(start_date);
            let forecast_rate = match self.term_structure {
                Some(term_structure) => {
                    term_structure.forward_rate(start_date, end_date, comp, freq)
                }
                None => panic!("No term structure for this IborIndex"),
            };
            let t1 = self
                .rate_definition
                .day_counter()
                .year_fraction(start_date, self.reference_date());
            let t2 = self
                .rate_definition
                .day_counter()
                .year_fraction(self.reference_date(), end_date);
            return (comp_rate * t1 + forecast_rate * t2) / (t1 + t2);
        }

        // forecast case
        if start_date >= self.reference_date() && end_date > self.reference_date() {
            match self.term_structure {
                Some(term_structure) => {
                    return term_structure.forward_rate(start_date, end_date, comp, freq)
                }
                None => panic!("No term structure for this IborIndex"),
            }
        } else {
            panic!("Invalid start and end dates")
        }
    }
}
