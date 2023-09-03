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
