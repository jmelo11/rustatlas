use thiserror::Error;

use crate::{
    rates::traits::{HasReferenceDate, YieldProvider, YieldProviderError},
    time::{date::Date, period::Period},
};

/// # YieldTermStructureTraitClone
/// Trait for cloning a given object.
pub trait YieldTermStructureTraitClone {
    fn clone_box(&self) -> Box<dyn YieldTermStructureTrait>;
}

/// # YieldTermStructureTraitClone for T
/// Implementation of YieldTermStructureTraitClone for T.
impl<T: 'static + YieldTermStructureTrait + Clone> YieldTermStructureTraitClone for T {
    fn clone_box(&self) -> Box<dyn YieldTermStructureTrait> {
        Box::new(self.clone())
    }
}

/// # Clone for Box<dyn YieldTermStructureTrait>
impl Clone for Box<dyn YieldTermStructureTrait> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

/// # AdvanceTermStructureInTime
/// Trait for advancing in time a given object. Returns a represation of the object
/// as it would be after the given period.
pub trait AdvanceTermStructureInTime {
    fn advance_to_period(
        &self,
        period: Period,
    ) -> Result<Box<dyn YieldTermStructureTrait>, AdvanceInTimeError>;
    fn advance_to_date(
        &self,
        date: Date,
    ) -> Result<Box<dyn YieldTermStructureTrait>, AdvanceInTimeError>;
}

#[derive(Error, Debug)]
pub enum AdvanceInTimeError {
    #[error("Invalid date")]
    InvalidDate,
    #[error("YieldProviderError: {0}")]
    YieldProviderError(#[from] YieldProviderError),
    #[error("TermStructureConstructorError: {0}")]
    TermStructureConstructorError(#[from] TermStructureConstructorError),
}

#[derive(Error, Debug)]
pub enum TermStructureConstructorError {
    #[error("Dates and discount factors should have the same size.")]
    DatesAndDiscountFactorsSize,
    #[error("Dates and rates should have the same size.")]
    DatesAndRatesSize,
    #[error("First date needs to be reference_date")]
    FirstDateNeedsToBeReferenceDate,
    #[error("First discount factor needs to be 1.0")]
    FirstDiscountFactorsNeedsToBeOne,
}

/// # YieldTermStructureTrait
/// Trait that defines a yield term structure.
pub trait YieldTermStructureTrait:
    YieldProvider + HasReferenceDate + YieldTermStructureTraitClone + AdvanceTermStructureInTime
{
}
