use crate::time::date::Date;


/// # DayCountProvider
/// Day count convention trait.
pub trait DayCountProvider {
    fn day_count(&self, start: Date, end: Date) -> i64;
    fn year_fraction(&self, start: Date, end: Date) -> f64;
}
