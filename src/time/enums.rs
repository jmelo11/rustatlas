use std::{
    hash::Hash,
    ops::{Add, Sub},
};

use serde::{Deserialize, Serialize};

use crate::utils::errors::{AtlasError, Result};

/// # Frequency
/// Enum representing a financial frequency.
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone, Copy, PartialOrd, Ord, Hash)]
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

impl TryFrom<String> for Frequency {
    type Error = AtlasError;

    fn try_from(s: String) -> Result<Self> {
        match s.as_str() {
            "NoFrequency" => Ok(Frequency::NoFrequency),
            "Once" => Ok(Frequency::Once),
            "Annual" => Ok(Frequency::Annual),
            "Semiannual" => Ok(Frequency::Semiannual),
            "EveryFourthMonth" => Ok(Frequency::EveryFourthMonth),
            "Quarterly" => Ok(Frequency::Quarterly),
            "Bimonthly" => Ok(Frequency::Bimonthly),
            "Monthly" => Ok(Frequency::Monthly),
            "EveryFourthWeek" => Ok(Frequency::EveryFourthWeek),
            "Biweekly" => Ok(Frequency::Biweekly),
            "Weekly" => Ok(Frequency::Weekly),
            "Daily" => Ok(Frequency::Daily),
            "OtherFrequency" => Ok(Frequency::OtherFrequency),
            _ => Err(AtlasError::InvalidValueErr(format!(
                "Invalid frequency: {}",
                s
            ))),
        }
    }
}

impl From<Frequency> for String {
    fn from(frequency: Frequency) -> Self {
        match frequency {
            Frequency::NoFrequency => "NoFrequency".to_string(),
            Frequency::Once => "Once".to_string(),
            Frequency::Annual => "Annual".to_string(),
            Frequency::Semiannual => "Semiannual".to_string(),
            Frequency::EveryFourthMonth => "EveryFourthMonth".to_string(),
            Frequency::Quarterly => "Quarterly".to_string(),
            Frequency::Bimonthly => "Bimonthly".to_string(),
            Frequency::Monthly => "Monthly".to_string(),
            Frequency::EveryFourthWeek => "EveryFourthWeek".to_string(),
            Frequency::Biweekly => "Biweekly".to_string(),
            Frequency::Weekly => "Weekly".to_string(),
            Frequency::Daily => "Daily".to_string(),
            Frequency::OtherFrequency => "OtherFrequency".to_string(),
        }
    }
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

impl TryFrom<String> for TimeUnit {
    type Error = AtlasError;

    fn try_from(s: String) -> Result<Self> {
        match s.as_str() {
            "Days" => Ok(TimeUnit::Days),
            "Weeks" => Ok(TimeUnit::Weeks),
            "Months" => Ok(TimeUnit::Months),
            "Years" => Ok(TimeUnit::Years),
            _ => Err(AtlasError::InvalidValueErr(format!(
                "Invalid time unit: {}",
                s
            ))),
        }
    }
}

impl From<TimeUnit> for String {
    fn from(time_unit: TimeUnit) -> Self {
        match time_unit {
            TimeUnit::Days => "Days".to_string(),
            TimeUnit::Weeks => "Weeks".to_string(),
            TimeUnit::Months => "Months".to_string(),
            TimeUnit::Years => "Years".to_string(),
        }
    }
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

impl TryFrom<String> for Month {
    type Error = AtlasError;

    fn try_from(s: String) -> Result<Self> {
        match s.as_str() {
            "January" => Ok(Month::January),
            "February" => Ok(Month::February),
            "March" => Ok(Month::March),
            "April" => Ok(Month::April),
            "May" => Ok(Month::May),
            "June" => Ok(Month::June),
            "July" => Ok(Month::July),
            "August" => Ok(Month::August),
            "September" => Ok(Month::September),
            "October" => Ok(Month::October),
            "November" => Ok(Month::November),
            "December" => Ok(Month::December),
            _ => Err(AtlasError::InvalidValueErr(format!("Invalid month: {}", s))),
        }
    }
}

impl From<Month> for String {
    fn from(month: Month) -> Self {
        match month {
            Month::January => "January".to_string(),
            Month::February => "February".to_string(),
            Month::March => "March".to_string(),
            Month::April => "April".to_string(),
            Month::May => "May".to_string(),
            Month::June => "June".to_string(),
            Month::July => "July".to_string(),
            Month::August => "August".to_string(),
            Month::September => "September".to_string(),
            Month::October => "October".to_string(),
            Month::November => "November".to_string(),
            Month::December => "December".to_string(),
        }
    }
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
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone, Copy, Hash)]
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

impl TryFrom<String> for DateGenerationRule {
    type Error = AtlasError;

