use serde::{Deserialize, Serialize};

use super::daycounters::{
    actual360::Actual360, actual365::Actual365, thirty360::Thirty360, traits::DayCountProvider,
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

impl TryFrom<String> for DayCounter {
    type Error = AtlasError;

    fn try_from(s: String) -> Result<Self> {
        match s.as_str() {
            "Actual360" => Ok(DayCounter::Actual360),
            "Actual365" => Ok(DayCounter::Actual365),
            "Thirty360" => Ok(DayCounter::Thirty360), // to match curveengine
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
        assert_eq!(year_fraction, 1.0 / 360.0);
        let year_fraction = DayCounter::Actual365.year_fraction(start, end);
        assert_eq!(year_fraction, 1.0 / 365.0);
        let year_fraction = DayCounter::Thirty360.year_fraction(start, end);
        assert_eq!(year_fraction, 1.0 / 360.0);
    }
}
