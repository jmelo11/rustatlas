use super::traits::DayCountProvider;
use crate::time::date::Date;

/// # Thirty360
/// 30/360 day count convention.
/// Calculates the day count fraction according to the formula:
/// $$
/// \frac{360(Y2 - Y1) + 30(M2 - M1) + (D2 - D1)}{360}
/// $$
/// where Y1, M1, D1 are the year, month and day of the start date, and Y2, M2, D2 are the year, month and day of the end date.
/// # Example
/// ```
/// use rustatlas::prelude::*;
///
/// let start = Date::from_ymd(2020, 1, 1);
/// let end = Date::from_ymd(2020, 2, 1);
/// let day_count = Thirty360 {};
/// assert_eq!(day_count.day_count(start, end), 30);
/// assert_eq!(day_count.year_fraction(start, end), 30.0 / 360.0);
/// ```
pub struct Thirty360;

impl DayCountProvider for Thirty360 {
    fn day_count(start: Date, end: Date) -> i64 {
        let d1 = start.day() as i64;
        let d2 = end.day() as i64;
        let m1 = start.month() as i64;
        let m2 = end.month() as i64;
        let y1 = start.year() as i64;
        let y2 = end.year() as i64;

        let dd1 = if d1 == 31 { 30 } else { d1 };
        let dd2 = if d2 == 31 && dd1 == 30 { 30 } else { d2 };

        return 360 * (y2 - y1) + 30 * (m2 - m1) + (dd2 - dd1);
    }

    fn year_fraction(start: Date, end: Date) -> f64 {
        return Thirty360::day_count(start, end) as f64 / 360.0;
    }
}
