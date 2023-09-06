use crate::{
    alm::traits::AdvanceInTime,
    rates::yieldtermstructure::enums::YieldTermStructure,
    rates::{
        enums::Compounding,
        interestrate::RateDefinition,
        traits::{HasReferenceDate, YieldProvider},
    },
    time::{
        date::Date,
        enums::{Frequency, TimeUnit},
        period::Period,
    },
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
    reference_date: Option<Date>,
}

impl IborIndex {
    pub fn new() -> IborIndex {
        IborIndex {
            tenor: Period::empty(),
            rate_definition: RateDefinition::default(),
            fixings: HashMap::new(),
            term_structure: None,
            provider_id: None,
            reference_date: None,
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
        let max_fixing_date = *self
            .fixings
            .keys()
            .max()
            .expect("Invalid fixings for this IborIndex");
        match self.reference_date {
            Some(date) => {
                if max_fixing_date != date {
                    panic!("Invalid fixings for this IborIndex");
                }
            }
            None => {
                self.reference_date = Some(max_fixing_date);
            }
        }
        self
    }

    pub fn with_term_structure(mut self, term_structure: YieldTermStructure) -> Self {
        self.term_structure = Some(term_structure);
        let curve_ref_date = term_structure.reference_date();
        match self.reference_date {
            Some(date) => {
                if curve_ref_date != date {
                    panic!("Invalid term structure for this IborIndex");
                }
            }
            None => {
                self.reference_date = Some(curve_ref_date);
            }
        }
        self
    }

    pub fn with_provider_id(mut self, provider_id: Option<usize>) -> Self {
        self.provider_id = provider_id;
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

    fn fixings(&self) -> &HashMap<Date, f64> {
        &self.fixings
    }

    fn add_fixing(&mut self, date: Date, rate: f64) {
        if date > self.reference_date() {
            panic!("Date must be less than reference date");
        }
        self.fixings.insert(date, rate);
    }
}

impl HasReferenceDate for IborIndex {
    fn reference_date(&self) -> Date {
        self.reference_date
            .expect("No reference date for this IborIndex")
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

impl AdvanceInTime for IborIndex {
    type Output = IborIndex;
    fn advance(&self, period: Period) -> Self::Output {
        let curve = self
            .term_structure
            .expect("No term structure for this IborIndex");

        let mut fixings = self.fixings().clone();
        let mut seed = self.reference_date();
        let end_date = seed.advance(period.length(), period.units());
        while seed <= end_date {
            let rate = curve.forward_rate(
                seed,
                seed + self.tenor,
                self.rate_definition.compounding(),
                self.rate_definition.frequency(),
            );
            fixings.insert(seed, rate);
            seed = seed.advance(1, TimeUnit::Days);
        }
        IborIndex::new()
            .with_tenor(self.tenor)
            .with_rate_definition(self.rate_definition)
            .with_fixings(fixings)
            .with_term_structure(curve.advance(period))
            .with_provider_id(self.provider_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        rates::{
            interestrate::InterestRate,
            yieldtermstructure::flatforwardtermstructure::FlatForwardTermStructure,
        },
        time::{daycounter::DayCounter, enums::TimeUnit},
    };

    #[test]
    fn test_ibor_index() {
        let tenor = Period::new(1, TimeUnit::Months);
        let rate_definition = RateDefinition::new(
            DayCounter::Actual360,
            Compounding::Simple,
            Frequency::Annual,
        );
        let ibor_index = IborIndex::new()
            .with_tenor(tenor)
            .with_rate_definition(rate_definition);
        assert_eq!(ibor_index.tenor(), tenor);
        assert_eq!(
            ibor_index.rate_definition().compounding(),
            Compounding::Simple
        );
        assert_eq!(ibor_index.rate_definition().frequency(), Frequency::Annual);
        assert_eq!(
            ibor_index.rate_definition().day_counter(),
            DayCounter::Actual360
        );
    }

    #[test]
    fn test_ibor_advance() {
        let ref_date = Date::new(2021, 1, 1);
        let advance_period = Period::new(1, TimeUnit::Months);
        let tenor = Period::new(1, TimeUnit::Months);
        let rate_definition = RateDefinition::new(
            DayCounter::Actual360,
            Compounding::Simple,
            Frequency::Annual,
        );
        let ibor_index = IborIndex::new()
            .with_tenor(tenor)
            .with_rate_definition(rate_definition);
        let curve = FlatForwardTermStructure::new(
            ref_date,
            InterestRate::new(
                0.05,
                Compounding::Simple,
                Frequency::Annual,
                DayCounter::Actual360,
            ),
        );
        let ibor_index =
            ibor_index.with_term_structure(YieldTermStructure::FlatForwardTermStructure(curve));
        let ibor_index_advance = ibor_index.advance(advance_period);

        let mut seed = ref_date;
        while seed < ref_date + advance_period {
            let rate = curve.forward_rate(
                seed,
                seed + ibor_index.tenor(),
                ibor_index.rate_definition().compounding(),
                ibor_index.rate_definition().frequency(),
            );
            assert_eq!(ibor_index_advance.fixing(seed).unwrap(), rate);
            seed = seed.advance(1, TimeUnit::Days);
        }
    }
}
