use crate::{
    rates::interestrate::RateDefinition,
    time::{date::Date, period::Period},
};

/// # FixingRateHolder
/// Implement this trait for a struct that holds floating rate information.
pub trait FixingRateHolder {
    fn fixing(&self, date: Date) -> Option<f64>;
    fn add_fixing(&mut self, date: Date, rate: f64);
    fn rate_definition(&self) -> RateDefinition;
    fn fixing_tenor(&self) -> Period;
}
