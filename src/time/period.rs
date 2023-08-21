use super::enums::{Frequency, TimeUnit};
use std::ops::{Add, AddAssign, DivAssign, MulAssign, Neg, Sub, SubAssign};

/// # Period
/// Struct representing a financial period.
/// # Examples
/// ```
/// use rustatlas::time::enums::{Frequency, TimeUnit};
/// use rustatlas::time::period::Period;
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
    pub fn new(length: i32, units: TimeUnit) -> Self {
        Self { length, units }
    }

    pub fn from_frequency(freq: Frequency) -> Result<Self, String> {
        match freq {
            Frequency::NoFrequency => Ok(Self {
                units: TimeUnit::Days,
                length: 0,
            }),
            Frequency::Once => Ok(Self {
                units: TimeUnit::Years,
                length: 0,
            }),
            Frequency::Annual => Ok(Self {
                units: TimeUnit::Years,
                length: 1,
            }),
            Frequency::Semiannual
            | Frequency::EveryFourthMonth
            | Frequency::Quarterly
            | Frequency::Bimonthly
            | Frequency::Monthly => Ok(Self {
                units: TimeUnit::Months,
                length: 12 / (freq as i32),
            }),
            Frequency::EveryFourthWeek | Frequency::Biweekly | Frequency::Weekly => Ok(Self {
                units: TimeUnit::Weeks,
                length: 52 / (freq as i32),
            }),
            Frequency::Daily => Ok(Self {
                units: TimeUnit::Days,
                length: 1,
            }),
            Frequency::OtherFrequency => Err("unknown frequency".to_string()),
            _ => Err(format!("unknown frequency ({:?})", freq)),
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
}

/// # Neg for Period
/// Negates a Period.
/// # Examples
/// ```
/// use rustatlas::time::enums::TimeUnit;
/// use rustatlas::time::period::Period;
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

/// # AddAssign for Period
/// Adds a Period to another Period.
/// # Examples
/// ```
/// use rustatlas::time::enums::TimeUnit;
/// use rustatlas::time::period::Period;
/// let mut p1 = Period::new(5, TimeUnit::Days);
/// let p2 = Period::new(3, TimeUnit::Days);
/// p1 += p2;
/// assert_eq!(p1.length(), 8);
/// assert_eq!(p1.units(), TimeUnit::Days);
/// ```
impl AddAssign for Period {
    fn add_assign(&mut self, other: Self) {
        if self.length == 0 {
            self.length = other.length;
            self.units = other.units;
        } else if self.units == other.units {
            self.length += other.length;
        } else {
            match self.units {
                TimeUnit::Years => match other.units {
                    TimeUnit::Months => {
                        self.units = TimeUnit::Months;
                        self.length = self.length * 12 + other.length;
                    }
                    TimeUnit::Weeks | TimeUnit::Days => {
                        assert!(
                            other.length == 0,
                            "impossible addition between {:?} and {:?}",
                            self,
                            other
                        );
                    }
                    _ => panic!("unknown time unit ({:?})", other.units),
                },

                TimeUnit::Months => match other.units {
                    TimeUnit::Years => {
                        self.length += other.length * 12;
                    }
                    TimeUnit::Weeks | TimeUnit::Days => {
                        assert!(
                            other.length == 0,
                            "impossible addition between {:?} and {:?}",
                            self,
                            other
                        );
                    }
                    _ => panic!("unknown time unit ({:?})", other.units),
                },

                TimeUnit::Weeks => match other.units {
                    TimeUnit::Days => {
                        self.units = TimeUnit::Days;
                        self.length = self.length * 7 + other.length;
                    }
                    TimeUnit::Years | TimeUnit::Months => {
                        assert!(
                            other.length == 0,
                            "impossible addition between {:?} and {:?}",
                            self,
                            other
                        );
                    }
                    _ => panic!("unknown time unit ({:?})", other.units),
                },

                TimeUnit::Days => match other.units {
                    TimeUnit::Weeks => {
                        self.length += other.length * 7;
                    }
                    TimeUnit::Years | TimeUnit::Months => {
                        assert!(
                            other.length == 0,
                            "impossible addition between {:?} and {:?}",
                            self,
                            other
                        );
                    }
                    _ => panic!("unknown time unit ({:?})", other.units),
                },
            }
        }
    }
}

/// # Add for Period
/// Adds a Period to another Period.
/// # Examples
/// ```
/// use rustatlas::time::enums::TimeUnit;
/// use rustatlas::time::period::Period;
/// let p1 = Period::new(5, TimeUnit::Days);
/// let p2 = Period::new(3, TimeUnit::Days);
/// let p3 = p1 + p2;
/// assert_eq!(p3.length(), 8);
/// assert_eq!(p3.units(), TimeUnit::Days);
/// ```
impl Add for Period {
    type Output = Period;

    fn add(self, other: Self) -> Self::Output {
        let mut result = self;
        result += other;
        result
    }
}

/// # SubAssign for Period
/// Subtracts a Period from another Period.
/// # Examples
/// ```
/// use rustatlas::time::enums::TimeUnit;
/// use rustatlas::time::period::Period;
/// let mut p1 = Period::new(5, TimeUnit::Days);
/// let p2 = Period::new(3, TimeUnit::Days);
/// p1 -= p2;
/// assert_eq!(p1.length(), 2);
/// assert_eq!(p1.units(), TimeUnit::Days);
/// ```
impl SubAssign for Period {
    fn sub_assign(&mut self, other: Self) {
        *self += -other;
    }
}

/// # Sub for Period
/// Subtracts a Period from another Period.
/// # Examples
/// ```
/// use rustatlas::time::enums::TimeUnit;
/// use rustatlas::time::period::Period;
/// let p1 = Period::new(5, TimeUnit::Days);
/// let p2 = Period::new(3, TimeUnit::Days);
/// let p3 = p1 - p2;
/// assert_eq!(p3.length(), 2);
/// assert_eq!(p3.units(), TimeUnit::Days);
/// ```
impl Sub for Period {
    type Output = Period;

    fn sub(self, other: Self) -> Self::Output {
        let mut result = self;
        result -= other;
        result
    }
}

/// # MulAssign<i32> for Period
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

/// # DivAssign<i32> for Period
/// Divides a Period by an integer.
/// # Examples
/// ```
/// use rustatlas::time::enums::TimeUnit;
/// use rustatlas::time::period::Period;
/// let mut p = Period::new(10, TimeUnit::Days);
/// p /= 2;
/// assert_eq!(p.length(), 5);
/// assert_eq!(p.units(), TimeUnit::Days);
/// ```
/// # Panics
/// Panics if the integer is zero.
impl DivAssign<i32> for Period {
    fn div_assign(&mut self, n: i32) {
        assert!(n != 0, "cannot be divided by zero");
        if self.length % n == 0 {
            self.length /= n;
        } else {
            let mut units = self.units;
            let mut length = self.length;
            match units {
                TimeUnit::Years => {
                    length *= 12;
                    units = TimeUnit::Months;
                }
                TimeUnit::Weeks => {
                    length *= 7;
                    units = TimeUnit::Days;
                }
                _ => {}
            }
            assert!(length % n == 0, "{:?} cannot be divided by {}", self, n);
            self.length = length / n;
            self.units = units;
        }
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
        let mut p1 = Period {
            length: 5,
            units: TimeUnit::Days,
        };
        let p2 = Period {
            length: 3,
            units: TimeUnit::Days,
        };
        p1 += p2;
        assert_eq!(p1.length, 8);
        assert_eq!(p1.units, TimeUnit::Days);
    }

    #[test]
    fn test_period_subtraction() {
        let mut p1 = Period {
            length: 5,
            units: TimeUnit::Days,
        };
        let p2 = Period {
            length: 3,
            units: TimeUnit::Days,
        };
        p1 -= p2;
        assert_eq!(p1.length, 2);
        assert_eq!(p1.units, TimeUnit::Days);
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
    fn test_period_division() {
        let mut p = Period {
            length: 10,
            units: TimeUnit::Days,
        };
        p /= 2;
        assert_eq!(p.length, 5);
        assert_eq!(p.units, TimeUnit::Days);
    }

    #[test]
    #[should_panic]
    fn test_period_division_by_zero() {
        let mut p = Period {
            length: 10,
            units: TimeUnit::Days,
        };
        p /= 0;
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
        let mut p1 = Period {
            length: 1,
            units: TimeUnit::Years,
        };
        let p2 = Period {
            length: 6,
            units: TimeUnit::Months,
        };
        p1 += p2;
        assert_eq!(p1.length, 18);
        assert_eq!(p1.units, TimeUnit::Months);
    }

    #[test]
    fn test_period_addition_different_units_weeks_days() {
        let mut p1 = Period {
            length: 2,
            units: TimeUnit::Weeks,
        };
        let p2 = Period {
            length: 3,
            units: TimeUnit::Days,
        };
        p1 += p2;
        assert_eq!(p1.length, 17);
        assert_eq!(p1.units, TimeUnit::Days);
    }

    #[test]
    fn test_period_addition_days_weeks() {
        let mut p1 = Period {
            length: 10,
            units: TimeUnit::Days,
        };
        let p2 = Period {
            length: 1,
            units: TimeUnit::Weeks,
        };
        p1 += p2;
        assert_eq!(p1.length, 17);
        assert_eq!(p1.units, TimeUnit::Days);
    }

    #[test]
    fn test_period_addition_months_years() {
        let mut p1 = Period {
            length: 6,
            units: TimeUnit::Months,
        };
        let p2 = Period {
            length: 1,
            units: TimeUnit::Years,
        };
        p1 += p2;
        assert_eq!(p1.length, 18);
        assert_eq!(p1.units, TimeUnit::Months);
    }

    #[test]
    #[should_panic(expected = "impossible addition")]
    fn test_impossible_addition_weeks_years() {
        let mut p1 = Period {
            length: 2,
            units: TimeUnit::Weeks,
        };
        let p2 = Period {
            length: 1,
            units: TimeUnit::Years,
        };
        p1 += p2;
    }

    #[test]
    #[should_panic(expected = "impossible addition")]
    fn test_impossible_addition_days_years() {
        let mut p1 = Period {
            length: 5,
            units: TimeUnit::Days,
        };
        let p2 = Period {
            length: 1,
            units: TimeUnit::Years,
        };
        p1 += p2;
    }
}
