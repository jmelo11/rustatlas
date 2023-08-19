use super::traits::DoesDayCount;
use crate::time::date::Date;

pub struct Actual365;

impl DoesDayCount for Actual365 {
    fn day_count(&self, start: Date, end: Date) -> i64 {
        return end - start;
    }

    fn year_fraction(&self, start: Date, end: Date) -> f64 {
        return self.day_count(start, end) as f64 / 365.0;
    }
}
