use serde::{Deserialize, Serialize};

use super::daycounters::{
    actual360::Actual360, actual365fixed::Actual365Fixed, thirty360::Thirty360ISMA, traits::DayCountProvider,
};
use crate::{
    time::date::Date,
    utils::errors::{AtlasError, Result}
};

/// # DayCounter
/// Day count convention enum.
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DayCounter {
    Actual360,
    Actual365Fixed,
    Thirty360ISMA,
}

impl DayCounter {
    pub fn day_count(&self, start: Date, end: Date) -> i64 {
        match self {
            DayCounter::Actual360 => Actual360::day_count(start, end),
            DayCounter::Actual365Fixed => Actual365Fixed::day_count(start, end),
            DayCounter::Thirty360ISMA => Thirty360ISMA::day_count(start, end),
        }
    }

    pub fn year_fraction(&self, start: Date, end: Date) -> f64 {
        match self {
            DayCounter::Actual360 => Actual360::year_fraction(start, end),
            DayCounter::Actual365Fixed => Actual365Fixed::year_fraction(start, end),
            DayCounter::Thirty360ISMA => Thirty360ISMA::year_fraction(start, end),
        }
    }
}

impl TryFrom<String> for DayCounter {
    type Error = AtlasError;

    fn try_from(s: String) -> Result<Self> {
        match s.as_str() {
            "Actual360" => Ok(DayCounter::Actual360),
            "Actual365Fixed" => Ok(DayCounter::Actual365Fixed),
            "Thirty360ISMA" => Ok(DayCounter::Thirty360ISMA),
            _ => Err(AtlasError::InvalidValueErr(format!(
                "Invalid day counter: {}",
                s
            ))),
        }
    }
}

impl From<DayCounter> for String {
    fn from(day_counter: DayCounter) -> Self {
        match day_counter {
            DayCounter::Actual360 => "Actual360".to_string(),
            DayCounter::Actual365Fixed => "Actual365Fixed".to_string(),
            DayCounter::Thirty360ISMA => "Thirty360ISMA".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::time::daycounter;

    use super::*;

    #[test]
    fn test_day_count_standard() {
        let start = Date::new(2020, 1, 1);
        let end = Date::new(2020, 1, 2);

        let day_count = DayCounter::Actual360.day_count(start, end);
        assert_eq!(day_count, 1);
        let day_count = DayCounter::Actual365Fixed.day_count(start, end);
        assert_eq!(day_count, 1);
        let day_count = DayCounter::Thirty360ISMA.day_count(start, end);
        assert_eq!(day_count, 1);

    }

    #[test]
    fn test_year_fraction() {
        let start = Date::new(2020, 1, 1);
        let end = Date::new(2020, 1, 2);

        let year_fraction = DayCounter::Actual360.year_fraction(start, end);
        assert_eq!(year_fraction, 1.0 / 360.0);
        let year_fraction = DayCounter::Actual365Fixed.year_fraction(start, end);
        assert_eq!(year_fraction, 1.0 / 365.0);
        let year_fraction = DayCounter::Thirty360ISMA.year_fraction(start, end);
        assert_eq!(year_fraction, 1.0 / 360.0);
    }


    #[test]
    fn test_year_fraction_trithy360_end_of_month() {
        let start = Date::new(2023, 12, 10);
        let end_1 = Date::new(2023, 12, 30);
        let end_2 = Date::new(2023, 12, 31);

        let yf_1 = DayCounter::Thirty360.year_fraction(start, end_1);
        let yf_2 = DayCounter::Thirty360.year_fraction(start, end_2);
        assert_ne!(yf_1, yf_2);
    }

    #[test]
    fn test_year_fraction_trithy360_end_of_month_2() {
        let start = Date::new(2023, 12, 10);
        let end_1 = Date::new(2023, 12, 31);
        let end_2 = Date::new(2024, 1, 1);

        let yf_1 = DayCounter::Thirty360.year_fraction(start, end_1);
        let yf_2 = DayCounter::Thirty360.year_fraction(start, end_2);
        println!("{} days between {} and {} with Thirty360", yf_1, start, end_1);
        println!("{} days between {} and {} with Thirty360", yf_2, start, end_2);
        assert_eq!(yf_1, yf_2);
    }

    #[test]
    fn test_year_fraction_trithy360_1() {
        let start = Date::new(2023, 12, 30);
        let end_1 = Date::new(2023, 12, 31);
        let end_2 = Date::new(2024, 1, 1);

        let yf_1 = DayCounter::Thirty360.year_fraction(start, end_1);
        let yf_2 = DayCounter::Thirty360.year_fraction(start, end_2);
        //println!("{} days between {} and {} with Thirty360", yf_1, start, end_1);
        //println!("{} days between {} and {} with Thirty360", yf_2, start, end_2);
        assert_ne!(yf_1, yf_2);
    }


    #[test]
    fn test_year_fraction_trithy360_4() {
        let start = Date::new(2024, 2, 10);
        let end_1 = Date::new(2024, 2, 29);
        let end_2 = Date::new(2024, 3, 1);

        let yf_1 = DayCounter::Thirty360.year_fraction(start, end_1);
        let yf_2 = DayCounter::Thirty360.year_fraction(start, end_2);
        //println!("{} days between {} and {} with Thirty360", yf_1, start, end_1);
        //println!("{} days between {} and {} with Thirty360", yf_2, start, end_2);
        assert_ne!(yf_1, yf_2);
    }


    #[test]
    fn test_year_fraction_trithy360_5() {
        let start = Date::new(2023, 2, 10);
        let end_1 = Date::new(2023, 2, 28);
        let end_2 = Date::new(2023, 3, 1);

        let yf_1 = DayCounter::Thirty360.year_fraction(start, end_1);
        let yf_2 = DayCounter::Thirty360.year_fraction(start, end_2);
        //println!("{} days between {} and {} with Thirty360", yf_1, start, end_1);
        //println!("{} days between {} and {} with Thirty360", yf_2, start, end_2);
        assert_ne!(yf_1, yf_2);
    }
}


