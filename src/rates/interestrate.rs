use crate::rates::enums::Compounding;
use crate::time::date::Date;
use crate::time::daycounter::DayCounter;
use crate::time::daycounters::traits::DayCountProvider;
use crate::time::enums::Frequency;

/// # RateDefinition
/// Struct that defines a rate.
/// # Example
/// ```
/// use rustatlas::prelude::*
/// let rate_definition = RateDefinition::new(Compounding::Simple, Frequency::Annual, DayCounter::Actual360);
/// assert_eq!(rate_definition.compounding(), Compounding::Simple);
/// assert_eq!(rate_definition.frequency(), Frequency::Annual);
/// assert_eq!(rate_definition.day_counter(), DayCounter::Actual360);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RateDefinition {
    compounding: Compounding,
    frequency: Frequency,
    day_counter: DayCounter,
}

impl RateDefinition {
    pub fn new(
        compounding: Compounding,
        frequency: Frequency,
        day_counter: DayCounter,
    ) -> RateDefinition {
        RateDefinition {
            compounding,
            frequency,
            day_counter,
        }
    }

    pub fn default() -> RateDefinition {
        return RateDefinition::new(
            Compounding::Simple,
            Frequency::Annual,
            DayCounter::Actual360,
        );
    }

    pub fn compounding(&self) -> Compounding {
        return self.compounding;
    }

    pub fn frequency(&self) -> Frequency {
        return self.frequency;
    }

    pub fn day_counter(&self) -> DayCounter {
        return self.day_counter;
    }
}

/// # InterestRate
/// Struct that defines an interest rate.
/// # Example
/// ```
/// use rustatlas::prelude::*;
/// let rate = InterestRate::new(0.05, Compounding::Simple, Frequency::Annual, DayCounter::Actual360);
/// assert_eq!(rate.rate(), 0.05);
/// assert_eq!(rate.compounding(), Compounding::Simple);
/// assert_eq!(rate.frequency(), Frequency::Annual);
/// assert_eq!(rate.day_counter(), DayCounter::Actual360);
/// ```
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct InterestRate {
    rate: f64,
    rate_definition: RateDefinition,
}

impl InterestRate {
    pub fn new(
        rate: f64,
        compounding: Compounding,
        frequency: Frequency,
        day_counter: DayCounter,
    ) -> InterestRate {
        InterestRate {
            rate,
            rate_definition: RateDefinition::new(compounding, frequency, day_counter),
        }
    }

    pub fn from_rate_definition(rate: f64, rate_definition: RateDefinition) -> InterestRate {
        InterestRate {
            rate,
            rate_definition,
        }
    }

    pub fn rate(&self) -> f64 {
        return self.rate;
    }

    pub fn rate_definition(&self) -> &RateDefinition {
        return &self.rate_definition;
    }

    pub fn compounding(&self) -> Compounding {
        return self.rate_definition.compounding();
    }

    pub fn frequency(&self) -> Frequency {
        return self.rate_definition.frequency();
    }

    pub fn day_counter(&self) -> DayCounter {
        return self.rate_definition.day_counter();
    }

    pub fn implied_rate(
        compound: f64,
        result_dc: DayCounter,
        comp: Compounding,
        freq: Frequency,
        t: f64,
    ) -> InterestRate {
        assert!(compound > 0.0, "positive compound factor required");
        let r: f64;
        let f = freq as i64 as f64;
        if compound == 1.0 {
            assert!(t >= 0.0, "non-negative time required");
            r = 0.0;
        } else {
            assert!(t > 0.0, "positive time required");
            match comp {
                Compounding::Simple => r = (compound - 1.0) / t,
                Compounding::Compounded => r = (compound.powf(1.0 / (f * t)) - 1.0) * f,
                Compounding::Continuous => r = (compound).ln() / t,
                Compounding::SimpleThenCompounded => {
                    if t <= 1.0 / f {
                        r = (compound - 1.0) / t
                    } else {
                        r = (compound.powf(1.0 / (f * t)) - 1.0) * f
                    }
                }
                Compounding::CompoundedThenSimple => {
                    if t > 1.0 / f {
                        r = (compound - 1.0) / t
                    } else {
                        r = (compound.powf(1.0 / (f * t)) - 1.0) * f
                    }
                }
            }
        }
        return InterestRate::new(r, comp, freq, result_dc);
    }

