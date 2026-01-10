use serde::{Deserialize, Serialize};

use crate::{
    time::{date::Date, daycounter::DayCounter, enums::Frequency},
    utils::errors::{AtlasError, Result},
};

use super::enums::Compounding;

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
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RateDefinition {
    day_counter: DayCounter,
    compounding: Compounding,
    frequency: Frequency,
}

impl RateDefinition {
    /// Creates a new `RateDefinition` with the specified day counter, compounding, and frequency.
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

    /// Returns the compounding method of this rate definition.
    pub fn compounding(&self) -> Compounding {
        self.compounding
    }

    /// Returns the frequency of this rate definition.
    pub fn frequency(&self) -> Frequency {
        self.frequency
    }

    /// Returns the day counter of this rate definition.
    pub fn day_counter(&self) -> DayCounter {
        self.day_counter
    }
}

impl Default for RateDefinition {
    fn default() -> Self {
        RateDefinition::new(
            DayCounter::Actual360,
            Compounding::Simple,
            Frequency::Annual,
        )
    }
}

/// # InterestRate
/// Struct that defines an interest rate.
///
/// ## Example
/// ```
/// use rustatlas::prelude::*;
/// let rate = InterestRate::new(0.05, Compounding::Simple, Frequency::Annual, DayCounter::Actual360);
/// assert_eq!(rate.rate(), 0.05);
/// assert_eq!(rate.compounding(), Compounding::Simple);
/// assert_eq!(rate.frequency(), Frequency::Annual);
/// assert_eq!(rate.day_counter(), DayCounter::Actual360);
/// ```
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq)]
pub struct InterestRate {
    rate: f64,
    rate_definition: RateDefinition,
}

impl InterestRate {
    /// Creates a new `InterestRate` with the specified rate value and rate definition parameters.
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

    /// Creates a new `InterestRate` from a rate value and a `RateDefinition`.
    pub fn from_rate_definition(rate: f64, rate_definition: RateDefinition) -> InterestRate {
        InterestRate {
            rate,
            rate_definition,
        }
    }

    /// Returns the rate value of this interest rate.
    pub fn rate(&self) -> f64 {
        self.rate
    }

    /// Returns the rate definition of this interest rate.
    pub fn rate_definition(&self) -> RateDefinition {
        self.rate_definition
    }

    /// Returns the compounding method of this interest rate.
    pub fn compounding(&self) -> Compounding {
        self.rate_definition.compounding()
    }

    /// Returns the frequency of this interest rate.
    pub fn frequency(&self) -> Frequency {
        self.rate_definition.frequency()
    }

    /// Returns the day counter of this interest rate.
    pub fn day_counter(&self) -> DayCounter {
        self.rate_definition.day_counter()
    }

