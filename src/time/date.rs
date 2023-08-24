use super::enums::TimeUnit;
use super::period::Period;
use chrono::{Datelike, Duration, Months, NaiveDate, Weekday};
use std::fmt::Display;
use std::ops::{Add, AddAssign, Sub, SubAssign};

/// # NaiveDateExt
/// Extends the NaiveDate struct from the chrono rustatlas.
/// # Examples
/// ```
/// use rustatlas::prelude::*;
/// use chrono::NaiveDate;
///
/// let date = NaiveDate::from_ymd_opt(2020, 2, 15).unwrap();
/// assert_eq!(date.days_in_month(), 29);
/// let date = NaiveDate::from_ymd_opt(2020, 5, 15).unwrap();
/// assert_eq!(date.days_in_year(), 366);
/// let date = NaiveDate::from_ymd_opt(2020, 5, 15).unwrap();
/// assert!(date.is_leap_year());
/// let date = NaiveDate::from_ymd_opt(2020, 1, 15).unwrap();
/// assert_eq!(date.advance(15, TimeUnit::Days), NaiveDate::from_ymd_opt(2020, 1, 30).unwrap());
/// ```
pub trait NaiveDateExt {
    fn days_in_month(&self) -> i32;
    fn days_in_year(&self) -> i32;
    fn is_leap_year(&self) -> bool;
    fn advance(&self, n: i32, units: TimeUnit) -> NaiveDate;
    fn end_of_month(&self, date: NaiveDate) -> NaiveDate;
}

impl NaiveDateExt for NaiveDate {
    fn days_in_month(&self) -> i32 {
        let month = self.month();
        match month {
            1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
            4 | 6 | 9 | 11 => 30,
            2 => {
                if self.is_leap_year() {
                    29
                } else {
                    28
                }
            }
            _ => panic!("Invalid month: {}", month),
        }
    }

    fn days_in_year(&self) -> i32 {
        if self.is_leap_year() {
            366
        } else {
            365
        }
    }

    fn is_leap_year(&self) -> bool {
        let year = self.year();
        return year % 4 == 0 && (year % 100 != 0 || year % 400 == 0);
    }

    fn advance(&self, n: i32, units: TimeUnit) -> NaiveDate {
        let date = *self;
        let flag = n >= 0;
        return match units {
            TimeUnit::Days => date + Duration::days(n as i64),
            TimeUnit::Weeks => date + Duration::days(7 * n as i64),
            TimeUnit::Months => {
                if flag {
                    return date + Months::new(n as u32);
                } else {
                    return date - Months::new((-n) as u32);
                }
            }
            TimeUnit::Years => {
                if flag {
                    return date + Months::new(12 * n as u32);
                } else {
                    return date - Months::new((-12 * n) as u32);
                }
            }
        };
    }

    fn end_of_month(&self, date: NaiveDate) -> NaiveDate {
        let month = date.month();
        let year = date.year();
        let mut end_of_month = NaiveDate::from_ymd_opt(year, month, 1).unwrap();
        end_of_month = end_of_month + Months::new(1);
        end_of_month = end_of_month - Duration::days(1);
        end_of_month
    }
}

/// # Add`<Period>` for NaiveDate
/// Adds a Period to a NaiveDate.
/// # Examples
/// ```
/// use chrono::NaiveDate;
/// use rustatlas::prelude::*;
///
/// let date = NaiveDate::from_ymd_opt(2020, 1, 15).unwrap();
/// let period = Period::new(15, TimeUnit::Days);
/// assert_eq!(date + period, NaiveDate::from_ymd_opt(2020, 1, 30).unwrap());
/// ```
impl Add<Period> for NaiveDate {
    type Output = NaiveDate;

    fn add(self, rhs: Period) -> Self::Output {
        let n = rhs.length();
        let units = rhs.units();
        return self.advance(n, units);
    }
}

/// # Date
/// Wrapper around the NaiveDate struct from the chrono rustatlas.
/// # Examples
/// ```
/// use rustatlas::time::date::Date;
/// let date = Date::new(2020, 2, 15);
/// assert_eq!(date.day(), 15);
/// assert_eq!(date.month(), 2);
/// assert_eq!(date.year(), 2020);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Date {
    base_date: NaiveDate,
}

