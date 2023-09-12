use super::enums::Compounding;
use crate::time::{date::Date, enums::Frequency};

/// # HasReferenceDate
/// Implement this trait for a struct that has a reference date.
pub trait HasReferenceDate {
    fn reference_date(&self) -> Date;
}

/// # YieldProvider
/// Implement this trait for a struct that provides yield information.
pub trait YieldProvider: HasReferenceDate {
    fn discount_factor(&self, date: Date) -> f64;
    fn forward_rate(
        &self,
        start_date: Date,
        end_date: Date,
        comp: Compounding,
        freq: Frequency,
    ) -> f64;
}

// pub trait YieldTermStructure<T> {
pub trait Spread<T> {
    fn return_spread_to_date(&self,date: Date) -> f64; 
}