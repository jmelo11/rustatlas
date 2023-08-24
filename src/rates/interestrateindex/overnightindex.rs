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

    pub fn fixing_compounded_rate(&self, start_date: Date) -> Option<f64> {
        let mut sorted_dates: Vec<&Date> = self
            .fixings
            .keys()
            .filter(|&&date| date >= start_date)
            .collect();
        sorted_dates.sort();

        if sorted_dates.is_empty() {
            return None;
        }

        let day_counter = self.rate_definition.day_counter();

        let mut compounded_product = 1.0;
        let mut prev_date = start_date;

        for &date in sorted_dates.iter() {
            let rate = self.fixings.get(date).unwrap();
            let year_fraction = day_counter.year_fraction(prev_date, *date);

            compounded_product *= (1.0 + rate * year_fraction);
            prev_date = *date;
        }

        Some(compounded_product - 1.0)
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
            let comp_rate = match self.fixing_compounded_rate(start_date) {
                Some(rate) => rate,
                None => panic!("No fixing rate for this OvernightIndex"),
            };
            let forecast_rate = match self.term_structure {
                Some(term_structure) => {
                    term_structure.forward_rate(start_date, end_date, comp, freq)
                }
                None => panic!("No term structure for this OvernightIndex"),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fixing_compounded_rate_no_fixings() {
        let index = OvernightIndex::new().with_rate_definition(RateDefinition::default());

        let start_date = Date::new(2022, 1, 1);

        assert_eq!(index.fixing_compounded_rate(start_date), None);
    }

    #[test]
    fn test_fixing_compounded_rate_single_fixing() {
        let mut fixings = HashMap::new();
        fixings.insert(Date::new(2022, 1, 2), 0.01);

        let index = OvernightIndex::new()
            .with_rate_definition(RateDefinition::default())
            .with_fixings(fixings);

        let start_date = Date::new(2022, 1, 1);

        assert_eq!(
            index.fixing_compounded_rate(start_date),
            Some(0.01 * 1.0 / 360.0)
        );
    }

    #[test]
    fn test_fixing_compounded_rate_multiple_fixings() {
        let mut fixings = HashMap::new();
        fixings.insert(Date::new(2022, 1, 2), 0.01);
        fixings.insert(Date::new(2022, 1, 3), 0.02);
        fixings.insert(Date::new(2022, 1, 4), 0.015);

        let index = OvernightIndex::new()
            .with_rate_definition(RateDefinition::default())
            .with_fixings(fixings);

        let start_date = Date::new(2022, 1, 1);

        let expected_result =
            (1.0 + 0.01 * 1.0 / 360.0) * (1.0 + 0.02 * 1.0 / 360.0) * (1.0 + 0.015 * 1.0 / 360.0)
                - 1.0;

        assert_eq!(
            index.fixing_compounded_rate(start_date),
            Some(expected_result)
        );
    }

    #[test]
    fn test_fixing_compounded_rate_with_future_date() {
        let mut fixings = HashMap::new();
        fixings.insert(Date::new(2022, 1, 2), 0.01);

        let index = OvernightIndex::new()
            .with_rate_definition(RateDefinition::default())
            .with_fixings(fixings);

        let start_date = Date::new(2022, 1, 3);

        assert_eq!(index.fixing_compounded_rate(start_date), None);
    }
}
