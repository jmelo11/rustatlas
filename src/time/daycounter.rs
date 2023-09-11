use super::daycounters::actual360::Actual360;
use super::daycounters::actual365::Actual365;
use super::daycounters::thirty360::Thirty360;
use super::daycounters::traits::DayCountProvider;
use crate::time::date::Date;

/// # DayCounter
/// Day count convention enum.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DayCounter {
    Actual360,
    Actual365,
    Thirty360,
}

impl DayCounter {
    pub fn day_count(&self, start: Date, end: Date) -> i64 {
        match self {
            DayCounter::Actual360 => Actual360::day_count(start, end),
            DayCounter::Actual365 => Actual365::day_count(start, end),
            DayCounter::Thirty360 => Thirty360::day_count(start, end),
        }
    }

    pub fn year_fraction(&self, start: Date, end: Date) -> f64 {
        match self {
            DayCounter::Actual360 => Actual360::year_fraction(start, end),
            DayCounter::Actual365 => Actual365::year_fraction(start, end),
            DayCounter::Thirty360 => Thirty360::year_fraction(start, end),
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_count() {
        let start = Date::new(2020, 1, 1);
        let end = Date::new(2020, 1, 2);  
        

        let day_count = DayCounter::Actual360.day_count(start, end);
        assert_eq!(day_count, 1);
        let day_count = DayCounter::Actual365.day_count(start, end);
        assert_eq!(day_count, 1);
        let day_count = DayCounter::Thirty360.day_count(start, end);
        assert_eq!(day_count, 1);
    }

    #[test]
    fn test_year_fraction() {
        let start = Date::new(2020, 1, 1);
        let end = Date::new(2020, 1, 2);   

        let year_fraction = DayCounter::Actual360.year_fraction(start, end);
        assert_eq!(year_fraction, 1.0/360.0);
        let year_fraction = DayCounter::Actual365.year_fraction(start, end);
        assert_eq!(year_fraction, 1.0/365.0);
        let year_fraction = DayCounter::Thirty360.year_fraction(start, end);
        assert_eq!(year_fraction, 1.0/360.0);

    }


}