    fn try_from(s: String) -> Result<Self> {
        match s.as_str() {
            "Backward" => Ok(DateGenerationRule::Backward),
            "Forward" => Ok(DateGenerationRule::Forward),
            "Zero" => Ok(DateGenerationRule::Zero),
            "ThirdWednesday" => Ok(DateGenerationRule::ThirdWednesday),
            "ThirdWednesdayInclusive" => Ok(DateGenerationRule::ThirdWednesdayInclusive),
            "Twentieth" => Ok(DateGenerationRule::Twentieth),
            "TwentiethIMM" => Ok(DateGenerationRule::TwentiethIMM),
            "OldCDS" => Ok(DateGenerationRule::OldCDS),
            "CDS" => Ok(DateGenerationRule::CDS),
            "CDS2015" => Ok(DateGenerationRule::CDS2015),
            _ => Err(AtlasError::InvalidValueErr(format!(
                "Invalid date generation rule: {}",
                s
            ))),
        }
    }
}

impl From<DateGenerationRule> for String {
    fn from(date_generation_rule: DateGenerationRule) -> Self {
        match date_generation_rule {
            DateGenerationRule::Backward => "Backward".to_string(),
            DateGenerationRule::Forward => "Forward".to_string(),
            DateGenerationRule::Zero => "Zero".to_string(),
            DateGenerationRule::ThirdWednesday => "ThirdWednesday".to_string(),
            DateGenerationRule::ThirdWednesdayInclusive => "ThirdWednesdayInclusive".to_string(),
            DateGenerationRule::Twentieth => "Twentieth".to_string(),
            DateGenerationRule::TwentiethIMM => "TwentiethIMM".to_string(),
            DateGenerationRule::OldCDS => "OldCDS".to_string(),
            DateGenerationRule::CDS => "CDS".to_string(),
            DateGenerationRule::CDS2015 => "CDS2015".to_string(),
        }
    }
}

/// # BusinessDayConvention
/// Enum representing a business day convention. Business day conventions are used to
/// adjust a date in case it is not a business day.
///
/// ## Convention
/// * Following - Choose the first business day after the given holiday.
/// * ModifiedFollowing - Choose the first business day after the given holiday unless
/// it belongs to a different month, in which case choose the first business day before the given holiday.
/// * Preceding - Choose the first business day before the given holiday.
/// * ModifiedPreceding - Choose the first business day before the given holiday unless
/// it belongs to a different month, in which case choose the first business day after the given holiday.
/// * Unadjusted - Do not adjust.
/// * HalfMonthModifiedFollowing - Choose the first business day after the given holiday
/// unless that day falls in the first half of the month, in which case choose the first business day before the given holiday.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Deserialize, Serialize, Hash)]
pub enum BusinessDayConvention {
    Following,
    ModifiedFollowing,
    Preceding,
    ModifiedPreceding,
    Unadjusted,
    HalfMonthModifiedFollowing,
    Nearest,
}

impl TryFrom<String> for BusinessDayConvention {
    type Error = AtlasError;

    fn try_from(s: String) -> Result<Self> {
        match s.as_str() {
            "Following" => Ok(BusinessDayConvention::Following),
            "ModifiedFollowing" => Ok(BusinessDayConvention::ModifiedFollowing),
            "Preceding" => Ok(BusinessDayConvention::Preceding),
            "ModifiedPreceding" => Ok(BusinessDayConvention::ModifiedPreceding),
            "Unadjusted" => Ok(BusinessDayConvention::Unadjusted),
            "HalfMonthModifiedFollowing" => Ok(BusinessDayConvention::HalfMonthModifiedFollowing),
            "Nearest" => Ok(BusinessDayConvention::Nearest),
            _ => Err(AtlasError::InvalidValueErr(format!(
                "Invalid business day convention: {}",
                s
            ))),
        }
    }
}

impl From<BusinessDayConvention> for String {
    fn from(business_day_convention: BusinessDayConvention) -> Self {
        match business_day_convention {
            BusinessDayConvention::Following => "Following".to_string(),
            BusinessDayConvention::ModifiedFollowing => "ModifiedFollowing".to_string(),
            BusinessDayConvention::Preceding => "Preceding".to_string(),
            BusinessDayConvention::ModifiedPreceding => "ModifiedPreceding".to_string(),
            BusinessDayConvention::Unadjusted => "Unadjusted".to_string(),
            BusinessDayConvention::HalfMonthModifiedFollowing => {
                "HalfMonthModifiedFollowing".to_string()
            }
            BusinessDayConvention::Nearest => "Nearest".to_string(),
        }
    }
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

impl TryFrom<String> for Weekday {
    type Error = AtlasError;

    fn try_from(s: String) -> Result<Self> {
        match s.as_str() {
            "Sunday" => Ok(Weekday::Sunday),
            "Monday" => Ok(Weekday::Monday),
            "Tuesday" => Ok(Weekday::Tuesday),
            "Wednesday" => Ok(Weekday::Wednesday),
            "Thursday" => Ok(Weekday::Thursday),
            "Friday" => Ok(Weekday::Friday),
            "Saturday" => Ok(Weekday::Saturday),
            _ => Err(AtlasError::InvalidValueErr(format!(
                "Invalid weekday: {}",
                s
            ))),
        }
    }
}

impl From<Weekday> for String {
    fn from(weekday: Weekday) -> Self {
        match weekday {
            Weekday::Sunday => "Sunday".to_string(),
            Weekday::Monday => "Monday".to_string(),
            Weekday::Tuesday => "Tuesday".to_string(),
            Weekday::Wednesday => "Wednesday".to_string(),
            Weekday::Thursday => "Thursday".to_string(),
            Weekday::Friday => "Friday".to_string(),
            Weekday::Saturday => "Saturday".to_string(),
        }
    }
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
