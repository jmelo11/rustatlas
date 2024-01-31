use super::traits::DayCountProvider;
use crate::time::date::Date;

/// # Thirty360 (ISMA)
/// Convention: if the starting date is the 31st of a
/// month, it becomes equal to the 30th of the same month.
/// If the ending date is the 31st of a month and the starting
/// date is the 30th or 31th of a month, the ending date
/// also becomes equal to the 30th of the month.
/// ```
/// use rustatlas::prelude::*;
///
/// let start = Date::new(2020, 1, 1);
/// let end = Date::new(2020, 2, 1);
/// assert_eq!(Thirty360::day_count(start, end), 30);
/// assert_eq!(Thirty360::year_fraction(start, end), 30.0 / 360.0);
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

/// # Thirty360US
/// convention: if the starting date is the 31st of a month or
/// the last day of February, it becomes equal to the 30th of the
/// same month.  If the ending date is the 31st of a month and the
/// starting date is the 30th or 31th of a month, the ending date
/// becomes equal to the 30th.  If the ending date is the last of
/// February and the starting date is also the last of February,
/// the ending date becomes equal to the 30th.
/// Also known as "30/360" or "360/360".
/// ```
/// use rustatlas::prelude::*;
///
/// let start = Date::new(2020, 1, 1);
/// let end = Date::new(2020, 2, 1);
/// assert_eq!(Thirty360US::day_count(start, end), 30);
/// assert_eq!(Thirty360US::year_fraction(start, end), 30.0 / 360.0);
/// ```
pub struct Thirty360US;


fn is_last_of_february(d: i64, m: i64, y: i32) -> bool {
    if Date::is_leap_year(y) {
        return m == 2 && d == 28 + 1;
    } else {
        return m == 2 && d == 28;
    }    
}

impl DayCountProvider for Thirty360US {
    fn day_count(start: Date, end: Date) -> i64 {
        let d1 = start.day() as i64;
        let d2 = end.day() as i64;
        let m1 = start.month() as i64;
        let m2 = end.month() as i64;
        let y1 = start.year();
        let y2 = end.year();

        let dd1 = if d1 == 31 { 30 } else { d1 };
        let dd2 = if d2 == 31 && dd1 >= 30 { 30 } else { d2 };

        let dd1 = if is_last_of_february(dd1, m1, y1) { 30 } else { dd1 };
        let dd2 = if is_last_of_february(dd2, m2, y2) { 30 } else { dd2 };

        return 360 * ((y2 as i64) - (y1 as i64)) + 30 * (m2 - m1) + (dd2 - dd1);
    }

    fn year_fraction(start: Date, end: Date) -> f64 {
        return Thirty360US::day_count(start, end) as f64 / 360.0;
    }
}