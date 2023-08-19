use super::enums::{Frequency, TimeUnit};
use std::ops::{Add, AddAssign, DivAssign, MulAssign, Neg, Sub, SubAssign};

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

    pub fn frequency(&self) -> Result<Frequency, String> {
        let length = self.length.abs(); // assuming `length` is i32 or some integer type

        if length == 0 {
            match self.units {
                TimeUnit::Years => return Ok(Frequency::Once),
                _ => return Ok(Frequency::NoFrequency),
            }
        }

        match self.units {
            TimeUnit::Years => {
                if length == 1 {
                    Ok(Frequency::Annual)
                } else {
                    Ok(Frequency::OtherFrequency)
                }
            }
            TimeUnit::Months => {
                let quotient = 12 / length;
                if 12 % length == 0 && length <= 12 {
                    match quotient {
                        1 => Ok(Frequency::Annual),
                        2 => Ok(Frequency::Semiannual),
                        3 => Ok(Frequency::Quarterly),
                        4 => Ok(Frequency::EveryFourthMonth),
                        6 => Ok(Frequency::Bimonthly),
                        12 => Ok(Frequency::Monthly),
                        _ => Ok(Frequency::OtherFrequency),
                    }
                } else {
                    Ok(Frequency::OtherFrequency)
                }
            }
            TimeUnit::Weeks => match length {
                1 => Ok(Frequency::Weekly),
                2 => Ok(Frequency::Biweekly),
                4 => Ok(Frequency::EveryFourthWeek),
                _ => Ok(Frequency::OtherFrequency),
            },
            TimeUnit::Days => {
                if length == 1 {
                    Ok(Frequency::Daily)
                } else {
                    Ok(Frequency::OtherFrequency)
                }
            }
            _ => Err(format!("Unsupported time unit ({:?})", self.units)),
        }
    }

    pub fn normalize(&mut self) -> Result<(), String> {
        if self.length == 0 {
            self.units = TimeUnit::Days;
            return Ok(());
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
            _ => return Err(format!("Unsupported time unit ({:?})", self.units)),
        }
        Ok(())
    }

    pub fn length(&self) -> i32 {
        self.length
    }

    pub fn units(&self) -> TimeUnit {
        self.units
    }
}

// Operators

impl Neg for Period {
    type Output = Period;

    fn neg(self) -> Self::Output {
        Period {
            length: -self.length,
            units: self.units,
        }
    }
}

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

                _ => panic!("unknown time unit ({:?})", self.units),
            }
        }
    }
}

impl Add for Period {
    type Output = Period;

    fn add(self, other: Self) -> Self::Output {
        let mut result = self;
        result += other;
        result
    }
}

impl SubAssign for Period {
    fn sub_assign(&mut self, other: Self) {
        *self += -other;
    }
}

impl Sub for Period {
    type Output = Period;

    fn sub(self, other: Self) -> Self::Output {
        let mut result = self;
        result -= other;
        result
    }
}

impl MulAssign<i32> for Period {
    fn mul_assign(&mut self, n: i32) {
        self.length *= n;
    }
}

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
    #[should_panic(expected = "cannot be divided by zero")]
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
        match p.normalize() {
            Ok(_) => {
                assert_eq!(p.length, 1);
                assert_eq!(p.units, TimeUnit::Weeks);
            }
            Err(e) => panic!("Error: {}", e),
        };
    }

    #[test]
    fn test_normalize_years() {
        let mut p = Period {
            length: 12,
            units: TimeUnit::Months,
        };
        match p.normalize() {
            Ok(_) => {
                assert_eq!(p.length, 1);
                assert_eq!(p.units, TimeUnit::Years);
            }
            Err(e) => panic!("Error: {}", e),
        };
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

    // ... You can add more test cases for more specific scenarios.
}