    /// Calculates the implied interest rate from a compound factor.
    pub fn implied_rate(
        compound: f64,
        result_dc: DayCounter,
        comp: Compounding,
        freq: Frequency,
        t: f64,
    ) -> Result<InterestRate> {
        if compound <= 0.0 {
            return Err(AtlasError::InvalidValueErr(
                "Positive compound factor required".to_string(),
            ));
        }
        let r: f64;
        let f = f64::from(freq as i32);
        if compound == 1.0 {
            if t < 0.0 {
                return Err(AtlasError::InvalidValueErr(
                    "Non-negative time required".to_string(),
                ));
            }
            r = 0.0;
        } else {
            if t <= 0.0 {
                return Err(AtlasError::InvalidValueErr(
                    "Positive time required".to_string(),
                ));
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
        Ok(InterestRate::new(r, comp, freq, result_dc))
    }

    /// Calculates the compound factor between two dates using the day counter.
    pub fn compound_factor(&self, start: Date, end: Date) -> f64 {
        let day_counter = self.day_counter();
        let year_fraction = day_counter.year_fraction(start, end);
        self.compound_factor_from_yf(year_fraction)
    }

    /// Calculates the compound factor from a year fraction.
    pub fn compound_factor_from_yf(&self, year_fraction: f64) -> f64 {
        let rate = self.rate();
        let compounding = self.compounding();
        let f = f64::from(self.frequency() as i32);
        match compounding {
            Compounding::Simple => 1.0 + rate * year_fraction,
            Compounding::Compounded => (1.0 + rate / f).powf(f * year_fraction),
            Compounding::Continuous => (rate * year_fraction).exp(),
            Compounding::SimpleThenCompounded => {
                if year_fraction <= 1.0 / f {
                    1.0 + rate * year_fraction
                } else {
                    (1.0 + rate / f).powf(year_fraction * f)
                }
            }
            Compounding::CompoundedThenSimple => {
                if year_fraction > 1.0 / f {
                    1.0 + rate * year_fraction
                } else {
                    (1.0 + rate / f).powf(year_fraction * f)
                }
            }
        }
    }

    /// Calculates the discount factor between two dates.
    pub fn discount_factor(&self, start: Date, end: Date) -> f64 {
        1.0 / self.compound_factor(start, end)
    }

    /// Calculates the forward rate between two dates with specified compounding and frequency.
    pub fn forward_rate(
        &self,
        _start_date: Date,
        _end_date: Date,
        _comp: Compounding,
        _freq: Frequency,
    ) -> f64 {
        self.rate
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

    struct InterestRateData {
        rate: f64,
        compounding: Compounding,
        frequency: Frequency,
        time: f64,
        compounding2: Compounding,
        frequency2: Frequency,
        rate2: f64,
        precision: i64,
    }
    fn test_cases() -> Vec<InterestRateData> {
        let test_cases = vec![
            InterestRateData {
                rate: 0.0800,
                compounding: Compounding::Compounded,
                frequency: Frequency::Quarterly,
                time: 1.00,
                compounding2: Compounding::Continuous,
                frequency2: Frequency::Annual,
                rate2: 0.0792,
                precision: 4,
            },
            InterestRateData {
                rate: 0.1200,
                compounding: Compounding::Continuous,
                frequency: Frequency::Annual,
                time: 1.00,
                compounding2: Compounding::Compounded,
                frequency2: Frequency::Annual,
                rate2: 0.1275,
                precision: 4,
            },
            InterestRateData {
                rate: 0.0800,
                compounding: Compounding::Compounded,
                frequency: Frequency::Quarterly,
                time: 1.00,
                compounding2: Compounding::Compounded,
                frequency2: Frequency::Annual,
                rate2: 0.0824,
                precision: 4,
            },
            InterestRateData {
                rate: 0.0700,
                compounding: Compounding::Compounded,
                frequency: Frequency::Quarterly,
                time: 1.00,
                compounding2: Compounding::Compounded,
                frequency2: Frequency::Semiannual,
                rate2: 0.0706,
                precision: 4,
            },
            InterestRateData {
                rate: 0.0100,
                compounding: Compounding::Compounded,
                frequency: Frequency::Annual,
                time: 1.00,
                compounding2: Compounding::Simple,
                frequency2: Frequency::Annual,
                rate2: 0.0100,
                precision: 4,
            },
            InterestRateData {
                rate: 0.0200,
                compounding: Compounding::Simple,
                frequency: Frequency::Annual,
                time: 1.00,
                compounding2: Compounding::Compounded,
                frequency2: Frequency::Annual,
                rate2: 0.0200,
                precision: 4,
            },
            InterestRateData {
                rate: 0.0300,
                compounding: Compounding::Compounded,
                frequency: Frequency::Semiannual,
                time: 0.50,
                compounding2: Compounding::Simple,
                frequency2: Frequency::Annual,
                rate2: 0.0300,
                precision: 4,
            },
            InterestRateData {
                rate: 0.0400,
                compounding: Compounding::Simple,
                frequency: Frequency::Annual,
                time: 0.50,
                compounding2: Compounding::Compounded,
                frequency2: Frequency::Semiannual,
                rate2: 0.0400,
                precision: 4,
            },
            InterestRateData {
                rate: 0.0500,
                compounding: Compounding::Compounded,
                frequency: Frequency::EveryFourthMonth,
                time: 1.0 / 3.0,
                compounding2: Compounding::Simple,
                frequency2: Frequency::Annual,
                rate2: 0.0500,
                precision: 4,
            },
            InterestRateData {
                rate: 0.0600,
                compounding: Compounding::Simple,
                frequency: Frequency::Annual,
                time: 1.0 / 3.0,
                compounding2: Compounding::Compounded,
                frequency2: Frequency::EveryFourthMonth,
                rate2: 0.0600,
                precision: 4,
            },
            InterestRateData {
                rate: 0.0500,
                compounding: Compounding::Compounded,
                frequency: Frequency::Quarterly,
                time: 0.25,
                compounding2: Compounding::Simple,
                frequency2: Frequency::Annual,
                rate2: 0.0500,
                precision: 4,
            },
            InterestRateData {
                rate: 0.0600,
                compounding: Compounding::Simple,
                frequency: Frequency::Annual,
                time: 0.25,
                compounding2: Compounding::Compounded,
                frequency2: Frequency::Quarterly,
                rate2: 0.0600,
                precision: 4,
            },
            InterestRateData {
                rate: 0.0700,
                compounding: Compounding::Compounded,
                frequency: Frequency::Bimonthly,
                time: 1.0 / 6.0,
                compounding2: Compounding::Simple,
                frequency2: Frequency::Annual,
                rate2: 0.0700,
                precision: 4,
            },
            InterestRateData {
                rate: 0.0800,
                compounding: Compounding::Simple,
                frequency: Frequency::Annual,
                time: 1.0 / 6.0,
                compounding2: Compounding::Compounded,
                frequency2: Frequency::Bimonthly,
                rate2: 0.0800,
                precision: 4,
            },
            InterestRateData {
                rate: 0.0900,
                compounding: Compounding::Compounded,
                frequency: Frequency::Monthly,
                time: 1.0 / 12.0,
                compounding2: Compounding::Simple,
                frequency2: Frequency::Annual,
                rate2: 0.0900,
                precision: 4,
            },
            InterestRateData {
                rate: 0.1000,
                compounding: Compounding::Simple,
                frequency: Frequency::Annual,
                time: 1.0 / 12.0,
                compounding2: Compounding::Compounded,
                frequency2: Frequency::Monthly,
                rate2: 0.1000,
                precision: 4,
            },
            InterestRateData {
                rate: 0.0300,
                compounding: Compounding::SimpleThenCompounded,
                frequency: Frequency::Semiannual,
                time: 0.25,
                compounding2: Compounding::Simple,
                frequency2: Frequency::Annual,
                rate2: 0.0300,
                precision: 4,
            },
            InterestRateData {
                rate: 0.0300,
                compounding: Compounding::SimpleThenCompounded,
                frequency: Frequency::Semiannual,
                time: 0.25,
                compounding2: Compounding::Simple,
                frequency2: Frequency::Semiannual,
                rate2: 0.0300,
                precision: 4,
            },
            InterestRateData {
                rate: 0.0300,
                compounding: Compounding::SimpleThenCompounded,
                frequency: Frequency::Semiannual,
                time: 0.25,
                compounding2: Compounding::Simple,
                frequency2: Frequency::Quarterly,
                rate2: 0.0300,
                precision: 4,
            },
            InterestRateData {
                rate: 0.0300,
                compounding: Compounding::SimpleThenCompounded,
                frequency: Frequency::Semiannual,
                time: 0.50,
                compounding2: Compounding::Simple,
                frequency2: Frequency::Annual,
                rate2: 0.0300,
                precision: 4,
            },
            InterestRateData {
                rate: 0.0300,
                compounding: Compounding::SimpleThenCompounded,
                frequency: Frequency::Semiannual,
                time: 0.50,
                compounding2: Compounding::Simple,
                frequency2: Frequency::Semiannual,
                rate2: 0.0300,
                precision: 4,
            },
            InterestRateData {
                rate: 0.0300,
                compounding: Compounding::SimpleThenCompounded,
                frequency: Frequency::Semiannual,
                time: 0.75,
                compounding2: Compounding::Compounded,
                frequency2: Frequency::Semiannual,
                rate2: 0.0300,
                precision: 4,
            },
            InterestRateData {
                rate: 0.0400,
                compounding: Compounding::Simple,
                frequency: Frequency::Semiannual,
                time: 0.25,
                compounding2: Compounding::SimpleThenCompounded,
                frequency2: Frequency::Quarterly,
                rate2: 0.0400,
                precision: 4,
            },
            InterestRateData {
                rate: 0.0400,
                compounding: Compounding::Simple,
                frequency: Frequency::Semiannual,
                time: 0.25,
                compounding2: Compounding::SimpleThenCompounded,
                frequency2: Frequency::Semiannual,
                rate2: 0.0400,
                precision: 4,
            },
            InterestRateData {
                rate: 0.0400,
                compounding: Compounding::Simple,
                frequency: Frequency::Semiannual,
                time: 0.25,
                compounding2: Compounding::SimpleThenCompounded,
                frequency2: Frequency::Annual,
                rate2: 0.0400,
                precision: 4,
            },
            InterestRateData {
                rate: 0.0400,
                compounding: Compounding::Compounded,
                frequency: Frequency::Quarterly,
                time: 0.50,
                compounding2: Compounding::SimpleThenCompounded,
                frequency2: Frequency::Quarterly,
                rate2: 0.0400,
                precision: 4,
            },
            InterestRateData {
                rate: 0.0400,
                compounding: Compounding::Simple,
                frequency: Frequency::Semiannual,
                time: 0.50,
                compounding2: Compounding::SimpleThenCompounded,
                frequency2: Frequency::Semiannual,
                rate2: 0.0400,
                precision: 4,
            },
            InterestRateData {
                rate: 0.0400,
                compounding: Compounding::Simple,
                frequency: Frequency::Semiannual,
                time: 0.50,
                compounding2: Compounding::SimpleThenCompounded,
                frequency2: Frequency::Annual,
                rate2: 0.0400,
                precision: 4,
            },
            InterestRateData {
                rate: 0.0400,
                compounding: Compounding::Compounded,
                frequency: Frequency::Quarterly,
                time: 0.75,
                compounding2: Compounding::SimpleThenCompounded,
                frequency2: Frequency::Quarterly,
                rate2: 0.0400,
                precision: 4,
            },
            InterestRateData {
                rate: 0.0400,
                compounding: Compounding::Compounded,
                frequency: Frequency::Semiannual,
                time: 0.75,
                compounding2: Compounding::SimpleThenCompounded,
                frequency2: Frequency::Semiannual,
                rate2: 0.0400,
                precision: 4,
            },
        ];
        test_cases
    }
    #[test]
    fn test_all_cases() {
        let test_cases = test_cases();
        for test_case in test_cases {
            let rate = InterestRate::new(
                test_case.rate,
                test_case.compounding,
                test_case.frequency,
                DayCounter::Actual360,
            );
            let implied_rate = InterestRate::implied_rate(
                rate.compound_factor_from_yf(test_case.time),
                DayCounter::Actual360,
                test_case.compounding2,
                test_case.frequency2,
                test_case.time,
            )
            .unwrap_or_else(|e| {
                panic!(
                    "implied_rate should succeed in test_implied_rate_for_compounding_and_frequency: {e}"
                )
            });
            assert!(
                (implied_rate.rate() - test_case.rate2).abs()
                    < (test_case.precision as f64) / 100.0
            );
        }
    }

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