    pub fn compound_factor(&self, start: Date, end: Date) -> f64 {
        let rate = self.rate();
        let compounding = self.compounding();
        let day_counter = self.day_counter();
        let year_fraction = day_counter.year_fraction(start, end);
        match compounding {
            Compounding::Simple => 1.0 + rate * year_fraction,
            Compounding::Compounded => (1.0 + rate).powf(year_fraction),
            Compounding::Continuous => (1.0 + rate).exp() * year_fraction,
            Compounding::SimpleThenCompounded => {
                if year_fraction <= 1.0 {
                    1.0 + rate * year_fraction
                } else {
                    (1.0 + rate).powf(year_fraction)
                }
            }
            Compounding::CompoundedThenSimple => {
                if year_fraction <= 1.0 {
                    (1.0 + rate).powf(year_fraction)
                } else {
                    1.0 + rate * year_fraction
                }
            }
        }
    }

    pub fn discount_factor(&self, start: Date, end: Date) -> f64 {
        return 1.0 / self.compound_factor(start, end);
    }

    pub fn forward_rate(
        &self,
        start_date: Date,
        end_date: Date,
        comp: Compounding,
        freq: Frequency,
    ) -> f64 {
        return self.rate;
    }
}

/// # YieldProvider for InterestRate
/// Implement YieldProvider for InterestRate.
/// # Example
/// ```
/// use rustatlas::prelude::*;
/// let start_date = Date::from_ymd(2020, 1, 1);
/// let end_date = Date::from_ymd(2020, 2, 1);
/// let day_count = DayCounter::Actual360;
/// let compounding = Compounding::Simple;
/// let frequency = Frequency::Annual;
/// let rate = InterestRate::new(0.05, compounding, frequency, day_count);
/// assert_eq!(rate.compound_factor(start_date, end_date), 1.0043055555555556);
/// assert_eq!(rate.discount_factor(start_date, end_date), 0.9957129027796985);
/// assert_eq!(rate.forward_rate(start_date, end_date, compounding, frequency), 0.05);
/// ```

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn test_rate_definition_new() {
        let rd = RateDefinition::new(
            Compounding::Simple,
            Frequency::Annual,
            DayCounter::Actual360,
        );
        assert_eq!(rd.compounding(), Compounding::Simple);
        assert_eq!(rd.frequency(), Frequency::Annual);
        assert_eq!(rd.day_counter(), DayCounter::Actual360);
    }

    #[test]
    fn test_rate_definition_common_definition() {
        let rd = RateDefinition::default();
        assert_eq!(rd.compounding(), Compounding::Simple);
        assert_eq!(rd.frequency(), Frequency::Annual);
        assert_eq!(rd.day_counter(), DayCounter::Actual360);
    }

    #[test]
    fn test_interest_rate_new() {
        let ir = InterestRate::new(
            0.05,
            Compounding::Simple,
            Frequency::Annual,
            DayCounter::Actual360,
        );
        assert_eq!(ir.rate(), 0.05);
        assert_eq!(ir.compounding(), Compounding::Simple);
        assert_eq!(ir.frequency(), Frequency::Annual);
        assert_eq!(ir.day_counter(), DayCounter::Actual360);
    }

    #[test]
    fn test_interest_rate_from_rate_definition() {
        let rd = RateDefinition::new(
            Compounding::Simple,
            Frequency::Annual,
            DayCounter::Actual360,
        );
        let ir = InterestRate::from_rate_definition(0.05, rd);
        assert_eq!(ir.rate(), 0.05);
        assert_eq!(ir.compounding(), Compounding::Simple);
        assert_eq!(ir.frequency(), Frequency::Annual);
        assert_eq!(ir.day_counter(), DayCounter::Actual360);
    }

    const EPSILON: f64 = 1e-9; // or any other small value that you choose

    #[test]
    fn test_implied_rate() {
        // Choose parameters that make sense for your implied_rate function
        // For example:
        let ir = InterestRate::implied_rate(
            1.05,
            DayCounter::Actual360,
            Compounding::Simple,
            Frequency::Annual,
            1.0,
        );
        let expected_rate = 0.05;
        assert!((ir.rate() - expected_rate).abs() < EPSILON);
    }

    #[test]
    #[should_panic(expected = "positive compound factor required")]
    fn test_implied_rate_panic() {
        InterestRate::implied_rate(
            0.0,
            DayCounter::Actual360,
            Compounding::Simple,
            Frequency::Annual,
            1.0,
        );
    }
}
