use super::enums::Compounding;
use crate::{
    core::meta::Numeric,
    time::{date::Date, enums::Frequency},
    utils::errors::Result,
};

/// # HasReferenceDate
/// Implement this trait for a struct that has a reference date.
pub trait HasReferenceDate {
    fn reference_date(&self) -> Date;
}

/// # YieldProvider
/// Implement this trait for a struct that provides yield information.
pub trait YieldProvider: HasReferenceDate {
    fn discount_factor(&self, date: Date) -> Result<Numeric>;
    fn forward_rate(
        &self,
        start_date: Date,
        end_date: Date,
        comp: Compounding,
        freq: Frequency,
    ) -> Result<Numeric>;
}
