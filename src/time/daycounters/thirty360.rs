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
/// use crate::time::daycounters::thirty360::Thirty360;
/// use crate::time::traits::DayCountProvider;
/// use crate::time::date::Date;
///
/// let start = Date::from_ymd(2020, 1, 1);
/// let end = Date::from_ymd(2020, 2, 1);
/// let day_count = Thirty360 {};
/// assert_eq!(day_count.day_count(start, end), 30);
/// assert_eq!(day_count.year_fraction(start, end), 30.0 / 360.0);
/// ```
pub struct Thirty360;

impl DayCountProvider for Thirty360 {
    fn day_count(&self, start: Date, end: Date) -> i64 {
        let mut d1 = start.day() as i32;
        let mut d2 = end.day() as i32;
        let mut m1 = start.month() as i32;
        let mut m2 = end.month() as i32;
        let mut y1 = start.year();
        let mut y2 = end.year();

        if d1 == 31 {
            d1 = 30;
        }
        if d2 == 31 {
            d2 = 30;
        }

        if d1 == 30 && d2 == 30 {
            d2 = 31;
        }

        if d1 == 30 && d2 == 31 {
            d2 = 1;
            m2 += 1;
        }

        if d1 == 31 && d2 == 30 {
            d2 = 1;
            m2 += 1;
        }

        if d1 == 31 && d2 == 31 {
            d2 = 1;
            m2 += 1;
        }

        if m1 == 12 && m2 == 1 {
            m2 = 12;
            y2 -= 1;
        }

        return (360 * (y2 - y1) + 30 * (m2 - m1) + (d2 - d1)) as i64;
    }

    fn year_fraction(&self, start: Date, end: Date) -> f64 {
        return self.day_count(start, end) as f64 / 360.0;
    }
}
