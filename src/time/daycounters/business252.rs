use super::traits::DayCountProvider;
use crate::time::{calendar::Calendar, calendars::{brazil::Brazil, traits::ImplCalendar}, date::Date};
use crate::time::calendars::brazil::Market;


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
    fn day_count(start: Date, end: Date) -> i64 {

        let calendar = Calendar::Brazil(Brazil::new(Market::Settlement));

        if end < start {
            
            return  -(calendar.business_day_list(start, end).len() as i64);
        }
        else {
            return calendar.business_day_list(start, end).len() as i64
        }
    }

    fn year_fraction(start: Date, end: Date) -> f64 {
        Self::day_count(start, end) as f64 / 252.0
    }
}

#[cfg(test)]
mod test {
    use crate::time::daycounters::traits::DayCountProvider;

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