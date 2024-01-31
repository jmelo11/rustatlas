use super::enums::{Frequency, TimeUnit};
use serde::de;
use serde::{de::Visitor, Deserializer, Serialize};
use std::fmt;
use std::{
    cmp::Ordering,
    ops::{Add, Mul, MulAssign, Neg, Sub},
};

use crate::utils::errors::{AtlasError, Result};

/// # Period
/// Struct representing a financial period.
/// # Examples
/// ```
/// use rustatlas::prelude::*;
///
/// let p = Period::new(5, TimeUnit::Days);
/// assert_eq!(p.length(), 5);
/// assert_eq!(p.units(), TimeUnit::Days);
/// ```
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Period {
    length: i32,
    units: TimeUnit,
}

impl Period {
    pub fn new(length: i32, units: TimeUnit) -> Period {
        Period { length, units }
    }

    pub fn from_frequency(freq: Frequency) -> Option<Period> {
        match freq {
            Frequency::NoFrequency => Some(Self {
                units: TimeUnit::Days,
                length: 0,
            }),
            Frequency::Once => Some(Self {
                units: TimeUnit::Years,
                length: 0,
            }),
            Frequency::Annual => Some(Self {
                units: TimeUnit::Years,
                length: 1,
            }),
            Frequency::Semiannual
            | Frequency::EveryFourthMonth
            | Frequency::Quarterly
            | Frequency::Bimonthly
            | Frequency::Monthly => Some(Self {
                units: TimeUnit::Months,
                length: 12 / (freq as i32),
            }),
            Frequency::EveryFourthWeek | Frequency::Biweekly | Frequency::Weekly => Some(Self {
                units: TimeUnit::Weeks,
                length: 52 / (freq as i32),
            }),
            Frequency::Daily => Some(Self {
                units: TimeUnit::Days,
                length: 1,
            }),
            Frequency::OtherFrequency => None,
        }
    }

    pub fn frequency(&self) -> Frequency {
        let length = self.length.abs(); // assuming `length` is i32 or some integer type

        if length == 0 {
            match self.units {
                TimeUnit::Years => return Frequency::Once,
                _ => return Frequency::NoFrequency,
            }
        }

        match self.units {
            TimeUnit::Years => {
                if length == 1 {
                    Frequency::Annual
                } else {
                    Frequency::OtherFrequency
                }
            }
            TimeUnit::Months => {
                let quotient = 12 / length;
                if 12 % length == 0 && length <= 12 {
                    match quotient {
                        1 => Frequency::Annual,
                        2 => Frequency::Semiannual,
                        3 => Frequency::Quarterly,
                        4 => Frequency::EveryFourthMonth,
                        6 => Frequency::Bimonthly,
                        12 => Frequency::Monthly,
                        _ => Frequency::OtherFrequency,
                    }
                } else {
                    Frequency::OtherFrequency
                }
            }
            TimeUnit::Weeks => match length {
                1 => Frequency::Weekly,
                2 => Frequency::Biweekly,
                4 => Frequency::EveryFourthWeek,
                _ => Frequency::OtherFrequency,
            },
            TimeUnit::Days => {
                if length == 1 {
                    Frequency::Daily
                } else {
                    Frequency::OtherFrequency
                }
            }
        }
    }

    pub fn normalize(&mut self) {
        if self.length == 0 {
            self.units = TimeUnit::Days;
        }

        match self.units {
            TimeUnit::Months => {
                if self.length % 12 == 0 {
                    self.length /= 12;
                    self.units = TimeUnit::Years;
                }
            }
            TimeUnit::Days => {
                if self.length % 7 == 0 {
                    self.length /= 7;
                    self.units = TimeUnit::Weeks;
                }
            }
            TimeUnit::Weeks | TimeUnit::Years => {}
        }
    }

    pub fn length(&self) -> i32 {
        self.length
    }

    pub fn units(&self) -> TimeUnit {
        self.units
    }

    pub fn empty() -> Self {
        Self {
            length: 0,
            units: TimeUnit::Days,
        }
    }

    pub fn from_str(tenor: &str) -> Result<Period> {
        // parse multiple periods and add them
        let mut chars = tenor.chars();
        let mut periods = Vec::new();
        let mut current_period = String::new();
        while let Some(c) = chars.next() {
            if c.is_numeric() {
                current_period.push(c);
            } else {
                current_period.push(c);
                periods.push(current_period);
                current_period = String::new();
            }
        }
        let mut result = Period::empty();
        for period in periods {
            result = (result + Period::parse_single_period(&period)?)?;
        }
        Ok(result)
    }

