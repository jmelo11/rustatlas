use super::traits::DayCountProvider;
use crate::time::date::Date;

/// # ActualActual
/// Actual/Actual day count convention.
/// Calculates the day count fraction according to the formula:
/// $$
/// \frac{ActualDays_of_leap_years}{366} + \frac{ActualDays_of_non_leap_years}{365}
/// $$
/// where ActualDays of leap years is the number of days between the start date and the end date in leap years
/// and ActualDays of non-leap years is the number of days between the start date and the end date in non-leap years.
/// # Example
/// ```
/// use rustatlas::prelude::*;
///
/// let start = Date::new(2020, 1, 1);
/// let end = Date::new(2020, 2, 1);
/// assert_eq!(ActualActual::day_count(start, end), 31);
/// assert_eq!(ActualActual::year_fraction(start, end), 31.0 / 366.0);
/// ```
pub struct ActualActual;

fn days_in_year(year: i32) -> i32 {
    if Date::is_leap_year(year) {
        366
    } else {
        365
    }
}

impl DayCountProvider for ActualActual {
    fn day_count(start: Date, end: Date) -> i64 {
        end - start
    }

    fn year_fraction(start: Date, end: Date) -> f64 {
        let days = ActualActual::day_count(start, end);

        let y1 = start.year();
        let y2 = end.year();

        if y1 == y2 {
            days as f64 / days_in_year(y1) as f64
        } else if y2 > y1 {
            let mut sum = 0.0;
            sum += (Date::new(y1 + 1, 1, 1) - start) as f64 / days_in_year(y1) as f64;
            for _year in y1 + 1..y2 - 1 {
                sum += 1.0;
            }
            sum += (end - Date::new(y2, 1, 1)) as f64 / days_in_year(y2) as f64;

            sum
        } else {
            let mut sum = 0.0;
            sum -= (Date::new(y2 + 1, 1, 1) - end) as f64 / days_in_year(y2) as f64;
            for _year in y2 + 1..y1 - 1 {
                sum -= 1.0;
            }
            sum -= (start - Date::new(y1, 1, 1)) as f64 / days_in_year(y1) as f64;
            sum
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::time::daycounters::traits::DayCountProvider;

    #[test]
    fn test_actualactual_day_count() {
        use super::ActualActual;
        use crate::time::date::Date;
        let start = Date::new(2020, 1, 1);
        let end = Date::new(2020, 2, 1);
        assert_eq!(ActualActual::day_count(start, end), 31);
    }

    #[test]
    fn test_actualactual_year_fraction() {
        use super::ActualActual;
        use crate::time::date::Date;
        let start = Date::new(2020, 1, 1);
        let end = Date::new(2020, 2, 1);
        assert_eq!(ActualActual::year_fraction(start, end), 31.0 / 366.0);
    }

    #[test]
    fn test_actualactual_year_fraction2() {
        use super::ActualActual;
        use crate::time::date::Date;
        let start = Date::new(2020, 1, 1);
        let end = Date::new(2021, 1, 1);
        assert_eq!(ActualActual::year_fraction(start, end), 1.0);
    }

    #[test]
    fn test_actualactual_year_fraction3() {
        use super::ActualActual;
        use crate::time::date::Date;
        let start = Date::new(2021, 1, 1);
        let end = Date::new(2020, 1, 1);
        assert_eq!(ActualActual::year_fraction(start, end), -1.0);
    }
}
