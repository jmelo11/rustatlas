use std::sync::Arc;

use crate::{
    rates::{
        enums::Compounding,
        interestrate::{InterestRate, RateDefinition},
        traits::{HasReferenceDate, YieldProvider},
    },
    time::{date::Date, enums::Frequency, period::Period},
    utils::errors::{AtlasError, Result},
};

use super::traits::{AdvanceTermStructureInTime, YieldTermStructureTrait};

/// # FlatForwardTermStructure
/// Struct that defines a flat forward term structure.
/// # Example
/// ```
/// use rustatlas::prelude::*;
///
/// let reference_date = Date::new(2023, 8, 19);
/// let term_structure = FlatForwardTermStructure::new(reference_date, 0.5, RateDefinition::default());
/// assert_eq!(term_structure.reference_date(), reference_date);
/// ```
#[derive(Clone, Copy)]
pub struct FlatForwardTermStructure {
    reference_date: Date,
    rate: InterestRate,
}

impl FlatForwardTermStructure {
    pub fn new(
        reference_date: Date,
        rate: f64,
        rate_definition: RateDefinition,
    ) -> FlatForwardTermStructure {
        let rate = InterestRate::from_rate_definition(rate, rate_definition);
        FlatForwardTermStructure {
            reference_date,
            rate,
        }
    }

    pub fn rate(&self) -> InterestRate {
        return self.rate;
    }

    pub fn value(&self) -> f64 {
        self.rate.rate()
    }

    pub fn rate_definition(&self) -> RateDefinition {
        self.rate.rate_definition()
    }
}

impl HasReferenceDate for FlatForwardTermStructure {
    fn reference_date(&self) -> Date {
        return self.reference_date;
    }
}

impl YieldProvider for FlatForwardTermStructure {
    fn discount_factor(&self, date: Date) -> Result<f64> {
        if date < self.reference_date() {
            return Err(AtlasError::InvalidValueErr(format!(
                "Date {:?} is before reference date {:?}",
                date,
                self.reference_date()
            )));
        }
        return Ok(self.rate.discount_factor(self.reference_date(), date));
    }
    fn forward_rate(
        &self,
        start_date: Date,
        end_date: Date,
        comp: Compounding,
        freq: Frequency,
    ) -> Result<f64> {
        let comp_factor = self.discount_factor(start_date)? / self.discount_factor(end_date)?;
        let t = self.rate.day_counter().year_fraction(start_date, end_date);
        return Ok(InterestRate::implied_rate(
            comp_factor,
            self.rate.day_counter(),
            comp,
            freq,
            t,
        )?
        .rate());
    }
}

/// # AdvanceTermStructureInTime for FlatForwardTermStructure
impl AdvanceTermStructureInTime for FlatForwardTermStructure {
    fn advance_to_period(&self, period: Period) -> Result<Arc<dyn YieldTermStructureTrait>> {
        let new_reference_date = self
            .reference_date()
            .advance(period.length(), period.units());
        return Ok(Arc::new(FlatForwardTermStructure::new(
            new_reference_date,
            self.value(),
            self.rate_definition(),
        )));
    }

    fn advance_to_date(&self, date: Date) -> Result<Arc<dyn YieldTermStructureTrait>> {
        return Ok(Arc::new(FlatForwardTermStructure::new(
            date,
            self.value(),
            self.rate_definition(),
        )));
    }
}

impl YieldTermStructureTrait for FlatForwardTermStructure {}

#[cfg(test)]
mod tests {
    use crate::time::{daycounter::DayCounter, enums::TimeUnit};

    use super::*;

    #[test]
    fn test_reference_date() {
        let reference_date = Date::new(2023, 8, 19);

        let term_structure =
            FlatForwardTermStructure::new(reference_date, 0.5, RateDefinition::default());
        assert_eq!(term_structure.reference_date(), reference_date);
    }

    #[test]
    fn test_discount() -> Result<()> {
        let reference_date = Date::new(2023, 8, 19);
        let target_date = Date::new(2024, 8, 19);
        let interest_rate = InterestRate::from_rate_definition(0.05, RateDefinition::default());

        let term_structure =
            FlatForwardTermStructure::new(reference_date, 0.05, RateDefinition::default());

        let expected_discount = interest_rate.discount_factor(reference_date, target_date);
        let actual_discount = term_structure.discount_factor(target_date)?;

        assert_eq!(actual_discount, expected_discount);
        Ok(())
    }

    #[test]
    fn test_discount_continuous() -> Result<()> {
        let reference_date = Date::new(2023, 8, 19);
        let target_date = reference_date + Period::new(1, TimeUnit::Years);
        let rate_definition = RateDefinition::new(
            DayCounter::Actual360,
            Compounding::Continuous,
            Frequency::Semiannual,
        );
        let interest_rate = InterestRate::from_rate_definition(0.05, rate_definition);
        let term_structure = FlatForwardTermStructure::new(reference_date, 0.05, rate_definition);

        let expected_discount = interest_rate.discount_factor(reference_date, target_date);
        let actual_discount = term_structure.discount_factor(target_date)?;

        assert_eq!(actual_discount, expected_discount);
        Ok(())
    }

    #[test]
    fn test_forward_rate() -> Result<()> {
        let reference_date = Date::new(2023, 8, 19);
        let interest_rate: InterestRate = InterestRate::new(
            0.05,
            Compounding::Simple,
            Frequency::Annual,
            DayCounter::Actual360,
        );
        let start_date = Date::new(2023, 9, 19);
        let end_date = Date::new(2024, 9, 19);
        let comp = Compounding::Simple;
        let freq = Frequency::Annual;

        let term_structure =
            FlatForwardTermStructure::new(reference_date, 0.5, RateDefinition::default());

        let comp_factor = term_structure.discount_factor(start_date)?
            / term_structure.discount_factor(end_date)?;
        let t = interest_rate
            .day_counter()
            .year_fraction(start_date, end_date);

        let expected_forward_rate =
            InterestRate::implied_rate(comp_factor, interest_rate.day_counter(), comp, freq, t)?
                .rate();
        let actual_forward_rate = term_structure.forward_rate(start_date, end_date, comp, freq)?;

        assert_eq!(actual_forward_rate, expected_forward_rate);

        Ok(())
    }
}