impl Date {
    pub fn new(year: i32, month: u32, day: u32) -> Date {
        let base_date = NaiveDate::from_ymd_opt(year, month, day);
        match base_date {
            Some(base_date) => Date::from_base_date(base_date),
            None => panic!("Invalid date: {}-{}-{}", year, month, day),
        }
    }

    pub fn from_base_date(base_date: NaiveDate) -> Date {
        Date { base_date }
    }

    // deprecated
    #[deprecated]
    pub fn from_ymd(year: i32, month: u32, day: u32) -> Date {
        Date::new(year, month, day)
    }

    pub fn base_date(&self) -> NaiveDate {
        self.base_date
    }

    pub fn day(&self) -> u32 {
        self.base_date.day()
    }

    pub fn month(&self) -> u32 {
        self.base_date.month()
    }

    pub fn year(&self) -> i32 {
        self.base_date.year()
    }

    pub fn days_in_month(&self) -> i32 {
        self.base_date.days_in_month()
    }

    pub fn days_in_year(&self) -> i32 {
        self.base_date.days_in_year()
    }

    pub fn is_leap_year(&self) -> bool {
        self.base_date.is_leap_year()
    }

    pub fn advance(&self, n: i32, units: TimeUnit) -> Date {
        let base_date = self.base_date.advance(n, units);
        Date::from_base_date(base_date)
    }

    pub fn add_period(&self, period: Period) -> Date {
        let base_date = self.base_date + period;
        Date::from_base_date(base_date)
    }

    pub fn end_of_month(&self) -> Date {
        let base_date = self.base_date.end_of_month(self.base_date);
        Date::from_base_date(base_date)
    }

    pub fn weekday(&self) -> Weekday {
        self.base_date.weekday()
    }

    pub fn empty() -> Date {
        //min
        Date::from_base_date(NaiveDate::MIN)
    }
}

/// # Sub for Date
/// Subtracts two Dates and returns the difference in days.
/// # Examples
/// ```
/// use rustatlas::time::date::Date;
/// let date1 = Date::new(2020, 2, 15);
/// let date2 = Date::new(2020, 2, 10);
/// assert_eq!(date1 - date2, 5);
/// ```
impl Sub for Date {
    type Output = i64;

    fn sub(self, rhs: Self) -> Self::Output {
        let base_date = self.base_date;
        let rhs_base_date = rhs.base_date;
        return (base_date - rhs_base_date).num_days();
    }
}

/// # Add`<Period>` for Date
/// Adds a Period to a Date.
/// # Examples
/// ```
/// use rustatlas::prelude::*;
/// let date = Date::new(2020, 1, 15);
/// let period = Period::new(15, TimeUnit::Days);
/// assert_eq!(date + period, Date::new(2020, 1, 30));
/// ```
impl Add<Period> for Date {
    type Output = Date;

    fn add(self, rhs: Period) -> Self::Output {
        let base_date: NaiveDate = self.base_date + rhs;
        Date::from_base_date(base_date)
    }
}

/// # Add`<i64>` for Date
/// Adds an i64 to a Date.
/// # Examples
/// ```
/// use rustatlas::prelude::*;
/// let date = Date::new(2020, 1, 15);
/// assert_eq!(date + 15, Date::new(2020, 1, 30));
/// ```
impl Add<i64> for Date {
    type Output = Date;

    fn add(self, rhs: i64) -> Self::Output {
        let base_date: NaiveDate = self.base_date + Duration::days(rhs as i64);
        Date::from_base_date(base_date)
    }
}

/// # AddAssign`<i64>` for Date
/// Adds an i64 to a Date.
/// # Examples
/// ```
/// use rustatlas::prelude::*;
/// let mut date = Date::new(2020, 1, 15);
/// date += 15;
/// assert_eq!(date, Date::new(2020, 1, 30));
/// ```
impl AddAssign<i64> for Date {
    fn add_assign(&mut self, rhs: i64) {
        self.base_date = self.base_date + Duration::days(rhs as i64);
    }
}

