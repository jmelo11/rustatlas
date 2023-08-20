use crate::{
    rates::interestrate::RateDefinition,
    time::{date::Date, period::Period},
};

/// # FloatingRateProvider
/// Implement this trait for a struct that provides floating rate information.
trait FloatingRateProvider {
    fn fixing(&self, date: Date) -> f64;
    fn add_fixing(&mut self, date: Date, rate: f64);
    fn rate_definition(&self) -> RateDefinition;
    fn fixing_tenor(&self) -> Period;
}
