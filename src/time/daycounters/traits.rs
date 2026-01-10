use crate::time::date::Date;

/// # DayCountProvider
/// Day count convention trait.
pub trait DayCountProvider {
    /// Calculates the number of days between two dates using the day count convention.
    ///
    /// # Arguments
    /// * `start` - The start date
    /// * `end` - The end date
    fn day_count(start: Date, end: Date) -> i64;
    /// Calculates the fraction of a year between two dates using the day count convention.
    ///
    /// # Arguments
    /// * `start` - The start date
    /// * `end` - The end date
    fn year_fraction(start: Date, end: Date) -> f64;
}
