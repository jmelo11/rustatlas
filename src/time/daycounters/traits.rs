use crate::{core::meta::Number, time::date::Date};

/// # DayCountProvider
/// Day count convention trait.
pub trait DayCountProvider {
    fn day_count(start: Date, end: Date) -> Number;
    fn year_fraction(start: Date, end: Date) -> Number;
}
