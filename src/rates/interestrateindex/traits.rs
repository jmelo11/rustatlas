use std::collections::HashMap;

use crate::time::date::Date;

/// # FloatingRateProvider
/// Implement this trait for a struct that holds floating rate information.
pub trait FixingProvider {
    fn fixing(&self, date: Date) -> Option<f64>;
    fn fixings(&self) -> &HashMap<Date, f64>;
    fn add_fixing(&mut self, date: Date, rate: f64);
}