    fn parse_single_period(tenor: &str) -> Result<Period> {
        let mut chars = tenor.chars();
        let mut length = String::new();
        let mut units = String::new();
        while let Some(c) = chars.next() {
            if c.is_numeric() {
                length.push(c);
            } else {
                units.push(c);
            }
        }
        let length = length.parse::<i32>().map_err(|_| {
            AtlasError::PeriodOperationErr(
                format!("Invalid period length ({})", length).to_string(),
            )
        })?;
        let units = match units.as_str() {
            "Y" => TimeUnit::Years,
            "M" => TimeUnit::Months,
            "W" => TimeUnit::Weeks,
            "D" => TimeUnit::Days,
            _ => {
                return Err(AtlasError::PeriodOperationErr(
                    format!("Invalid time unit ({})", units).to_string(),
                ))
            }
        };
        Ok(Period::new(length, units))
    }

    pub fn period_in_year(&self) -> f64 {
        match self.units {
            TimeUnit::Years => self.length as f64,
            TimeUnit::Months => self.length as f64 / 12.0,
            TimeUnit::Weeks => self.length as f64 / 52.0,
            TimeUnit::Days => self.length as f64 / 365.0,
        }
    }
}

impl TryFrom<String> for Period {
    type Error = AtlasError;

    fn try_from(s: String) -> Result<Self> {
        Period::from_str(&s)
    }
}

impl From<Period> for String {
    fn from(period: Period) -> Self {
        match period.units {
            TimeUnit::Years => format!("{}Y", period.length),
            TimeUnit::Months => format!("{}M", period.length),
            TimeUnit::Weeks => format!("{}W", period.length),
            TimeUnit::Days => format!("{}D", period.length),
        }
    }
}

/// Deserializes a string in the format like 1Y or 1Y6M to a Period.
impl<'de> serde::Deserialize<'de> for Period {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Period, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct PeriodVisitor;

        impl<'de> Visitor<'de> for PeriodVisitor {
            type Value = Period;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a string in the format like 1Y or 1Y6M")
            }

            fn visit_str<E>(self, v: &str) -> std::result::Result<Period, E>
            where
                E: de::Error,
            {
                // Parse the string to create a Period
                Period::from_str(v).map_err(de::Error::custom)
            }
        }

        deserializer.deserialize_str(PeriodVisitor)
    }
}

/// Serializes a Period to a string in the format like 1Y or 1Y6M.
impl Serialize for Period {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self.units {
            TimeUnit::Years => serializer.serialize_str(&format!("{}Y", self.length)),
            TimeUnit::Months => serializer.serialize_str(&format!("{}M", self.length)),
            TimeUnit::Weeks => serializer.serialize_str(&format!("{}W", self.length)),
            TimeUnit::Days => serializer.serialize_str(&format!("{}D", self.length)),
        }
    }
}

/// # PartialEq for Period
/// Compares two Periods.
/// # Examples
/// ```
/// use rustatlas::prelude::*;
/// let p1 = Period::new(5, TimeUnit::Days);
/// let p2 = Period::new(5, TimeUnit::Days);
/// assert_eq!(p1, p2);
///
/// let p3 = Period::new(5, TimeUnit::Days);
/// let p4 = Period::new(5, TimeUnit::Weeks);
/// assert!(p3<p4);
/// ```
impl PartialOrd for Period {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.length == other.length {
            self.units.partial_cmp(&other.units)
        } else {
            self.length.partial_cmp(&other.length)
        }
    }
}

/// # Ord for Period
/// Compares two Periods.
/// # Examples
/// ```
/// use rustatlas::prelude::*;
/// let p1 = Period::new(5, TimeUnit::Days);
/// let p2 = Period::new(5, TimeUnit::Days);
/// assert_eq!(p1, p2);
/// ```
impl Ord for Period {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.length == other.length {
            self.units.cmp(&other.units)
        } else {
            self.length.cmp(&other.length)
        }
    }

    fn max(self, other: Self) -> Self
    where
        Self: Sized,
    {
        std::cmp::max_by(self, other, Ord::cmp)
    }

    fn min(self, other: Self) -> Self
    where
        Self: Sized,
    {
        std::cmp::min_by(self, other, Ord::cmp)
    }

    fn clamp(self, min: Self, max: Self) -> Self
    where
        Self: Sized,
    {
        std::cmp::min(std::cmp::max(self, min), max)
    }
}

/// # Neg for Period
/// Negates a Period.
/// # Examples
/// ```
/// use rustatlas::prelude::*;
/// let p = Period::new(5, TimeUnit::Days);
/// let negated = -p;
/// assert_eq!(negated.length(), -5);
/// assert_eq!(negated.units(), TimeUnit::Days);
/// ```
impl Neg for Period {
    type Output = Period;

    fn neg(self) -> Self::Output {
        Period {
            length: -self.length,
            units: self.units,
        }
    }
}

