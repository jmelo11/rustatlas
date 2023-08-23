use crate::{
    rates::interestrate::RateDefinition,
    time::{date::Date, period::Period},
};

/// # FloatingRateProvider
/// Implement this trait for a struct that holds floating rate information.
pub trait FloatingRateProvider {
    fn fixing(&self, date: Date) -> Option<f64>;
    fn add_fixing(&mut self, date: Date, rate: f64);
    fn rate_definition(&self) -> RateDefinition;
    fn fixing_tenor(&self) -> Period;
}
