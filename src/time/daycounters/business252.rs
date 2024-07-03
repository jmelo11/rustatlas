use super::traits::DayCountProvider;
use crate::core::meta::{NewNumeric, Numeric};
use crate::time::calendar::Calendar;
use crate::time::calendars::brazil::Market;
use crate::time::{
    calendars::{brazil::Brazil, traits::ImplCalendar},
    date::Date,
};

/// # Business252
/// Business/252 day count convention.
/// Calculates the number of business days between two dates.
/// # Example
/// ```
/// use rustatlas::prelude::*;
///
/// let start = Date::new(2020, 1, 1);
/// let end = Date::new(2020, 2, 1);
/// assert_eq!(Business252::day_count(start, end), 22);
/// assert_eq!(Business252::year_fraction(start, end), 22.0 / 252.0);
/// ```
pub struct Business252;

impl DayCountProvider for Business252 {
    fn day_count(start: Date, end: Date) -> Numeric {
        let calendar = Calendar::Brazil(Brazil::new(Market::Settlement));

        if end < start {
            return Numeric::new(-(calendar.business_day_list(start, end).len() as f64));
        } else {
            return Numeric::new(calendar.business_day_list(start, end).len() as f64);
        }
    }

    fn year_fraction(start: Date, end: Date) -> Numeric {
        Self::day_count(start, end) / 252.0
    }
}

#[cfg(feature = "f64")]
#[cfg(test)]
mod test {
    use crate::prelude::DayCountProvider;

    #[test]
    fn test_business252() {
        use crate::time::date::Date;
        use crate::time::daycounters::business252::Business252;
        let start = Date::new(2020, 1, 1);
        let end = Date::new(2020, 2, 1);
        assert_eq!(Business252::day_count(start, end), 22);
        assert_eq!(Business252::year_fraction(start, end), 22.0 / 252.0);
    }
}
