use super::enums::Compounding;
use crate::{
    time::{date::Date, enums::Frequency},
    utils::errors::Result,
};

/// # `HasReferenceDate`
/// Implement this trait for a struct that has a reference date.
pub trait HasReferenceDate {
    /// Returns the reference date for this object.
    fn reference_date(&self) -> Date;
}

/// # `YieldProvider`
/// Implement this trait for a struct that provides yield information.
pub trait YieldProvider: HasReferenceDate {
    /// Calculates the discount factor for the given date.
    ///
    /// # Errors
    /// Returns an error if the discount factor cannot be computed for the date.
    fn discount_factor(&self, date: Date) -> Result<f64>;
    /// Calculates the forward rate between two dates with the specified compounding and frequency.
    ///
    /// # Errors
    /// Returns an error if the forward rate cannot be computed for the date range.
    fn forward_rate(
        &self,
        start_date: Date,
        end_date: Date,
        comp: Compounding,
        freq: Frequency,
    ) -> Result<f64>;
}