/// # Sub`<i64>` for Date
/// Subtracts an i64 from a Date.
/// # Examples
/// ```
/// use rustatlas::prelude::*;
/// let date = Date::new(2020, 1, 30);
/// assert_eq!(date - 15, Date::new(2020, 1, 15));
/// ```
impl Sub<i64> for Date {
    type Output = Date;

    fn sub(self, rhs: i64) -> Self::Output {
        let base_date: NaiveDate = self.base_date - Duration::days(rhs as i64);
        Date::from_base_date(base_date)
    }
}

/// # SubAssign`<i64>` for Date
/// Subtracts an i64 from a Date.
/// # Examples
/// ```
/// use rustatlas::prelude::*;
/// let mut date = Date::new(2020, 1, 30);
/// date -= 15;
/// assert_eq!(date, Date::new(2020, 1, 15));
/// ```
impl SubAssign<i64> for Date {
    fn sub_assign(&mut self, rhs: i64) {
        self.base_date = self.base_date - Duration::days(rhs as i64);
    }
}

/// # Display for Date
/// Formats a Date as a string.
/// # Examples
/// ```
/// use rustatlas::time::date::Date;
/// let date = Date::new(2020, 1, 15);
/// assert_eq!(date.to_string(), "2020-01-15");
/// ```
impl Display for Date {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let base_date = self.base_date;
        write!(f, "{}", base_date.format("%Y-%m-%d"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    #[test]
    fn test_days_in_month() {
        let date = NaiveDate::from_ymd_opt(2020, 2, 15).unwrap();
        assert_eq!(date.days_in_month(), 29);

        let date = NaiveDate::from_ymd_opt(2021, 2, 15).unwrap();
        assert_eq!(date.days_in_month(), 28);

        let date = NaiveDate::from_ymd_opt(2021, 4, 15).unwrap();
        assert_eq!(date.days_in_month(), 30);

        let date = NaiveDate::from_ymd_opt(2021, 7, 15).unwrap();
        assert_eq!(date.days_in_month(), 31);
    }

    #[test]
    fn test_days_in_year() {
        let date = NaiveDate::from_ymd_opt(2020, 5, 15).unwrap();
        assert_eq!(date.days_in_year(), 366);

        let date = NaiveDate::from_ymd_opt(2021, 5, 15).unwrap();
        assert_eq!(date.days_in_year(), 365);
    }

    #[test]
    fn test_is_leap_year() {
        let date = NaiveDate::from_ymd_opt(2020, 5, 15).unwrap();
        assert!(date.is_leap_year());

        let date = NaiveDate::from_ymd_opt(2021, 5, 15).unwrap();
        assert!(!date.is_leap_year());
    }

    #[test]
    fn test_advance() {
        let date = NaiveDate::from_ymd_opt(2020, 1, 15).unwrap();
        assert_eq!(
            date.advance(15, TimeUnit::Days),
            NaiveDate::from_ymd_opt(2020, 1, 30).unwrap()
        );

        let date = NaiveDate::from_ymd_opt(2020, 1, 15).unwrap();
        assert_eq!(
            date.advance(3, TimeUnit::Weeks),
            NaiveDate::from_ymd_opt(2020, 2, 5).unwrap()
        );

        let date = NaiveDate::from_ymd_opt(2020, 1, 15).unwrap();
        assert_eq!(
            date.advance(2, TimeUnit::Months),
            NaiveDate::from_ymd_opt(2020, 3, 15).unwrap()
        );

        let date = NaiveDate::from_ymd_opt(2020, 1, 15).unwrap();
        assert_eq!(
            date.advance(2, TimeUnit::Years),
            NaiveDate::from_ymd_opt(2022, 1, 15).unwrap()
        );
    }

    #[test]
    fn test_addition_with_period() {
        let date = NaiveDate::from_ymd_opt(2020, 1, 15).unwrap();
        let period = Period::new(15, TimeUnit::Days);
        assert_eq!(date + period, NaiveDate::from_ymd_opt(2020, 1, 30).unwrap());

        let date = NaiveDate::from_ymd_opt(2020, 1, 1).unwrap();
        let period = Period::new(6, TimeUnit::Months);
        assert_eq!(date + period, NaiveDate::from_ymd_opt(2020, 7, 1).unwrap());
    }
}
