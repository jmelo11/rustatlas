use crate::{
    rates::{
        enums::Compounding,
        interestrate::InterestRate,
        traits::{HasReferenceDate, YieldProvider},
    },
    time::{date::Date, daycounters::traits::*, enums::Frequency},
};

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
    fn discount_factor(&self, date: Date) -> f64 {
        if date < self.reference_date() {
            panic!("date must be greater than reference date");
        }
        return self.rate.discount_factor(self.reference_date(), date);
    }
    fn forward_rate(
        &self,
        start_date: Date,
        end_date: Date,
        comp: Compounding,
        freq: Frequency,
    ) -> f64 {
        let comp_factor = self.discount_factor(start_date) / self.discount_factor(end_date);
        let t = self.rate.day_counter().year_fraction(start_date, end_date);
        return InterestRate::implied_rate(comp_factor, self.rate.day_counter(), comp, freq, t)
            .rate();
    }
}

#[cfg(test)]
mod tests {
    use crate::time::daycounter::DayCounter;
    use crate::time::daycounters::traits::DayCountProvider;

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
    fn test_discount() {
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
        let actual_discount = term_structure.discount_factor(target_date);

        assert_eq!(actual_discount, expected_discount);
    }

    #[test]
    fn test_forward_rate() {
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

        let comp_factor =
            term_structure.discount_factor(start_date) / term_structure.discount_factor(end_date);
        let t = interest_rate
            .day_counter()
            .year_fraction(start_date, end_date);

        let expected_forward_rate =
            InterestRate::implied_rate(comp_factor, interest_rate.day_counter(), comp, freq, t)
                .rate();
        let actual_forward_rate = term_structure.forward_rate(start_date, end_date, comp, freq);

        assert_eq!(actual_forward_rate, expected_forward_rate);
    }
}
