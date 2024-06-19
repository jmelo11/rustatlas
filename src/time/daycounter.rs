use serde::{Deserialize, Serialize};

use super::daycounters::{
    actual360::Actual360, actual365::Actual365, actualactual::ActualActual, business252::Business252, thirty360::*, traits::DayCountProvider
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
    Actual365,
    Thirty360,
    Thirty360US,
    ActualActual,
    Business252,
}

impl DayCounter {
    pub fn day_count(&self, start: Date, end: Date) -> i64 {
        match self {
            DayCounter::Actual360 => Actual360::day_count(start, end),
            DayCounter::Actual365 => Actual365::day_count(start, end),
            DayCounter::Thirty360 => Thirty360::day_count(start, end),
            DayCounter::Thirty360US => Thirty360US::day_count(start, end),
            DayCounter::ActualActual => ActualActual::day_count(start, end),
            DayCounter::Business252 => Business252::day_count(start, end),
        }
    }

    pub fn year_fraction(&self, start: Date, end: Date) -> f64 {
        match self {
            DayCounter::Actual360 => Actual360::year_fraction(start, end),
            DayCounter::Actual365 => Actual365::year_fraction(start, end),
            DayCounter::Thirty360 => Thirty360::year_fraction(start, end),
            DayCounter::Thirty360US => Thirty360US::year_fraction(start, end),
            DayCounter::ActualActual => ActualActual::year_fraction(start, end),
            DayCounter::Business252 => Business252::year_fraction(start, end),
        }
    }
}

impl TryFrom<String> for DayCounter {
    type Error = AtlasError;

    fn try_from(s: String) -> Result<Self> {
        match s.as_str() {
            "Actual360" => Ok(DayCounter::Actual360),
            "Actual365" => Ok(DayCounter::Actual365),
            "Thirty360" => Ok(DayCounter::Thirty360), // to match curveengine
            "Thirty360US" => Ok(DayCounter::Thirty360US),
            "ActualActual" => Ok(DayCounter::ActualActual),
            "Business252" => Ok(DayCounter::Business252),
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
            DayCounter::Actual365 => "Actual365".to_string(),
            DayCounter::Thirty360 => "Thirty360".to_string(),
            DayCounter::Thirty360US => "Thirty360US".to_string(),
            DayCounter::ActualActual => "ActualActual".to_string(),
            DayCounter::Business252 => "Business252".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_count_standard() {
        let start = Date::new(2020, 1, 1);
        let end = Date::new(2020, 1, 2);

        let day_count = DayCounter::Actual360.day_count(start, end);
        assert_eq!(day_count, 1);
        let day_count = DayCounter::Actual365.day_count(start, end);
        assert_eq!(day_count, 1);
        let day_count = DayCounter::Thirty360.day_count(start, end);
        assert_eq!(day_count, 1);
        let day_count = DayCounter::Thirty360US.day_count(start, end);
        assert_eq!(day_count, 1);
        let day_count = DayCounter::ActualActual.day_count(start, end);
        assert_eq!(day_count, 1);
    }


    #[test]
    fn test_day_count_standard_inverted() {
        let start = Date::new(2020, 1, 1);
        let end = Date::new(2020, 1, 2);

        let day_count = DayCounter::Actual360.day_count(end, start);
        assert_eq!(day_count, -1);
        let day_count = DayCounter::Actual365.day_count(end, start);
        assert_eq!(day_count, -1);
        let day_count = DayCounter::Thirty360.day_count(end, start);
        assert_eq!(day_count, -1);
        let day_count = DayCounter::Thirty360US.day_count(end, start);
        assert_eq!(day_count, -1);
        let day_count = DayCounter::ActualActual.day_count(end, start);
        assert_eq!(day_count, -1);
    }
    

    #[test]
    fn test_year_fraction() {
        let start = Date::new(2020, 1, 1);
        let end = Date::new(2020, 1, 2);

        let year_fraction = DayCounter::Actual360.year_fraction(start, end);
        assert_eq!(year_fraction, 1.0 / 360.0);
        let year_fraction = DayCounter::Actual365.year_fraction(start, end);
        assert_eq!(year_fraction, 1.0 / 365.0);
        let year_fraction = DayCounter::Thirty360.year_fraction(start, end);
        assert_eq!(year_fraction, 1.0 / 360.0);
        let year_fraction = DayCounter::Thirty360US.year_fraction(start, end);
        assert_eq!(year_fraction, 1.0 / 360.0);
        let year_fraction = DayCounter::ActualActual.year_fraction(start, end);
        assert_eq!(year_fraction, 1.0 / 366.0);
    }


    #[test]
    fn test_year_fraction_inverse() {
        let start = Date::new(2020, 1, 1);
        let end = Date::new(2020, 1, 2);

        let year_fraction = DayCounter::Actual360.year_fraction(end, start);
        assert_eq!(year_fraction, -1.0 / 360.0);
        let year_fraction = DayCounter::Actual365.year_fraction(end, start);
        assert_eq!(year_fraction, -1.0 / 365.0);
        let year_fraction = DayCounter::Thirty360.year_fraction(end, start);
        assert_eq!(year_fraction, -1.0 / 360.0);
        let year_fraction = DayCounter::Thirty360US.year_fraction(end, start);
        assert_eq!(year_fraction, -1.0 / 360.0);
        let year_fraction = DayCounter::ActualActual.year_fraction(end, start);
        assert_eq!(year_fraction, -1.0 / 366.0);
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
    fn test_year_fraction_trithy360_beetween_end_and_start_of_month() {
        let start = Date::new(2023, 12, 10);
        let end_1 = Date::new(2023, 12, 31);
        let end_2 = Date::new(2024, 1, 1);

        let yf_1 = DayCounter::Thirty360.year_fraction(start, end_1);
        let yf_2 = DayCounter::Thirty360.year_fraction(start, end_2);
        assert_eq!(yf_1, yf_2);
    }


    #[test]
    fn test_year_fraction_trithy360_star_in_end_of_month() {
        let start = Date::new(2023, 12, 30);
        let end_1 = Date::new(2023, 12, 31);
        let end_2 = Date::new(2024, 1, 1);

        let yf_1 = DayCounter::Thirty360.year_fraction(start, end_1);
        let yf_2 = DayCounter::Thirty360.year_fraction(start, end_2);
        assert_ne!(yf_1, yf_2);
    }


    #[test]
    fn test_year_fraction_trithy360_leap_february() {
        let start = Date::new(2024, 2, 10);
        let end_1 = Date::new(2024, 2, 29);
        let end_2 = Date::new(2024, 3, 1);

        let yf_1 = DayCounter::Thirty360.year_fraction(start, end_1);
        let yf_2 = DayCounter::Thirty360.year_fraction(start, end_2);
      
        assert_ne!(yf_1, yf_2);
    }


    #[test]
    fn test_year_fraction_trithy360_february() {
        let start = Date::new(2023, 2, 10);
        let end_1 = Date::new(2023, 2, 28);
        let end_2 = Date::new(2023, 3, 1);

        let yf_1 = DayCounter::Thirty360.year_fraction(start, end_1);
        let yf_2 = DayCounter::Thirty360.year_fraction(start, end_2);
       
        assert_ne!(yf_1, yf_2);
    }
}


