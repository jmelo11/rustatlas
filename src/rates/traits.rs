use thiserror::Error;

use super::{enums::Compounding, interestrate::InterestRateError};
use crate::time::{date::Date, enums::Frequency, period::Period};

/// # HasReferenceDate
/// Implement this trait for a struct that has a reference date.
pub trait HasReferenceDate {
    fn reference_date(&self) -> Date;
}

#[derive(Error, Debug)]
pub enum YieldProviderError {
    #[error("Invalid date: {0}")]
    InvalidDate(String),
    #[error("Invalid term structure")]
    NoTermStructure,
    #[error("Invalid interest rate")]
    InterestRateError(#[from] InterestRateError),
    #[error("No fixing rate for date {0}")]
    NoFixingRate(Date),
}
/// # YieldProvider
/// Implement this trait for a struct that provides yield information.
pub trait YieldProvider: HasReferenceDate {
    fn discount_factor(&self, date: Date) -> Result<f64, YieldProviderError>;
    fn forward_rate(
        &self,
        start_date: Date,
        end_date: Date,
        comp: Compounding,
        freq: Frequency,
    ) -> Result<f64, YieldProviderError>;
}

/// # AdvanceInTime
/// Trait for advancing in time a given object. Returns a represation of the object
/// as it would be after the given period.
pub trait AdvanceInTime<E> {
    type Output;
    fn advance(&self, period: Period) -> Result<Self::Output, E>;
    // fn advance_period(&self, period: Period) -> Result<Self::Output, E>;
    // fn advance_date(&self, date: Date) -> Result<Self::Output, E>;
}

// pub trait YieldTermStructure<T> {
pub trait Spread<T> {
    fn return_spread_to_date(&self,year_fraction: f64) -> f64; 
}