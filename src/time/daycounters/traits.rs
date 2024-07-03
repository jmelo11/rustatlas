use crate::{core::meta::Numeric, time::date::Date};

/// # DayCountProvider
/// Day count convention trait.
pub trait DayCountProvider {
    fn day_count(start: Date, end: Date) -> Numeric;
    fn year_fraction(start: Date, end: Date) -> Numeric;
}
