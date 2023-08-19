use crate::time::date::Date;

pub trait DoesDayCount {
    fn day_count(&self, start: Date, end: Date) -> i64;
    fn year_fraction(&self, start: Date, end: Date) -> f64;
}
