use super::traits::DayCountProvider;
use crate::time::date::Date;

/// # Actual360
/// Actual/360 day count convention.
/// Calculates the day count fraction according to the formula:
/// $$
/// \frac{ActualDays}{360}
/// $$
/// where ActualDays is the number of days between the start date and the end date.
/// # Example
/// ```
/// use rustatlas::prelude::*;
///
/// let start = Date::from_ymd(2020, 1, 1);
/// let end = Date::from_ymd(2020, 2, 1);
/// let day_count = Actual360 {};
/// assert_eq!(day_count.day_count(start, end), 31);
/// assert_eq!(day_count.year_fraction(start, end), 31.0 / 360.0);
/// ```
pub struct Actual360;

impl DayCountProvider for Actual360 {
    fn day_count(&self, start: Date, end: Date) -> i64 {
        return end - start;
    }

    fn year_fraction(&self, start: Date, end: Date) -> f64 {
        return self.day_count(start, end) as f64 / 360.0;
    }
}
