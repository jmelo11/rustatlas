use super::actual360::Actual360;
use super::actual365::Actual365;
use super::thirty360::Thirty360;
use super::traits::DayCountProvider;
use crate::time::date::Date;

/// # DayCounter
/// Day count convention enum.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DayCounter {
    Actual360,
    Actual365,
    Thirty360,
}

impl DayCountProvider for DayCounter {
    fn day_count(&self, start: Date, end: Date) -> i64 {
        match self {
            DayCounter::Actual360 => Actual360.day_count(start, end),
            DayCounter::Actual365 => Actual365.day_count(start, end),
            DayCounter::Thirty360 => Thirty360.day_count(start, end),
        }
    }

    fn year_fraction(&self, start: Date, end: Date) -> f64 {
        match self {
            DayCounter::Actual360 => Actual360.year_fraction(start, end),
            DayCounter::Actual365 => Actual365.year_fraction(start, end),
            DayCounter::Thirty360 => Thirty360.year_fraction(start, end),
        }
    }
}
