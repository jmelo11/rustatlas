use super::traits::DayCountProvider;
use crate::time::date::Date;

/// # `ActualActual`
/// Actual/Actual day count convention.
/// Calculates the day count fraction according to the formula:
/// $$
/// \frac{`ActualDays_of_leap_years`}{366} + \frac{`ActualDays_of_non_leap_years`}{365}
/// $$
/// where `ActualDays` of leap years is the number of days between the start date and the end date in leap years
/// and `ActualDays` of non-leap years is the number of days between the start date and the end date in non-leap years.
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

const fn days_in_year(year: i32) -> i32 {
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
        let days = Self::day_count(start, end);

        let y1 = start.year();
        let y2 = end.year();

        match y1.cmp(&y2) {
            std::cmp::Ordering::Equal => {
                let days = i32::try_from(days)
                    .unwrap_or_else(|_| panic!("day count should fit in i32"));
                f64::from(days) / f64::from(days_in_year(y1))
            }
            std::cmp::Ordering::Less => {
                let mut sum = 0.0;
                let start_days = i32::try_from(Date::new(y1 + 1, 1, 1) - start)
                    .unwrap_or_else(|_| panic!("day count should fit in i32"));
                sum += f64::from(start_days) / f64::from(days_in_year(y1));
                for _year in y1 + 1..y2 - 1 {
                    sum += 1.0;
                }
                let end_days = i32::try_from(end - Date::new(y2, 1, 1))
                    .unwrap_or_else(|_| panic!("day count should fit in i32"));
                sum += f64::from(end_days) / f64::from(days_in_year(y2));

                sum
            }
            std::cmp::Ordering::Greater => {
                let mut sum = 0.0;
                let end_days = i32::try_from(Date::new(y2 + 1, 1, 1) - end)
                    .unwrap_or_else(|_| panic!("day count should fit in i32"));
                sum -= f64::from(end_days) / f64::from(days_in_year(y2));
                for _year in y2 + 1..y1 - 1 {
                    sum -= 1.0;
                }
                let start_days = i32::try_from(start - Date::new(y1, 1, 1))
                    .unwrap_or_else(|_| panic!("day count should fit in i32"));
                sum -= f64::from(start_days) / f64::from(days_in_year(y1));
                sum
            }
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
        let yf = ActualActual::year_fraction(start, end);
        assert!((yf - 31.0 / 366.0).abs() < 1e-12);
    }

    #[test]
    fn test_actualactual_year_fraction2() {
        use super::ActualActual;
        use crate::time::date::Date;
        let start = Date::new(2020, 1, 1);
        let end = Date::new(2021, 1, 1);
        let yf = ActualActual::year_fraction(start, end);
        assert!((yf - 1.0).abs() < 1e-12);
    }

    #[test]
    fn test_actualactual_year_fraction3() {
        use super::ActualActual;
        use crate::time::date::Date;
        let start = Date::new(2021, 1, 1);
        let end = Date::new(2020, 1, 1);
        let yf = ActualActual::year_fraction(start, end);
        assert!((yf + 1.0).abs() < 1e-12);
    }
}
