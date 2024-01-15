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
/// use rustatlas::prelude::*;
///
/// let start = Date::new(2020, 1, 1);
/// let end = Date::new(2020, 2, 1);
/// assert_eq!(Actual365Fixed::day_count(start, end), 31);
/// assert_eq!(Actual365Fixed::year_fraction(start, end), 31.0 / 365.0);
/// ```
pub struct Actual365Fixed;

impl DayCountProvider for Actual365Fixed {
    fn day_count(start: Date, end: Date) -> i64 {
        return end - start;
    }

    fn year_fraction(start: Date, end: Date) -> f64 {
        return Actual365Fixed
        ::day_count(start, end) as f64 / 365.0;
    }
}
