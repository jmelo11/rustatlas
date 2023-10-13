use std::collections::HashMap;

use crate::{
    rates::{
        enums::Compounding,
        interestrate::{InterestRate, RateDefinition},
        traits::{HasReferenceDate, YieldProvider, YieldProviderError},
        yieldtermstructure::traits::YieldTermStructureTrait,
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
    term_structure: Option<Box<dyn YieldTermStructureTrait>>,
    rate_definition: RateDefinition,
    tenor: Period,
    provider_id: Option<usize>,
    reference_date: Date,
}

impl OvernightIndex {
    pub fn new(reference_date: Date) -> OvernightIndex {
        OvernightIndex {
            fixings: HashMap::new(),
            term_structure: None,
            rate_definition: RateDefinition::default(),
            tenor: Period::new(1, TimeUnit::Days),
            provider_id: None,
            reference_date: reference_date,
        }
    }

    pub fn term_structure(&self) -> Option<&dyn YieldTermStructureTrait> {
        self.term_structure.as_deref()
    }

    pub fn rate_definition(&self) -> RateDefinition {
        self.rate_definition
    }

    pub fn tenor(&self) -> Period {
        self.tenor
    }

    pub fn provider_id(&self) -> Option<usize> {
        self.provider_id
    }

    pub fn with_rate_definition(mut self, rate_definition: RateDefinition) -> Self {
        self.rate_definition = rate_definition;
        self
    }

    pub fn with_fixings(mut self, fixings: HashMap<Date, f64>) -> Self {
        self.fixings = fixings;
        self
    }

    pub fn with_term_structure(mut self, term_structure: Box<dyn YieldTermStructureTrait>) -> Self {
        self.term_structure = Some(term_structure);
        self
    }

    pub fn with_provider_id(mut self, provider_id: Option<usize>) -> Self {
        self.provider_id = provider_id;
        self
    }

    pub fn average_rate(
        &self,
        start_date: Date,
        end_date: Date,
    ) -> Result<f64, YieldProviderError> {
        let start_index = self
            .fixings
            .get(&start_date)
            .ok_or(YieldProviderError::NoFixingRate(start_date))?;
        let end_index = self
            .fixings
            .get(&end_date)
            .ok_or(YieldProviderError::NoFixingRate(end_date))?;

        let comp = end_index / start_index;
        let day_counter = self.rate_definition.day_counter();
        Ok(InterestRate::implied_rate(
            comp,
            day_counter,
            self.rate_definition.compounding(),
            self.rate_definition.frequency(),
            day_counter.year_fraction(start_date, end_date),
        )?
        .rate())
    }
}

impl FixingProvider for OvernightIndex {
    fn fixing(&self, date: Date) -> Option<f64> {
        self.fixings.get(&date).cloned()
    }

    fn fixings(&self) -> &HashMap<Date, f64> {
        &self.fixings
    }

    fn add_fixing(&mut self, date: Date, rate: f64) {
        self.fixings.insert(date, rate);
    }
}

impl HasReferenceDate for OvernightIndex {
    fn reference_date(&self) -> Date {
        self.reference_date
    }
}

impl YieldProvider for OvernightIndex {
    fn discount_factor(&self, date: Date) -> Result<f64, YieldProviderError> {
        self.term_structure()
            .ok_or(YieldProviderError::NoTermStructure)?
            .discount_factor(date)
    }

    fn forward_rate(
        &self,
        start_date: Date,
        end_date: Date,
        comp: Compounding,
        freq: Frequency,
    ) -> Result<f64, YieldProviderError> {
        // mixed case - return w.a.
        if start_date < self.reference_date() && end_date > self.reference_date() {
            let first_fixing = self
                .fixing(self.reference_date())
                .ok_or(YieldProviderError::NoFixingRate(self.reference_date()))?;
            let second_fixing = self
                .fixing(self.reference_date())
                .ok_or(YieldProviderError::NoFixingRate(self.reference_date()))?;

            let df = self
                .term_structure()
                .ok_or(YieldProviderError::NoTermStructure)?
                .discount_factor(end_date)?;

            let third_fixing = second_fixing / df;

            let comp = third_fixing / first_fixing;
            let day_counter = self.rate_definition.day_counter();
            return Ok(InterestRate::implied_rate(
                comp,
                day_counter,
                self.rate_definition.compounding(),
                self.rate_definition.frequency(),
                day_counter.year_fraction(start_date, end_date),
            )?
            .rate());
        }

        // past fixing case
        if start_date < self.reference_date() && end_date <= self.reference_date() {
            return self.average_rate(start_date, end_date);
        }

        // forecast case
        if start_date >= self.reference_date() && end_date > self.reference_date() {
            self.term_structure()
                .ok_or(YieldProviderError::NoTermStructure)?
                .forward_rate(start_date, end_date, comp, freq)
        } else {
            Err(YieldProviderError::InvalidDate(format!(
                "Invalid date: {}",
                end_date
            )))?
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
        let date = Date::new(2021, 1, 1);
        let overnight_index = OvernightIndex::new(date);
        assert!(overnight_index.fixings.is_empty());
        assert!(overnight_index.term_structure.is_none());
    }

    #[test]
    fn test_with_rate_definition() {
        let date = Date::new(2021, 1, 1);
        let overnight_index =
            OvernightIndex::new(date).with_rate_definition(RateDefinition::default());
        assert_eq!(overnight_index.rate_definition, RateDefinition::default());
    }

    #[test]
    fn test_with_fixings() {
        let date = Date::new(2021, 1, 1);
        let mut fixings = HashMap::new();
        fixings.insert(Date::new(2021, 1, 1), 0.02);
        let overnight_index = OvernightIndex::new(date).with_fixings(fixings.clone());
        assert_eq!(overnight_index.fixings, fixings);
    }

    #[test]
    fn test_average_rate() {
        let date = Date::new(2021, 1, 1);
        let mut fixings = HashMap::new();
        let start_date = Date::new(2021, 1, 1);
        let end_date = Date::new(2022, 1, 1);

        fixings.insert(start_date, 100.0);
        fixings.insert(end_date, 105.0);
        let overnight_index = OvernightIndex::new(date)
            .with_fixings(fixings)
            .with_rate_definition(RateDefinition::default());

        let average_rate = overnight_index.average_rate(start_date, end_date).unwrap();

        // Add your assertions here based on how average_rate is calculated
        assert!(average_rate > 0.0);
    }

    #[test]
    fn test_fixing() {
        let date = Date::new(2021, 1, 1);
        let mut fixings = HashMap::new();
        fixings.insert(Date::new(2021, 1, 1), 0.02);
        let overnight_index = OvernightIndex::new(date).with_fixings(fixings);

        assert_eq!(overnight_index.fixing(Date::new(2021, 1, 1)), Some(0.02));
        assert_eq!(overnight_index.fixing(Date::new(2021, 1, 2)), None);
    }

    #[test]
    fn test_reference_date() {
        let date = Date::new(2021, 1, 1);
        let mut fixings = HashMap::new();
        let ref_date = Date::new(2021, 1, 1);

        fixings.insert(ref_date, 100.0);
        let overnight_index = OvernightIndex::new(date)
            .with_fixings(fixings.clone())
            .with_term_structure(Box::new(FlatForwardTermStructure::new(
                ref_date,
                InterestRate::new(
                    0.02,
                    Compounding::Simple,
                    Frequency::Annual,
                    DayCounter::Actual360,
                ),
            )));

        assert_eq!(overnight_index.reference_date(), ref_date);

        let next_date_2 = Date::new(2021, 1, 3);
        fixings.insert(next_date_2, 100.0);
        let overnight_index = OvernightIndex::new(next_date_2)
            .with_term_structure(Box::new(FlatForwardTermStructure::new(
                next_date_2,
                InterestRate::new(
                    0.02,
                    Compounding::Simple,
                    Frequency::Annual,
                    DayCounter::Actual360,
                ),
            )))
            .with_fixings(fixings);

        assert_eq!(overnight_index.reference_date(), next_date_2);
    }
}
