use super::traits::DayCountProvider;
use crate::time::date::Date;

/// # Actual365
/// Actual/365 day count convention.
/// Calculates the day count fraction according to the formula:
/// $$
/// \frac{ActualDays}{365}
/// $$
/// where ActualDays is the number of days between the start date and the end date.
/// # Example
/// ```
/// use crate::time::daycounters::actual365::Actual365;
/// use crate::time::traits::DayCountProvider;
/// use crate::time::date::Date;
///
/// let start = Date::from_ymd(2020, 1, 1);
/// let end = Date::from_ymd(2020, 2, 1);
/// let day_count = Actual365 {};
/// assert_eq!(day_count.day_count(start, end), 31);
/// assert_eq!(day_count.year_fraction(start, end), 31.0 / 365.0);
/// ```
pub struct Actual365;

impl DayCountProvider for Actual365 {
    fn day_count(&self, start: Date, end: Date) -> i64 {
        return end - start;
    }

    fn year_fraction(&self, start: Date, end: Date) -> f64 {
        return self.day_count(start, end) as f64 / 365.0;
    }
}