/// # Add for Period
/// Adds a Period to another Period.
/// # Examples
/// ```
/// use rustatlas::prelude::*;
/// let p1 = Period::new(5, TimeUnit::Days);
/// let p2 = Period::new(3, TimeUnit::Days);
/// let p3 = (p1 + p2).unwrap();
/// assert_eq!(p3.length(), 8);
/// assert_eq!(p3.units(), TimeUnit::Days);
impl Add for Period {
    type Output = Result<Period>;

    fn add(self, other: Self) -> Self::Output {
        let mut result = self;
        if result.length == 0 {
            result.length = other.length;
            result.units = other.units;
        } else if result.units == other.units {
            result.length += other.length;
        } else {
            match result.units {
                TimeUnit::Years => match other.units {
                    TimeUnit::Months => {
                        result.units = TimeUnit::Months;
                        result.length = result.length * 12 + other.length;
                    }
                    TimeUnit::Weeks | TimeUnit::Days => {
                        return Err(AtlasError::PeriodOperationErr(
                            format!("impossible addition between {:?} and {:?}", result, other)
                                .to_string(),
                        ));
                    }

                    _ => {
                        return Err(AtlasError::PeriodOperationErr(
                            format!("unknown time unit ({:?})", other.units).to_string(),
                        ))
                    }
                },

                TimeUnit::Months => match other.units {
                    TimeUnit::Years => {
                        result.length += other.length * 12;
                    }
                    TimeUnit::Weeks | TimeUnit::Days => {
                        return Err(AtlasError::PeriodOperationErr(
                            format!("impossible addition between {:?} and {:?}", result, other)
                                .to_string(),
                        ));
                    }

                    _ => {
                        return Err(AtlasError::PeriodOperationErr(
                            format!("unknown time unit ({:?})", other.units).to_string(),
                        ))
                    }
                },

                TimeUnit::Weeks => match other.units {
                    TimeUnit::Days => {
                        result.units = TimeUnit::Days;
                        result.length = result.length * 7 + other.length;
                    }
                    TimeUnit::Years | TimeUnit::Months => {
                        return Err(AtlasError::PeriodOperationErr(
                            format!("impossible addition between {:?} and {:?}", result, other)
                                .to_string(),
                        ));
                    }

                    _ => {
                        return Err(AtlasError::PeriodOperationErr(
                            format!("unknown time unit ({:?})", other.units).to_string(),
                        ))
                    }
                },

                TimeUnit::Days => match other.units {
                    TimeUnit::Weeks => {
                        result.length += other.length * 7;
                    }
                    TimeUnit::Years | TimeUnit::Months => {
                        return Err(AtlasError::PeriodOperationErr(
                            format!("impossible addition between {:?} and {:?}", result, other)
                                .to_string(),
                        ));
                    }

                    _ => {
                        return Err(AtlasError::PeriodOperationErr(
                            format!("unknown time unit ({:?})", other.units).to_string(),
                        ))
                    }
                },
            }
        }
        Ok(result)
    }
}

/// # Sub for Period
/// Subtracts a Period from another Period.
/// # Examples
/// ```
/// use rustatlas::prelude::*;
///
/// let p1 = Period::new(5, TimeUnit::Days);
/// let p2 = Period::new(3, TimeUnit::Days);
/// let p3 = (p1 - p2).unwrap();
/// assert_eq!(p3.length(), 2);
/// assert_eq!(p3.units(), TimeUnit::Days);
/// ```
impl Sub for Period {
    type Output = Result<Period>;

    fn sub(self, other: Self) -> Self::Output {
        self + -other
    }
}

/// # Mul`<i32>` for Period
/// Multiplies a Period by an integer.
/// # Examples
/// ```
/// use rustatlas::time::enums::TimeUnit;
/// use rustatlas::time::period::Period;
/// let p = Period::new(5, TimeUnit::Days);
/// let p2 = p * 2;
/// assert_eq!(p2.length(), 10);
/// assert_eq!(p2.units(), TimeUnit::Days);
/// ```
impl Mul<i32> for Period {
    type Output = Period;

    fn mul(self, n: i32) -> Self::Output {
        Period {
            length: self.length * n,
            units: self.units,
        }
    }
}

/// # MulAssign`<i32>` for Period
/// Multiplies a Period by an integer.
/// # Examples
/// ```
/// use rustatlas::time::enums::TimeUnit;
/// use rustatlas::time::period::Period;
/// let mut p = Period::new(5, TimeUnit::Days);
/// p *= 2;
/// assert_eq!(p.length(), 10);
/// assert_eq!(p.units(), TimeUnit::Days);
/// ```
impl MulAssign<i32> for Period {
    fn mul_assign(&mut self, n: i32) {
        self.length *= n;
    }
}

// Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_period_negation() {
        let p = Period {
            length: 5,
            units: TimeUnit::Days,
        };
        let negated = -p;
        assert_eq!(negated.length, -5);
        assert_eq!(negated.units, TimeUnit::Days);
    }

    #[test]
    fn test_period_addition() {
        let p1 = Period {
            length: 5,
            units: TimeUnit::Days,
        };
        let p2 = Period {
            length: 3,
            units: TimeUnit::Days,
        };
        let p3 = (p1 + p2).unwrap();
        assert_eq!(p3.length, 8);
        assert_eq!(p3.units, TimeUnit::Days);
    }

    #[test]
    fn test_period_subtraction() {
        let p1 = Period {
            length: 5,
            units: TimeUnit::Days,
        };
        let p2 = Period {
            length: 3,
            units: TimeUnit::Days,
        };
        let p3 = (p1 - p2).unwrap();
        assert_eq!(p3.length, 2);
        assert_eq!(p3.units, TimeUnit::Days);
    }

    #[test]
    fn test_period_multiplication() {
        let mut p = Period {
            length: 5,
            units: TimeUnit::Days,
        };
        p *= 2;
        assert_eq!(p.length, 10);
        assert_eq!(p.units, TimeUnit::Days);
    }

    #[test]
    fn test_normalize_weeks() {
        let mut p = Period {
            length: 7,
            units: TimeUnit::Days,
        };
        p.normalize();
        assert_eq!(p.length, 1);
        assert_eq!(p.units, TimeUnit::Weeks);
    }

    #[test]
    fn test_normalize_years() {
        let mut p = Period {
            length: 12,
            units: TimeUnit::Months,
        };
        p.normalize();
        assert_eq!(p.length, 1);
        assert_eq!(p.units, TimeUnit::Years);
    }

    #[test]
    fn test_period_addition_different_units_years_months() {
        let p1 = Period {
            length: 1,
            units: TimeUnit::Years,
        };
        let p2 = Period {
            length: 6,
            units: TimeUnit::Months,
        };
        let p3 = (p1 + p2).unwrap();
        assert_eq!(p3.length, 18);
        assert_eq!(p3.units, TimeUnit::Months);
    }

    #[test]
    fn test_period_addition_different_units_weeks_days() {
        let p1 = Period {
            length: 2,
            units: TimeUnit::Weeks,
        };
        let p2 = Period {
            length: 3,
            units: TimeUnit::Days,
        };
        let p3 = (p1 + p2).unwrap();
        assert_eq!(p3.length, 17);
        assert_eq!(p3.units, TimeUnit::Days);
    }

    #[test]
    fn test_period_addition_days_weeks() {
        let p1 = Period {
            length: 10,
            units: TimeUnit::Days,
        };
        let p2 = Period {
            length: 1,
            units: TimeUnit::Weeks,
        };
        let p3: Period = (p1 + p2).unwrap();
        assert_eq!(p3.length, 17);
        assert_eq!(p3.units, TimeUnit::Days);
    }

    #[test]
    fn test_period_addition_months_years() {
        let p1 = Period {
            length: 6,
            units: TimeUnit::Months,
        };
        let p2 = Period {
            length: 1,
            units: TimeUnit::Years,
        };
        let p3 = (p1 + p2).unwrap();
        assert_eq!(p3.length, 18);
        assert_eq!(p3.units, TimeUnit::Months);
    }

    #[test]
    fn test_impossible_addition_weeks_years() -> Result<()> {
        // should err
        let p1 = Period {
            length: 2,
            units: TimeUnit::Weeks,
        };
        let p2 = Period {
            length: 1,
            units: TimeUnit::Years,
        };
        let r = p1 + p2;
        match r {
            Err(_) => Ok(()),
            _ => Err(AtlasError::PeriodOperationErr(
                "impossible addition between 2W and 1Y".to_string(),
            )),
        }
    }

    #[test]
    fn test_impossible_addition_days_years() -> Result<()> {
        let p1 = Period {
            length: 5,
            units: TimeUnit::Days,
        };
        let p2 = Period {
            length: 1,
            units: TimeUnit::Years,
        };
        let r = p1 + p2;
        match r {
            Err(_) => Ok(()),
            _ => Err(AtlasError::PeriodOperationErr(
                "impossible addition between 5D and 1Y".to_string(),
            )),
        }
    }

    #[test]
    fn test_period_parsing() {
        let p = Period::from_str("1Y").unwrap();
        assert_eq!(p.length(), 1);
        assert_eq!(p.units(), TimeUnit::Years);

        let p = Period::from_str("1M").unwrap();
        assert_eq!(p.length(), 1);
        assert_eq!(p.units(), TimeUnit::Months);

        let p = Period::from_str("1Y1M").unwrap();
        assert_eq!(p.length(), 13);
        assert_eq!(p.units(), TimeUnit::Months);
    }
}
