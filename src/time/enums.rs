use std::ops::{Add, Sub};

use serde::{Deserialize, Serialize};

/// # Frequency
/// Enum representing a financial frequency.
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
pub enum Frequency {
    NoFrequency = -1,
    Once = 0,
    Annual = 1,
    Semiannual = 2,
    EveryFourthMonth = 3,
    Quarterly = 4,
    Bimonthly = 6,
    Monthly = 12,
    EveryFourthWeek = 13,
    Biweekly = 26,
    Weekly = 52,
    Daily = 365,
    OtherFrequency = 999,
}

/// # TimeUnit
/// Enum representing a time unit.
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
pub enum TimeUnit {
    Days,
    Weeks,
    Months,
    Years,
}

/// # Month
/// Enum representing a month.
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
pub enum Month {
    January = 1,
    February,
    March,
    April,
    May,
    June,
    July,
    August,
    September,
    October,
    November,
    December,
}

/// # IMMMonth
/// Enum representing an IMM month.
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
pub enum IMMMonth {
    F = 1,
    G = 2,
    H = 3,
    J = 4,
    K = 5,
    M = 6,
    N = 7,
    Q = 8,
    U = 9,
    V = 10,
    X = 11,
    Z = 12,
}

/// # DateGenerationRule
/// Enum representing a date generation rule.
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone, Copy)]
pub enum DateGenerationRule {
    Backward,
    Forward,
    Zero,
    ThirdWednesday,
    ThirdWednesdayInclusive,
    Twentieth,
    TwentiethIMM,
    OldCDS,
    CDS,
    CDS2015,
}

/// # BusinessDayConvention
/// Enum representing a business day convention.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum BusinessDayConvention {
    Following,
    ModifiedFollowing,
    Preceding,
    ModifiedPreceding,
    Unadjusted,
    HalfMonthModifiedFollowing,
    Nearest,
}

/// # Weekday
/// Enum representing a weekday.
#[derive(Debug, PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
pub enum Weekday {
    Sunday = 1,
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
}

impl Add<i32> for Weekday {
    type Output = i32;

    fn add(self, rhs: i32) -> Self::Output {
        let res = self as i32 + rhs;
        return res;
    }
}

impl Sub<i32> for Weekday {
    type Output = i32;

    fn sub(self, rhs: i32) -> Self::Output {
        return self + -rhs;
    }
}

impl Add<Weekday> for Weekday {
    type Output = i32;

    fn add(self, rhs: Weekday) -> Self::Output {
        return rhs as i32 + self as i32;
    }
}

impl Sub<Weekday> for Weekday {
    type Output = i32;

    fn sub(self, rhs: Weekday) -> Self::Output {
        return self as i32 + -(rhs as i32);
    }
}

impl Add<Weekday> for i32 {
    type Output = i32;

    fn add(self, rhs: Weekday) -> Self::Output {
        return rhs + self;
    }
}

impl Sub<Weekday> for i32 {
    type Output = i32;

    fn sub(self, rhs: Weekday) -> Self::Output {
        return self + -(rhs as i32);
    }
}

#[cfg(test)]
mod tests {
    use super::Weekday;

    #[test]
    fn test_add() {
        assert_eq!(Weekday::Monday + 1, 3);
    }

    #[test]
    fn test_sub() {
        assert_eq!(Weekday::Monday - 1, 1);
    }

    #[test]
    fn test_add_weekday() {
        assert_eq!(Weekday::Monday + Weekday::Tuesday, 5);
    }

    #[test]
    fn test_sub_weekday() {
        assert_eq!(Weekday::Monday - Weekday::Tuesday, -1);
    }

    #[test]
    fn test_add_i32() {
        assert_eq!(1 + Weekday::Monday, 3);
    }

    #[test]
    fn test_sub_i32() {
        assert_eq!(1 - Weekday::Monday, -1);
    }
}
