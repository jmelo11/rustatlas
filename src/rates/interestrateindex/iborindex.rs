use crate::{
    rates::{
        enums::Compounding,
        interestrate::RateDefinition,
        traits::{HasReferenceDate, YieldProvider, YieldProviderError},
        yieldtermstructure::traits::YieldTermStructureTrait,
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
/// let ref_date = Date::new(2021, 1, 1);
/// let tenor = Period::new(1, TimeUnit::Months);
/// let rate_definition = RateDefinition::new(DayCounter::Actual360, Compounding::Simple, Frequency::Annual);
/// let ibor_index = IborIndex::new(ref_date).with_tenor(tenor).with_rate_definition(rate_definition);
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
    term_structure: Option<Box<dyn YieldTermStructureTrait>>,
    provider_id: Option<usize>,
    reference_date: Date,
}

impl IborIndex {
    pub fn new(reference_date: Date) -> IborIndex {
        IborIndex {
            reference_date: reference_date,
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

    pub fn term_structure(&self) -> Option<&dyn YieldTermStructureTrait> {
        self.term_structure.as_deref()
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

    pub fn with_term_structure(mut self, term_structure: Box<dyn YieldTermStructureTrait>) -> Self {
        self.term_structure = Some(term_structure);
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
    }
}

impl YieldProvider for IborIndex {
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
        if end_date < start_date {
            return Err(YieldProviderError::InvalidDate(format!(
                "End date {} is before start date {}",
                end_date, start_date
            )));
        }
        if start_date < self.reference_date() {
            self.fixing(start_date)
                .ok_or(YieldProviderError::NoFixingRate(start_date))
        } else {
            self.term_structure()
                .ok_or(YieldProviderError::NoTermStructure)?
                .forward_rate(start_date, end_date, comp, freq)
        }
    }
}

// impl InterestRateIndexTrait for IborIndex {}

// impl AdvanceInTime for IborIndex {
//     type Output = IborIndex;
//     fn advance_to_period(&self, period: Period) -> Result<Self::Output, E> {
//     {
//         let curve = self
//             .term_structure().ok_or(YieldProviderError::NoTermStructure)?;

//         let mut fixings = self.fixings().clone();
//         let mut seed = self.reference_date();
//         let end_date = seed.advance(period.length(), period.units());
//         while seed <= end_date {
//             let rate = curve.forward_rate(
//                 seed,
//                 seed + self.tenor,
//                 self.rate_definition.compounding(),
//                 self.rate_definition.frequency(),
//             );
//             fixings.insert(seed, rate);
//             seed = seed.advance(1, TimeUnit::Days);
//         }
//         Ok(IborIndex::new()
//                     .with_tenor(self.tenor)
//                     .with_rate_definition(self.rate_definition)
//                     .with_fixings(fixings)
//                     .with_term_structure(curve.advance(period))
//                     .with_provider_id(self.provider_id))
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;
    use crate::time::{daycounter::DayCounter, enums::TimeUnit};

    #[test]
    fn test_ibor_index() {
        let ref_date = Date::new(2021, 1, 1);
        let tenor = Period::new(1, TimeUnit::Months);
        let rate_definition = RateDefinition::new(
            DayCounter::Actual360,
            Compounding::Simple,
            Frequency::Annual,
        );
        let ibor_index = IborIndex::new(ref_date)
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

    // #[test]
    // fn test_ibor_advance() {
    //     let ref_date = Date::new(2021, 1, 1);
    //     let advance_period = Period::new(1, TimeUnit::Months);
    //     let tenor = Period::new(1, TimeUnit::Months);
    //     let rate_definition = RateDefinition::new(
    //         DayCounter::Actual360,
    //         Compounding::Simple,
    //         Frequency::Annual,
    //     );
    //     let ibor_index = IborIndex::new()
    //         .with_tenor(tenor)
    //         .with_rate_definition(rate_definition);
    //     let curve = FlatForwardTermStructure::new(
    //         ref_date,
    //         InterestRate::new(
    //             0.05,
    //             Compounding::Simple,
    //             Frequency::Annual,
    //             DayCounter::Actual360,
    //         ),
    //     );
    //     let ibor_index =
    //         ibor_index.with_term_structure(YieldTermStructure::FlatForwardTermStructure(curve));
    //     let ibor_index_advance = ibor_index.advance(advance_period);

    //     let mut seed = ref_date;
    //     while seed < ref_date + advance_period {
    //         let rate = curve.forward_rate(
    //             seed,
    //             seed + ibor_index.tenor(),
    //             ibor_index.rate_definition().compounding(),
    //             ibor_index.rate_definition().frequency(),
    //         );
    //         assert_eq!(ibor_index_advance.fixing(seed).unwrap(), rate);
    //         seed = seed.advance(1, TimeUnit::Days);
    //     }
    // }
}
