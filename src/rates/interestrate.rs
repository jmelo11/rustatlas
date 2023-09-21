use thiserror::Error;

use crate::rates::enums::Compounding;
use crate::time::date::Date;
use crate::time::daycounter::DayCounter;
use crate::time::enums::Frequency;

/// # RateDefinition
/// Struct that defines a rate.
/// # Example
/// ```
/// use rustatlas::prelude::*;
/// let rate_definition = RateDefinition::new(DayCounter::Actual360, Compounding::Simple, Frequency::Annual);
/// assert_eq!(rate_definition.compounding(), Compounding::Simple);
/// assert_eq!(rate_definition.frequency(), Frequency::Annual);
/// assert_eq!(rate_definition.day_counter(), DayCounter::Actual360);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RateDefinition {
    day_counter: DayCounter,
    compounding: Compounding,
    frequency: Frequency,
}

impl RateDefinition {
    pub fn new(
        day_counter: DayCounter,
        compounding: Compounding,
        frequency: Frequency,
    ) -> RateDefinition {
        RateDefinition {
            day_counter,
            compounding,
            frequency,
        }
    }

    pub fn default() -> RateDefinition {
        return RateDefinition::new(
            DayCounter::Actual360,
            Compounding::Simple,
            Frequency::Annual,
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

#[derive(Error, Debug)]
pub enum InterestRateError {
    #[error("negative compound factor not allowed")]
    NegativeCompoundFactor,
    #[error("non-negative time required")]
    NonNegativeTime,
    #[error("positive time required")]
    PositiveTime,
    #[error("positive compound factor required")]
    PositiveCompoundFactor,
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
            rate_definition: RateDefinition::new(day_counter, compounding, frequency),
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
    ) -> Result<InterestRate, InterestRateError> {
        //assert!(compound > 0.0, "positive compound factor required");
        if compound <= 0.0 {
            return Err(InterestRateError::PositiveCompoundFactor);
        }
        let r: f64;
        let f = freq as i64 as f64;
        if compound == 1.0 {
            //assert!(t >= 0.0, "non-negative time required");
            if t < 0.0 {
                return Err(InterestRateError::NonNegativeTime);
            }
            r = 0.0;
        } else {
            //assert!(t > 0.0, "positive time required");
            if t <= 0.0 {
                return Err(InterestRateError::PositiveTime);
            }
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
        return Ok(InterestRate::new(r, comp, freq, result_dc));
    }

    pub fn compound_factor(&self, start: Date, end: Date) -> f64 {
        let day_counter = self.day_counter();
        let year_fraction = day_counter.year_fraction(start, end);
        return  self.compound_factor_from_yf(year_fraction)
    }

    pub fn compound_factor_from_yf(&self, year_fraction: f64) -> f64 {
        let rate = self.rate();
        let compounding = self.compounding();

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
        _start_date: Date,
        _end_date: Date,
        _comp: Compounding,
        _freq: Frequency,
    ) -> f64 {
        return self.rate;
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        rates::{
            enums::Compounding,
            interestrate::{InterestRate, RateDefinition},
        },
        time::{daycounter::DayCounter, enums::Frequency},
    };

    #[test]
    fn test_rate_definition_new() {
        let rd = RateDefinition::new(
            DayCounter::Actual360,
            Compounding::Simple,
            Frequency::Annual,
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
            DayCounter::Actual360,
            Compounding::Simple,
            Frequency::Annual,
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
        )
        .unwrap();
        let expected_rate = 0.05;
        assert!((ir.rate() - expected_rate).abs() < EPSILON);
    }

    #[test]
    fn test_implied_rate_panic() {
        let err = InterestRate::implied_rate(
            0.0,
            DayCounter::Actual360,
            Compounding::Simple,
            Frequency::Annual,
            1.0,
        );
        assert!(err.is_err());
    }
}
