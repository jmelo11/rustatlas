use crate::{
    rates::{
        enums::Compounding,
        interestrate::InterestRate,
        traits::{HasReferenceDate, YieldProvider, YieldProviderError},
    },
    time::{date::Date, enums::Frequency, period::Period},
};

use super::traits::{AdvanceInTimeError, AdvanceTermStructureInTime, YieldTermStructureTrait};

/// # FlatForwardTermStructure
/// Struct that defines a flat forward term structure.
/// # Example
/// ```
/// use rustatlas::prelude::*;
///
/// let reference_date = Date::new(2023, 8, 19);
/// let interest_rate: InterestRate = InterestRate::new(0.05, Compounding::Simple, Frequency::Annual, DayCounter::Actual360);
/// let term_structure = FlatForwardTermStructure::new(reference_date, interest_rate);
/// assert_eq!(term_structure.reference_date(), reference_date);
/// ```
#[derive(Clone, Copy)]
pub struct FlatForwardTermStructure {
    reference_date: Date,
    rate: InterestRate,
}

impl FlatForwardTermStructure {
    pub fn new(reference_date: Date, rate: InterestRate) -> FlatForwardTermStructure {
        FlatForwardTermStructure {
            reference_date,
            rate,
        }
    }

    pub fn rate(&self) -> InterestRate {
        return self.rate;
    }
}

impl HasReferenceDate for FlatForwardTermStructure {
    fn reference_date(&self) -> Date {
        return self.reference_date;
    }
}

impl YieldProvider for FlatForwardTermStructure {
    fn discount_factor(&self, date: Date) -> Result<f64, YieldProviderError> {
        if date < self.reference_date() {
            Err(YieldProviderError::InvalidDate(format!(
                "Invalid date: {}",
                date
            )))?;
        }
        return Ok(self.rate.discount_factor(self.reference_date(), date));
    }
    fn forward_rate(
        &self,
        start_date: Date,
        end_date: Date,
        comp: Compounding,
        freq: Frequency,
    ) -> Result<f64, YieldProviderError> {
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
    fn advance_to_period(
        &self,
        period: Period,
    ) -> Result<Box<dyn YieldTermStructureTrait>, AdvanceInTimeError> {
        let new_reference_date = self
            .reference_date()
            .advance(period.length(), period.units());
        return Ok(Box::new(FlatForwardTermStructure::new(
            new_reference_date,
            self.rate(),
        )));
    }

    fn advance_to_date(
        &self,
        date: Date,
    ) -> Result<Box<dyn YieldTermStructureTrait>, AdvanceInTimeError> {
        return Ok(Box::new(FlatForwardTermStructure::new(date, self.rate())));
    }
}

impl YieldTermStructureTrait for FlatForwardTermStructure {}

#[cfg(test)]
mod tests {
    use crate::time::daycounter::DayCounter;

    use super::*;

    #[test]
    fn test_reference_date() {
        let reference_date = Date::new(2023, 8, 19);
        let interest_rate: InterestRate = InterestRate::new(
            0.05,
            Compounding::Simple,
            Frequency::Annual,
            DayCounter::Actual360,
        );

        let term_structure = FlatForwardTermStructure::new(reference_date, interest_rate);
        assert_eq!(term_structure.reference_date(), reference_date);
    }

    #[test]
    fn test_discount() -> Result<(), YieldProviderError> {
        let reference_date = Date::new(2023, 8, 19);
        let interest_rate: InterestRate = InterestRate::new(
            0.05,
            Compounding::Simple,
            Frequency::Annual,
            DayCounter::Actual360,
        );
        let target_date = Date::new(2024, 8, 19);

        let term_structure = FlatForwardTermStructure::new(reference_date, interest_rate);

        let expected_discount = interest_rate.discount_factor(reference_date, target_date);
        let actual_discount = term_structure.discount_factor(target_date)?;

        assert_eq!(actual_discount, expected_discount);
        Ok(())
    }

    #[test]
    fn test_forward_rate() -> Result<(), YieldProviderError> {
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

        let term_structure = FlatForwardTermStructure::new(reference_date, interest_rate);

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
