use thiserror::Error;

use super::{enums::Compounding, interestrate::InterestRateError};
use crate::time::{date::Date, enums::Frequency};

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
    #[error("Date must be greater than reference date")]
    DateMustBeGreaterThanReferenceDate,
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
